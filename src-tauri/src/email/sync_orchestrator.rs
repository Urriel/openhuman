//! Background Sync Orchestration
//!
//! This module manages periodic email synchronization across multiple accounts.
//! Features:
//! - Initial sync on app launch
//! - Periodic background sync with configurable interval
//! - Parallel multi-account sync
//! - Sync cancellation support
//! - Progress tracking and events

use crate::email::pop3::Pop3Client;
use crate::error::{SyncError, SyncResult};
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::time::{interval, Duration};
use tokio_util::sync::CancellationToken;

/// Sync orchestrator for managing email synchronization
pub struct SyncOrchestrator {
    pool: Arc<SqlitePool>,
    cancellation_token: CancellationToken,
    sync_interval_minutes: u64,
}

/// Sync statistics for a single account
#[derive(Debug, Clone)]
pub struct SyncStats {
    pub account_id: i64,
    pub messages_synced: i32,
    pub new_messages: i32,
    pub errors: Vec<String>,
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

impl SyncOrchestrator {
    /// Create a new sync orchestrator
    pub fn new(pool: Arc<SqlitePool>, sync_interval_minutes: u64) -> Self {
        Self {
            pool,
            cancellation_token: CancellationToken::new(),
            sync_interval_minutes,
        }
    }

    /// Start the sync scheduler with periodic background sync
    pub async fn start_scheduler(&self) -> SyncResult<()> {
        // Run initial sync on app launch
        self.sync_all_accounts().await?;

        // Start periodic background sync
        let mut timer = interval(Duration::from_secs(self.sync_interval_minutes * 60));

        loop {
            tokio::select! {
                _ = self.cancellation_token.cancelled() => {
                    // Sync cancelled
                    break;
                }
                _ = timer.tick() => {
                    // Periodic sync
                    if let Err(e) = self.sync_all_accounts().await {
                        eprintln!("Periodic sync failed: {}", e);
                    }
                }
            }
        }

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

            let task =
                tokio::spawn(async move { sync_account(pool, account.id, cancel_token).await });

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
        )
        .await
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
            pop3_host: String,
            pop3_port: i64,
        }

        let accounts = sqlx::query_as::<_, AccountRow>(
            "SELECT id, email, pop3_host, pop3_port FROM accounts WHERE sync_enabled = 1",
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| SyncError::DatabaseError(e.into()))?;

        Ok(accounts
            .into_iter()
            .map(|row| Account {
                id: row.id,
                email: row.email,
                pop3_host: row.pop3_host,
                pop3_port: row.pop3_port as u16,
            })
            .collect())
    }
}

/// Account info for syncing
#[derive(Debug, Clone)]
struct Account {
    id: i64,
    email: String,
    pop3_host: String,
    pop3_port: u16,
}

/// Sync a single account
async fn sync_account(
    pool: Arc<SqlitePool>,
    account_id: i64,
    cancel_token: CancellationToken,
) -> SyncResult<SyncStats> {
    // Update sync status to syncing
    update_sync_status(&pool, account_id, SyncStatus::Syncing, None).await?;

    let mut stats = SyncStats {
        account_id,
        messages_synced: 0,
        new_messages: 0,
        errors: Vec::new(),
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

    // Connect to POP3 server
    let _client = Pop3Client::connect(
        &account.pop3_host,
        account.pop3_port,
        &account.email,
        &password,
    )
    .await
    .map_err(SyncError::Pop3Error)?;

    // Get new message UIDLs (incremental sync would be implemented here)
    // For now, this is a placeholder
    let new_uidls: Vec<String> = vec![]; // client.fetch_new_uidls().await?;

    stats.new_messages = new_uidls.len() as i32;
    stats.messages_synced = new_uidls.len() as i32;

    // Update sync state
    update_sync_state(&pool, account_id, stats.messages_synced).await?;

    // Mark sync as completed
    update_sync_status(&pool, account_id, SyncStatus::Completed, None).await?;

    Ok(stats)
}

/// Get account from database
async fn get_account(pool: &SqlitePool, account_id: i64) -> SyncResult<Account> {
    #[derive(sqlx::FromRow)]
    struct AccountRow {
        id: i64,
        email: String,
        pop3_host: String,
        pop3_port: i64,
    }

    let row = sqlx::query_as::<_, AccountRow>(
        "SELECT id, email, pop3_host, pop3_port FROM accounts WHERE id = ?",
    )
    .bind(account_id)
    .fetch_one(pool)
    .await
    .map_err(|e| SyncError::DatabaseError(e.into()))?;

    Ok(Account {
        id: row.id,
        email: row.email,
        pop3_host: row.pop3_host,
        pop3_port: row.pop3_port as u16,
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
        let orchestrator = SyncOrchestrator::new(pool, 5);

        assert_eq!(orchestrator.sync_interval_minutes, 5);
    }

    #[tokio::test]
    async fn test_cancel_sync() {
        let pool = Arc::new(crate::db::init_db(":memory:").await.unwrap());
        let orchestrator = SyncOrchestrator::new(pool, 5);

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
        };

        assert_eq!(stats.account_id, 1);
        assert_eq!(stats.messages_synced, 10);
        assert_eq!(stats.new_messages, 5);
        assert_eq!(stats.errors.len(), 0);
    }
}
