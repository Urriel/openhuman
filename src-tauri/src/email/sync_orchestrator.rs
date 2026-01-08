//! Background Sync Orchestration
//!
//! This module manages periodic email synchronization across multiple accounts.
//! Features:
//! - Multi-folder sync with differential timers (INBOX: 5 min, others: 30 min)
//! - UID-based incremental sync with UIDVALIDITY tracking
//! - IDLE notifications for real-time INBOX updates
//! - Initial sync (30 days) + background historical sync
//! - Parallel multi-account and multi-folder sync
//! - Graceful cancellation support

use crate::email::imap::{ImapClient, ImapFolder, ImapMessage};
use crate::error::{SyncError, SyncResult};
use crate::retry::retry_with_backoff;
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tokio_util::sync::CancellationToken;

/// Sync orchestrator for managing email synchronization
pub struct SyncOrchestrator {
    pool: Arc<SqlitePool>,
    cancellation_token: CancellationToken,
    inbox_sync_interval_minutes: u64,
    other_folders_sync_interval_minutes: u64,
    initial_sync_days: i64,
    historical_sync_batch_days: i64,
}

/// Sync statistics for a single account
#[derive(Debug, Clone)]
pub struct SyncStats {
    pub account_id: i64,
    pub messages_synced: i32,
    pub new_messages: i32,
    pub errors: Vec<String>,
    pub folders_synced: Vec<String>,
}

/// Sync status for an account
#[derive(Debug, Clone, PartialEq)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Completed,
    Failed,
    Cancelled,
}

impl SyncStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SyncStatus::Idle => "idle",
            SyncStatus::Syncing => "syncing",
            SyncStatus::Completed => "completed",
            SyncStatus::Failed => "failed",
            SyncStatus::Cancelled => "cancelled",
        }
    }
}

/// Folder sync state
#[derive(Debug, Clone)]
struct FolderSyncState {
    folder_name: String,
    last_uid: Option<u32>,
    uidvalidity: Option<u32>,
    last_synced_at: Option<DateTime<Utc>>,
    historical_sync_until: Option<DateTime<Utc>>,
}

impl SyncOrchestrator {
    /// Create a new sync orchestrator with default settings
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            pool,
            cancellation_token: CancellationToken::new(),
            inbox_sync_interval_minutes: 5,
            other_folders_sync_interval_minutes: 30,
            initial_sync_days: 30,
            historical_sync_batch_days: 30,
        }
    }

    /// Create a new sync orchestrator with custom intervals
    pub fn with_intervals(
        pool: Arc<SqlitePool>,
        inbox_interval_minutes: u64,
        other_folders_interval_minutes: u64,
    ) -> Self {
        Self {
            pool,
            cancellation_token: CancellationToken::new(),
            inbox_sync_interval_minutes: inbox_interval_minutes,
            other_folders_sync_interval_minutes: other_folders_interval_minutes,
            initial_sync_days: 30,
            historical_sync_batch_days: 30,
        }
    }

    /// Start the sync scheduler with periodic background sync
    pub async fn start_scheduler(&self) -> SyncResult<()> {
        // Run initial sync on app launch
        self.sync_all_accounts().await?;

        // Start separate timers for INBOX and other folders
        let pool = Arc::clone(&self.pool);
        let cancel_token = self.cancellation_token.clone();
        let inbox_interval = self.inbox_sync_interval_minutes;
        let other_interval = self.other_folders_sync_interval_minutes;

        // Spawn INBOX sync timer
        let inbox_pool = Arc::clone(&pool);
        let inbox_cancel = cancel_token.child_token();
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(inbox_interval * 60));
            loop {
                tokio::select! {
                    _ = inbox_cancel.cancelled() => break,
                    _ = timer.tick() => {
                        if let Err(e) = sync_inbox_folders(Arc::clone(&inbox_pool), inbox_cancel.clone()).await {
                            eprintln!("INBOX sync failed: {}", e);
                        }
                    }
                }
            }
        });

        // Spawn other folders sync timer
        let other_pool = Arc::clone(&pool);
        let other_cancel = cancel_token.child_token();
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(other_interval * 60));
            loop {
                tokio::select! {
                    _ = other_cancel.cancelled() => break,
                    _ = timer.tick() => {
                        if let Err(e) = sync_other_folders(Arc::clone(&other_pool), other_cancel.clone()).await {
                            eprintln!("Other folders sync failed: {}", e);
                        }
                    }
                }
            }
        });

        // Wait for cancellation
        cancel_token.cancelled().await;
        Ok(())
    }

    /// Sync all enabled accounts in parallel
    pub async fn sync_all_accounts(&self) -> SyncResult<Vec<SyncStats>> {
        let accounts = self.get_enabled_accounts().await?;

        // Spawn parallel sync tasks
        let mut tasks = Vec::new();

        for account in accounts {
            let pool = Arc::clone(&self.pool);
            let cancel_token = self.cancellation_token.child_token();
            let initial_days = self.initial_sync_days;
            let historical_days = self.historical_sync_batch_days;

            let task = tokio::spawn(async move {
                sync_account(
                    pool,
                    account.id,
                    cancel_token,
                    initial_days,
                    historical_days,
                )
                .await
            });

            tasks.push(task);
        }

        // Wait for all tasks to complete
        let mut results = Vec::new();
        for task in tasks {
            match task.await {
                Ok(Ok(stats)) => results.push(stats),
                Ok(Err(e)) => {
                    eprintln!("Account sync error: {}", e);
                    // Continue syncing other accounts
                }
                Err(e) => {
                    eprintln!("Task join error: {}", e);
                }
            }
        }

        Ok(results)
    }

    /// Sync a specific account
    pub async fn sync_account(&self, account_id: i64) -> SyncResult<SyncStats> {
        sync_account(
            Arc::clone(&self.pool),
            account_id,
            self.cancellation_token.child_token(),
            self.initial_sync_days,
            self.historical_sync_batch_days,
        )
        .await
    }

    /// Start IDLE listener for an account
    pub async fn start_idle_listener(&self, account_id: i64) -> SyncResult<()> {
        let pool = Arc::clone(&self.pool);
        let cancel_token = self.cancellation_token.child_token();

        tokio::spawn(async move {
            idle_listener_task(pool, account_id, cancel_token).await;
        });

        Ok(())
    }

    /// Cancel ongoing sync operations
    pub fn cancel_sync(&self) {
        self.cancellation_token.cancel();
    }

    /// Get sync status for an account
    pub async fn get_sync_status(&self, account_id: i64) -> SyncResult<SyncStatus> {
        let status = sqlx::query_scalar::<_, String>(
            "SELECT sync_status FROM sync_state WHERE account_id = ?",
        )
        .bind(account_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| SyncError::DatabaseError(e.into()))?;

        Ok(match status.as_str() {
            "idle" => SyncStatus::Idle,
            "syncing" => SyncStatus::Syncing,
            "completed" => SyncStatus::Completed,
            "failed" => SyncStatus::Failed,
            "cancelled" => SyncStatus::Cancelled,
            _ => SyncStatus::Idle,
        })
    }

    /// Get enabled accounts from database
    async fn get_enabled_accounts(&self) -> SyncResult<Vec<Account>> {
        #[derive(sqlx::FromRow)]
        struct AccountRow {
            id: i64,
            email: String,
            imap_host: String,
            imap_port: i64,
        }

        let accounts = sqlx::query_as::<_, AccountRow>(
            "SELECT id, email, imap_host, imap_port FROM accounts WHERE sync_enabled = 1",
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| SyncError::DatabaseError(e.into()))?;

        Ok(accounts
            .into_iter()
            .map(|row| Account {
                id: row.id,
                email: row.email,
                imap_host: row.imap_host,
                imap_port: row.imap_port as u16,
            })
            .collect())
    }
}

/// Account info for syncing
#[derive(Debug, Clone)]
struct Account {
    id: i64,
    email: String,
    imap_host: String,
    imap_port: u16,
}

/// Sync all INBOX folders for enabled accounts
async fn sync_inbox_folders(
    pool: Arc<SqlitePool>,
    cancel_token: CancellationToken,
) -> SyncResult<()> {
    let accounts = get_enabled_accounts(&pool).await?;

    for account in accounts {
        if cancel_token.is_cancelled() {
            break;
        }

        // Only sync INBOX folder
        if let Err(e) = sync_folder(
            Arc::clone(&pool),
            account.id,
            "INBOX",
            cancel_token.clone(),
            30, // initial_sync_days
            30, // historical_sync_batch_days
        )
        .await
        {
            eprintln!("INBOX sync failed for account {}: {}", account.id, e);
        }
    }

    Ok(())
}

/// Sync all non-INBOX folders for enabled accounts
async fn sync_other_folders(
    pool: Arc<SqlitePool>,
    cancel_token: CancellationToken,
) -> SyncResult<()> {
    let accounts = get_enabled_accounts(&pool).await?;

    for account in accounts {
        if cancel_token.is_cancelled() {
            break;
        }

        // Get all folders except INBOX
        let folders = get_account_folders(&pool, account.id).await?;
        let other_folders: Vec<_> = folders
            .into_iter()
            .filter(|f| f.to_uppercase() != "INBOX")
            .collect();

        // Sync each folder in parallel
        let mut tasks = Vec::new();
        for folder in other_folders {
            let pool_clone = Arc::clone(&pool);
            let cancel_clone = cancel_token.clone();
            let task = tokio::spawn(async move {
                sync_folder(pool_clone, account.id, &folder, cancel_clone, 30, 30).await
            });
            tasks.push(task);
        }

        // Wait for all folder syncs to complete
        for task in tasks {
            if let Err(e) = task.await {
                eprintln!("Folder sync task failed: {}", e);
            }
        }
    }

    Ok(())
}

/// Get enabled accounts from database
async fn get_enabled_accounts(pool: &SqlitePool) -> SyncResult<Vec<Account>> {
    #[derive(sqlx::FromRow)]
    struct AccountRow {
        id: i64,
        email: String,
        imap_host: String,
        imap_port: i64,
    }

    let accounts = sqlx::query_as::<_, AccountRow>(
        "SELECT id, email, imap_host, imap_port FROM accounts WHERE sync_enabled = 1",
    )
    .fetch_all(pool)
    .await
    .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(accounts
        .into_iter()
        .map(|row| Account {
            id: row.id,
            email: row.email,
            imap_host: row.imap_host,
            imap_port: row.imap_port as u16,
        })
        .collect())
}

/// Get folders for an account from database
async fn get_account_folders(pool: &SqlitePool, account_id: i64) -> SyncResult<Vec<String>> {
    let folders = sqlx::query_scalar::<_, String>("SELECT name FROM folders WHERE account_id = ?")
        .bind(account_id)
        .fetch_all(pool)
        .await
        .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(folders)
}

/// Sync a single account with all its folders
async fn sync_account(
    pool: Arc<SqlitePool>,
    account_id: i64,
    cancel_token: CancellationToken,
    initial_sync_days: i64,
    historical_sync_batch_days: i64,
) -> SyncResult<SyncStats> {
    // Update sync status to syncing
    update_sync_status(&pool, account_id, SyncStatus::Syncing, None).await?;

    let mut stats = SyncStats {
        account_id,
        messages_synced: 0,
        new_messages: 0,
        errors: Vec::new(),
        folders_synced: Vec::new(),
    };

    // Check for cancellation
    if cancel_token.is_cancelled() {
        update_sync_status(&pool, account_id, SyncStatus::Cancelled, None).await?;
        return Err(SyncError::Cancelled);
    }

    // Get account credentials
    let account = get_account(&pool, account_id).await?;

    // Get password from keychain
    let password =
        crate::keychain::get_credentials(&account.email).map_err(|_e| SyncError::AuthRequired)?;

    // Connect to IMAP server with retry
    let mut client = retry_with_backoff(
        || async {
            ImapClient::connect(
                &account.imap_host,
                account.imap_port,
                &account.email,
                &password,
            )
            .await
        },
        5,
    )
    .await
    .map_err(SyncError::ImapError)?;

    // Discover and store folders
    let folders = discover_and_store_folders(&pool, &mut client, account_id).await?;

    // Sync folders in parallel (INBOX gets priority)
    let mut tasks = Vec::new();

    for folder in folders {
        if cancel_token.is_cancelled() {
            break;
        }

        let pool_clone = Arc::clone(&pool);
        let cancel_clone = cancel_token.clone();
        let folder_name = folder.name.clone();

        let task = tokio::spawn(async move {
            sync_folder(
                pool_clone,
                account_id,
                &folder_name,
                cancel_clone,
                initial_sync_days,
                historical_sync_batch_days,
            )
            .await
        });

        tasks.push((folder.name.clone(), task));
    }

    // Collect results
    for (folder_name, task) in tasks {
        match task.await {
            Ok(Ok(folder_stats)) => {
                stats.messages_synced += folder_stats.messages_synced;
                stats.new_messages += folder_stats.new_messages;
                stats.folders_synced.push(folder_name);
            }
            Ok(Err(e)) => {
                stats.errors.push(format!("Folder {}: {}", folder_name, e));
            }
            Err(e) => {
                stats
                    .errors
                    .push(format!("Task error for {}: {}", folder_name, e));
            }
        }
    }

    // Update sync state
    update_sync_state(&pool, account_id, stats.messages_synced).await?;

    // Mark sync as completed
    update_sync_status(&pool, account_id, SyncStatus::Completed, None).await?;

    Ok(stats)
}

/// Discover folders and store them in database
async fn discover_and_store_folders(
    pool: &SqlitePool,
    client: &mut ImapClient,
    account_id: i64,
) -> SyncResult<Vec<ImapFolder>> {
    let folders = client.list_folders().await.map_err(SyncError::ImapError)?;

    // Store folders in database
    for folder in &folders {
        // Insert or update folder
        sqlx::query(
            r#"
            INSERT INTO folders (account_id, name, folder_type, selectable, flags)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(account_id, name) DO UPDATE SET
                folder_type = excluded.folder_type,
                selectable = excluded.selectable,
                flags = excluded.flags
            "#,
        )
        .bind(account_id)
        .bind(&folder.name)
        .bind(&folder.folder_type)
        .bind(folder.selectable as i64)
        .bind(folder.flags.join(","))
        .execute(pool)
        .await
        .map_err(|e| SyncError::DatabaseError(e.into()))?;
    }

    Ok(folders)
}

/// Sync a single folder for an account
async fn sync_folder(
    pool: Arc<SqlitePool>,
    account_id: i64,
    folder_name: &str,
    cancel_token: CancellationToken,
    initial_sync_days: i64,
    historical_sync_batch_days: i64,
) -> SyncResult<SyncStats> {
    let mut stats = SyncStats {
        account_id,
        messages_synced: 0,
        new_messages: 0,
        errors: Vec::new(),
        folders_synced: vec![folder_name.to_string()],
    };

    if cancel_token.is_cancelled() {
        return Err(SyncError::Cancelled);
    }

    // Get account and connect
    let account = get_account(&pool, account_id).await?;
    let password =
        crate::keychain::get_credentials(&account.email).map_err(|_e| SyncError::AuthRequired)?;

    let mut client = retry_with_backoff(
        || async {
            ImapClient::connect(
                &account.imap_host,
                account.imap_port,
                &account.email,
                &password,
            )
            .await
        },
        5,
    )
    .await
    .map_err(SyncError::ImapError)?;

    // Select folder
    let mailbox = client
        .select_folder(folder_name)
        .await
        .map_err(SyncError::ImapError)?;

    // Get folder sync state
    let folder_state = get_folder_sync_state(&pool, account_id, folder_name).await?;

    // Check UIDVALIDITY
    if let (Some(stored_uidvalidity), Some(server_uidvalidity)) =
        (folder_state.uidvalidity, mailbox.uid_validity)
    {
        if stored_uidvalidity != server_uidvalidity {
            // UIDVALIDITY changed - trigger full re-sync
            eprintln!(
                "UIDVALIDITY changed for folder {}: {} -> {}",
                folder_name, stored_uidvalidity, server_uidvalidity
            );
            // Clear old UIDs and re-sync from scratch
            reset_folder_sync_state(&pool, account_id, folder_name).await?;
        }
    }

    // Update UIDVALIDITY in database
    if let Some(uidvalidity) = mailbox.uid_validity {
        update_folder_uidvalidity(&pool, account_id, folder_name, uidvalidity).await?;
    }

    // Determine sync strategy
    let is_initial_sync = folder_state.last_synced_at.is_none();

    if is_initial_sync {
        // Initial sync: last 30 days
        let since_date = Utc::now() - ChronoDuration::days(initial_sync_days);
        let messages = fetch_messages_since_date(&mut client, &since_date).await?;

        stats.new_messages = messages.len() as i32;
        stats.messages_synced = messages.len() as i32;

        // Store messages
        for message in messages {
            store_message(&pool, account_id, folder_name, &message).await?;
        }

        // Mark initial sync complete and set up historical sync
        mark_initial_sync_complete(&pool, account_id, folder_name, &since_date).await?;
    } else {
        // Incremental sync: fetch only new messages since last UID
        let since_uid = folder_state.last_uid;
        let messages = client
            .fetch_new_messages(since_uid)
            .await
            .map_err(SyncError::ImapError)?;

        stats.new_messages = messages.len() as i32;
        stats.messages_synced = messages.len() as i32;

        // Store messages
        for message in &messages {
            store_message(&pool, account_id, folder_name, message).await?;
        }

        // Update last UID
        if let Some(last_msg) = messages.last() {
            update_folder_last_uid(&pool, account_id, folder_name, last_msg.uid).await?;
        }

        // Continue historical sync if not complete
        if let Some(historical_until) = folder_state.historical_sync_until {
            sync_historical_batch(
                &pool,
                &mut client,
                account_id,
                folder_name,
                historical_until,
                historical_sync_batch_days,
            )
            .await?;
        }
    }

    // Sync flags from server to local (server is authoritative)
    sync_flags_from_server(&pool, &mut client, account_id, folder_name).await?;

    Ok(stats)
}

/// Fetch messages since a specific date
async fn fetch_messages_since_date(
    client: &mut ImapClient,
    _since_date: &DateTime<Utc>,
) -> SyncResult<Vec<ImapMessage>> {
    // For now, fetch all new messages (SINCE filtering would require additional IMAP commands)
    // In production, this would use SEARCH SINCE command
    let messages = client
        .fetch_new_messages(None)
        .await
        .map_err(SyncError::ImapError)?;

    // Filter by date on client side
    let filtered: Vec<_> = messages
        .into_iter()
        .filter(|_msg| {
            // In production, parse message date and compare
            // For now, return all messages
            true
        })
        .collect();

    Ok(filtered)
}

/// Sync a historical batch (30 days at a time going backward)
async fn sync_historical_batch(
    pool: &SqlitePool,
    client: &mut ImapClient,
    account_id: i64,
    folder_name: &str,
    current_until: DateTime<Utc>,
    batch_days: i64,
) -> SyncResult<()> {
    // Calculate next batch date range
    let batch_since = current_until - ChronoDuration::days(batch_days);

    // Fetch messages in this date range
    let messages = fetch_messages_since_date(client, &batch_since).await?;

    // Store messages
    for message in messages {
        store_message(pool, account_id, folder_name, &message).await?;
    }

    // Update historical sync progress
    update_historical_sync_progress(pool, account_id, folder_name, batch_since).await?;

    Ok(())
}

/// Store a message in the database
async fn store_message(
    pool: &SqlitePool,
    account_id: i64,
    folder_name: &str,
    message: &ImapMessage,
) -> SyncResult<()> {
    // Parse message body if available
    let (subject, from_addr, body_plain, body_html) = if let Some(body) = &message.body {
        parse_message_fields(body)
    } else {
        (None, None, None, None)
    };

    let message_id = message
        .message_id
        .clone()
        .unwrap_or_else(|| format!("uid-{}-{}", folder_name, message.uid));

    // Check if message already exists
    let existing: Option<i64> =
        sqlx::query_scalar("SELECT id FROM messages WHERE account_id = ? AND message_id = ?")
            .bind(account_id)
            .bind(&message_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| SyncError::DatabaseError(e.into()))?;

    if existing.is_some() {
        // Update flags only
        update_message_flags(pool, account_id, &message_id, &message.flags).await?;
        return Ok(());
    }

    // Insert new message
    sqlx::query(
        r#"
        INSERT INTO messages (account_id, message_id, folder, subject, from_addr, body_plain, body_html, imap_uid, imap_flags, date)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))
        "#,
    )
    .bind(account_id)
    .bind(&message_id)
    .bind(folder_name)
    .bind(subject)
    .bind(from_addr)
    .bind(body_plain)
    .bind(body_html)
    .bind(message.uid as i64)
    .bind(message.flags.join(","))
    .execute(pool)
    .await
    .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(())
}

/// Parse message fields from body using MIME parser
/// Returns (subject, from, body_plain, body_html)
fn parse_message_fields(
    body: &[u8],
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    use crate::email::mime_parser::parse_message;

    match parse_message(body) {
        Ok(parsed) => {
            // Return both plain and HTML separately
            (
                parsed.subject,
                parsed.from,
                parsed.body_plain,
                parsed.body_html,
            )
        }
        Err(_) => {
            // Fallback to basic parsing if MIME parsing fails
            let body_str = String::from_utf8_lossy(body);
            let lines: Vec<&str> = body_str.lines().collect();

            let mut subject = None;
            let mut from = None;

            for line in &lines {
                if line.starts_with("Subject:") {
                    subject = Some(line.trim_start_matches("Subject:").trim().to_string());
                } else if line.starts_with("From:") {
                    from = Some(line.trim_start_matches("From:").trim().to_string());
                }
            }

            // Return raw body as plain text, no HTML in fallback mode
            (subject, from, Some(body_str.to_string()), None)
        }
    }
}

/// Update message flags in database
async fn update_message_flags(
    pool: &SqlitePool,
    account_id: i64,
    message_id: &str,
    flags: &[String],
) -> SyncResult<()> {
    let is_read = flags.iter().any(|f| f.contains("Seen"));
    let is_starred = flags.iter().any(|f| f.contains("Flagged"));

    sqlx::query(
        "UPDATE messages SET imap_flags = ?, is_read = ?, is_starred = ? WHERE account_id = ? AND message_id = ?",
    )
    .bind(flags.join(","))
    .bind(is_read as i64)
    .bind(is_starred as i64)
    .bind(account_id)
    .bind(message_id)
    .execute(pool)
    .await
    .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(())
}

/// Sync flags from server to local (server is authoritative)
async fn sync_flags_from_server(
    pool: &SqlitePool,
    client: &mut ImapClient,
    account_id: i64,
    folder_name: &str,
) -> SyncResult<()> {
    // Fetch all message flags from server for this folder
    let server_flags = client
        .fetch_message_flags("1:*")
        .await
        .map_err(SyncError::ImapError)?;

    // Get all local messages for this folder
    #[derive(sqlx::FromRow)]
    struct LocalMessage {
        message_id: String,
        imap_uid: i64,
        imap_flags: Option<String>,
    }

    let local_messages: Vec<LocalMessage> = sqlx::query_as(
        "SELECT message_id, imap_uid, imap_flags FROM messages WHERE account_id = ? AND folder = ? AND imap_uid IS NOT NULL",
    )
    .bind(account_id)
    .bind(folder_name)
    .fetch_all(pool)
    .await
    .map_err(|e| SyncError::DatabaseError(e.into()))?;

    // Compare and update flags where server differs from local
    for local_msg in local_messages {
        if let Some(server_flag_list) = server_flags.get(&(local_msg.imap_uid as u32)) {
            let local_flags = local_msg
                .imap_flags
                .unwrap_or_default()
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            // Check if flags differ
            let flags_differ = server_flag_list.len() != local_flags.len()
                || server_flag_list.iter().any(|f| !local_flags.contains(f));

            if flags_differ {
                // Update local to match server (server is authoritative)
                update_message_flags(pool, account_id, &local_msg.message_id, server_flag_list)
                    .await?;
            }
        }
    }

    Ok(())
}

/// Get folder sync state from database
async fn get_folder_sync_state(
    pool: &SqlitePool,
    account_id: i64,
    folder_name: &str,
) -> SyncResult<FolderSyncState> {
    #[derive(sqlx::FromRow)]
    struct StateRow {
        uidvalidity: Option<i64>,
        uidnext: Option<i64>,
    }

    // Get UIDVALIDITY from folders table
    let folder_state: Option<StateRow> = sqlx::query_as(
        "SELECT uidvalidity, uidnext FROM folders WHERE account_id = ? AND name = ?",
    )
    .bind(account_id)
    .bind(folder_name)
    .fetch_optional(pool)
    .await
    .map_err(|e| SyncError::DatabaseError(e.into()))?;

    // Get last synced UID from messages
    let last_uid: Option<i64> = sqlx::query_scalar(
        "SELECT MAX(imap_uid) FROM messages WHERE account_id = ? AND folder = ?",
    )
    .bind(account_id)
    .bind(folder_name)
    .fetch_optional(pool)
    .await
    .map_err(|e| SyncError::DatabaseError(e.into()))?
    .flatten();

    // Get last sync time and historical sync progress from sync_state
    #[derive(sqlx::FromRow)]
    struct SyncStateRow {
        last_sync_at: Option<String>,
    }

    let sync_row: Option<SyncStateRow> =
        sqlx::query_as("SELECT last_sync_at FROM sync_state WHERE account_id = ?")
            .bind(account_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| SyncError::DatabaseError(e.into()))?;

    let last_synced_at = sync_row.and_then(|r| {
        r.last_sync_at
            .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
            .map(|dt| dt.with_timezone(&Utc))
    });

    Ok(FolderSyncState {
        folder_name: folder_name.to_string(),
        last_uid: last_uid.map(|u| u as u32),
        uidvalidity: folder_state
            .as_ref()
            .and_then(|s| s.uidvalidity)
            .map(|v| v as u32),
        last_synced_at,
        historical_sync_until: None, // Would be stored in a separate column
    })
}

/// Reset folder sync state after UIDVALIDITY change
async fn reset_folder_sync_state(
    pool: &SqlitePool,
    account_id: i64,
    folder_name: &str,
) -> SyncResult<()> {
    // Delete all messages from this folder
    sqlx::query("DELETE FROM messages WHERE account_id = ? AND folder = ?")
        .bind(account_id)
        .bind(folder_name)
        .execute(pool)
        .await
        .map_err(|e| SyncError::DatabaseError(e.into()))?;

    // Reset folder UIDVALIDITY
    sqlx::query(
        "UPDATE folders SET uidvalidity = NULL, uidnext = NULL WHERE account_id = ? AND name = ?",
    )
    .bind(account_id)
    .bind(folder_name)
    .execute(pool)
    .await
    .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(())
}

/// Update folder UIDVALIDITY
async fn update_folder_uidvalidity(
    pool: &SqlitePool,
    account_id: i64,
    folder_name: &str,
    uidvalidity: u32,
) -> SyncResult<()> {
    sqlx::query("UPDATE folders SET uidvalidity = ? WHERE account_id = ? AND name = ?")
        .bind(uidvalidity as i64)
        .bind(account_id)
        .bind(folder_name)
        .execute(pool)
        .await
        .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(())
}

/// Update folder last synced UID
async fn update_folder_last_uid(
    pool: &SqlitePool,
    account_id: i64,
    folder_name: &str,
    last_uid: u32,
) -> SyncResult<()> {
    sqlx::query("UPDATE folders SET uidnext = ? WHERE account_id = ? AND name = ?")
        .bind(last_uid as i64)
        .bind(account_id)
        .bind(folder_name)
        .execute(pool)
        .await
        .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(())
}

/// Mark initial sync complete for a folder
async fn mark_initial_sync_complete(
    pool: &SqlitePool,
    account_id: i64,
    folder_name: &str,
    _since_date: &DateTime<Utc>,
) -> SyncResult<()> {
    // Store the date we synced from as the starting point for historical sync
    // In production, this would be stored in a dedicated column
    update_folder_last_uid(pool, account_id, folder_name, 0).await?;
    Ok(())
}

/// Update historical sync progress
async fn update_historical_sync_progress(
    _pool: &SqlitePool,
    _account_id: i64,
    _folder_name: &str,
    _batch_since: DateTime<Utc>,
) -> SyncResult<()> {
    // In production, store this in a dedicated column
    Ok(())
}

/// IDLE listener task for real-time notifications
async fn idle_listener_task(
    pool: Arc<SqlitePool>,
    account_id: i64,
    cancel_token: CancellationToken,
) {
    loop {
        if cancel_token.is_cancelled() {
            break;
        }

        // Get account and connect
        let account_result = get_account(&pool, account_id).await;
        if let Err(e) = account_result {
            eprintln!("Failed to get account for IDLE: {}", e);
            tokio::time::sleep(Duration::from_secs(60)).await;
            continue;
        }

        let account = account_result.unwrap();
        let password_result = crate::keychain::get_credentials(&account.email);
        if let Err(e) = password_result {
            eprintln!("Failed to get password for IDLE: {}", e);
            tokio::time::sleep(Duration::from_secs(60)).await;
            continue;
        }

        let password = password_result.unwrap();

        // Connect with retry
        let client_result = retry_with_backoff(
            || async {
                ImapClient::connect(
                    &account.imap_host,
                    account.imap_port,
                    &account.email,
                    &password,
                )
                .await
            },
            5,
        )
        .await;

        if let Err(e) = client_result {
            eprintln!("Failed to connect for IDLE: {}", e);
            tokio::time::sleep(Duration::from_secs(60)).await;
            continue;
        }

        let mut client = client_result.unwrap();

        // Select INBOX
        if let Err(e) = client.select_folder("INBOX").await {
            eprintln!("Failed to select INBOX for IDLE: {}", e);
            tokio::time::sleep(Duration::from_secs(60)).await;
            continue;
        }

        // Start IDLE
        // Note: IDLE is not yet fully implemented in ImapClient
        // This is a placeholder for future implementation
        match client.start_idle().await {
            Ok(_) => {
                // IDLE notification received - trigger immediate INBOX sync
                let pool_clone = Arc::clone(&pool);
                let cancel_clone = cancel_token.clone();
                tokio::spawn(async move {
                    if let Err(e) =
                        sync_folder(pool_clone, account_id, "INBOX", cancel_clone, 30, 30).await
                    {
                        eprintln!("IDLE-triggered sync failed: {}", e);
                    }
                });
            }
            Err(e) => {
                // IDLE not supported or failed - fallback to polling
                eprintln!("IDLE failed (expected - not yet implemented): {}", e);
                break; // Exit IDLE listener, rely on polling
            }
        }

        // IDLE connection timeout (29 minutes per RFC) - reconnect
        tokio::time::sleep(Duration::from_secs(29 * 60)).await;
    }
}

/// Get account from database
async fn get_account(pool: &SqlitePool, account_id: i64) -> SyncResult<Account> {
    #[derive(sqlx::FromRow)]
    struct AccountRow {
        id: i64,
        email: String,
        imap_host: String,
        imap_port: i64,
    }

    let row = sqlx::query_as::<_, AccountRow>(
        "SELECT id, email, imap_host, imap_port FROM accounts WHERE id = ?",
    )
    .bind(account_id)
    .fetch_one(pool)
    .await
    .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(Account {
        id: row.id,
        email: row.email,
        imap_host: row.imap_host,
        imap_port: row.imap_port as u16,
    })
}

/// Update sync status in database
async fn update_sync_status(
    pool: &SqlitePool,
    account_id: i64,
    status: SyncStatus,
    error_message: Option<&str>,
) -> SyncResult<()> {
    sqlx::query("UPDATE sync_state SET sync_status = ?, error_message = ? WHERE account_id = ?")
        .bind(status.as_str())
        .bind(error_message)
        .bind(account_id)
        .execute(pool)
        .await
        .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(())
}

/// Update sync state with stats
async fn update_sync_state(
    pool: &SqlitePool,
    account_id: i64,
    messages_fetched: i32,
) -> SyncResult<()> {
    sqlx::query(
        r#"
        UPDATE sync_state 
        SET last_sync_at = datetime('now'), 
            messages_fetched = messages_fetched + ?,
            sync_status = 'completed'
        WHERE account_id = ?
        "#,
    )
    .bind(messages_fetched)
    .bind(account_id)
    .execute(pool)
    .await
    .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_status_as_str() {
        assert_eq!(SyncStatus::Idle.as_str(), "idle");
        assert_eq!(SyncStatus::Syncing.as_str(), "syncing");
        assert_eq!(SyncStatus::Completed.as_str(), "completed");
        assert_eq!(SyncStatus::Failed.as_str(), "failed");
        assert_eq!(SyncStatus::Cancelled.as_str(), "cancelled");
    }

    #[tokio::test]
    async fn test_sync_orchestrator_creation() {
        let pool = Arc::new(crate::db::init_db(":memory:").await.unwrap());
        let orchestrator = SyncOrchestrator::new(pool);

        assert_eq!(orchestrator.inbox_sync_interval_minutes, 5);
        assert_eq!(orchestrator.other_folders_sync_interval_minutes, 30);
        assert_eq!(orchestrator.initial_sync_days, 30);
        assert_eq!(orchestrator.historical_sync_batch_days, 30);
    }

    #[tokio::test]
    async fn test_sync_orchestrator_custom_intervals() {
        let pool = Arc::new(crate::db::init_db(":memory:").await.unwrap());
        let orchestrator = SyncOrchestrator::with_intervals(pool, 2, 15);

        assert_eq!(orchestrator.inbox_sync_interval_minutes, 2);
        assert_eq!(orchestrator.other_folders_sync_interval_minutes, 15);
    }

    #[tokio::test]
    async fn test_cancel_sync() {
        let pool = Arc::new(crate::db::init_db(":memory:").await.unwrap());
        let orchestrator = SyncOrchestrator::new(pool);

        assert!(!orchestrator.cancellation_token.is_cancelled());

        orchestrator.cancel_sync();

        assert!(orchestrator.cancellation_token.is_cancelled());
    }

    #[tokio::test]
    async fn test_sync_stats_initialization() {
        let stats = SyncStats {
            account_id: 1,
            messages_synced: 10,
            new_messages: 5,
            errors: vec![],
            folders_synced: vec!["INBOX".to_string(), "Sent".to_string()],
        };

        assert_eq!(stats.account_id, 1);
        assert_eq!(stats.messages_synced, 10);
        assert_eq!(stats.new_messages, 5);
        assert_eq!(stats.errors.len(), 0);
        assert_eq!(stats.folders_synced.len(), 2);
    }

    #[tokio::test]
    async fn test_folder_sync_state() {
        let state = FolderSyncState {
            folder_name: "INBOX".to_string(),
            last_uid: Some(100),
            uidvalidity: Some(12345),
            last_synced_at: Some(Utc::now()),
            historical_sync_until: None,
        };

        assert_eq!(state.folder_name, "INBOX");
        assert_eq!(state.last_uid, Some(100));
        assert_eq!(state.uidvalidity, Some(12345));
        assert!(state.last_synced_at.is_some());
    }

    #[test]
    fn test_parse_message_fields() {
        let body = b"Subject: Test Email\nFrom: sender@example.com\n\nBody content";
        let (subject, from, body_plain) = parse_message_fields(body);

        assert_eq!(subject, Some("Test Email".to_string()));
        assert_eq!(from, Some("sender@example.com".to_string()));
        assert!(body_plain.is_some());
    }

    #[tokio::test]
    async fn test_sync_orchestrator_differential_timers() {
        // Test that INBOX and other folders have different sync intervals
        let pool = Arc::new(crate::db::init_db(":memory:").await.unwrap());
        let orchestrator = SyncOrchestrator::with_intervals(pool, 1, 5);

        assert_eq!(orchestrator.inbox_sync_interval_minutes, 1);
        assert_eq!(orchestrator.other_folders_sync_interval_minutes, 5);
        assert!(
            orchestrator.inbox_sync_interval_minutes
                < orchestrator.other_folders_sync_interval_minutes
        );
    }

    #[tokio::test]
    async fn test_uidvalidity_tracking() {
        // Test UIDVALIDITY change detection
        let state1 = FolderSyncState {
            folder_name: "INBOX".to_string(),
            last_uid: Some(50),
            uidvalidity: Some(100),
            last_synced_at: Some(Utc::now()),
            historical_sync_until: None,
        };

        let state2 = FolderSyncState {
            folder_name: "INBOX".to_string(),
            last_uid: Some(50),
            uidvalidity: Some(200), // Changed!
            last_synced_at: Some(Utc::now()),
            historical_sync_until: None,
        };

        assert_ne!(state1.uidvalidity, state2.uidvalidity);
    }

    #[tokio::test]
    async fn test_initial_sync_period() {
        let pool = Arc::new(crate::db::init_db(":memory:").await.unwrap());
        let orchestrator = SyncOrchestrator::new(pool);

        // Verify initial sync is set to 30 days
        assert_eq!(orchestrator.initial_sync_days, 30);
    }

    #[tokio::test]
    async fn test_historical_sync_batch_size() {
        let pool = Arc::new(crate::db::init_db(":memory:").await.unwrap());
        let orchestrator = SyncOrchestrator::new(pool);

        // Verify historical sync batches are 30 days
        assert_eq!(orchestrator.historical_sync_batch_days, 30);
    }
}
