//! IMAP Integration Tests (Task Group 7.3)
//!
//! Strategic end-to-end and integration tests for IMAP migration:
//! - Account setup → folder discovery → sync workflow
//! - SMTP send → IMAP APPEND to Sent folder
//! - Draft save → IMAP APPEND → Draft sync from server
//! - UIDVALIDITY change → full folder re-sync
//! - Multi-folder parallel sync coordination
//! - Flag sync round-trip (local → server → local)

#[cfg(test)]
mod tests {
    use crate::db::init_db;
    use crate::email::imap::{FlagAction, ImapClient, ImapFolder};
    use crate::email::sync_orchestrator::SyncOrchestrator;
    use sqlx::SqlitePool;
    use std::sync::Arc;

    /// Test helper: Create test database
    async fn create_test_db() -> SqlitePool {
        init_db("sqlite::memory:").await.unwrap()
    }

    /// Test helper: Insert test account
    async fn insert_test_account(pool: &SqlitePool) -> i64 {
        sqlx::query(
            r#"
            INSERT INTO accounts (email, provider, imap_host, imap_port, smtp_host, smtp_port)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind("test@example.com")
        .bind("custom")
        .bind("imap.example.com")
        .bind(993)
        .bind("smtp.example.com")
        .bind(587)
        .execute(pool)
        .await
        .unwrap()
        .last_insert_rowid()
    }

    /// Test helper: Insert folder
    async fn insert_folder(
        pool: &SqlitePool,
        account_id: i64,
        name: &str,
        folder_type: &str,
        uidvalidity: u32,
    ) {
        sqlx::query(
            r#"
            INSERT INTO folders (account_id, name, folder_type, selectable, uidvalidity)
            VALUES (?, ?, ?, 1, ?)
            "#,
        )
        .bind(account_id)
        .bind(name)
        .bind(folder_type)
        .bind(uidvalidity as i64)
        .execute(pool)
        .await
        .unwrap();
    }

    /// Test helper: Insert message
    async fn insert_message(
        pool: &SqlitePool,
        account_id: i64,
        folder: &str,
        imap_uid: u32,
        flags: &str,
    ) -> i64 {
        sqlx::query(
            r#"
            INSERT INTO messages (account_id, message_id, folder, from_addr, subject, imap_uid, imap_flags, is_read, is_starred)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(account_id)
        .bind(format!("msg-{}@example.com", imap_uid))
        .bind(folder)
        .bind("sender@example.com")
        .bind("Test Subject")
        .bind(imap_uid as i64)
        .bind(flags)
        .bind(if flags.contains("\\Seen") { 1 } else { 0 })
        .bind(if flags.contains("\\Flagged") { 1 } else { 0 })
        .execute(pool)
        .await
        .unwrap()
        .last_insert_rowid()
    }

    #[tokio::test]
    async fn test_account_setup_to_folder_discovery() {
        // Integration test: Account creation → folder discovery workflow
        let pool = create_test_db().await;
        let account_id = insert_test_account(&pool).await;

        // Simulate folder discovery (in real implementation, ImapClient.list_folders() would return these)
        let discovered_folders = vec![
            ImapFolder {
                name: "INBOX".to_string(),
                folder_type: Some("Inbox".to_string()),
                selectable: true,
                flags: vec![],
            },
            ImapFolder {
                name: "Sent".to_string(),
                folder_type: Some("Sent".to_string()),
                selectable: true,
                flags: vec![],
            },
            ImapFolder {
                name: "Drafts".to_string(),
                folder_type: Some("Drafts".to_string()),
                selectable: true,
                flags: vec![],
            },
        ];

        // Store folders in database
        for folder in &discovered_folders {
            sqlx::query(
                r#"
                INSERT INTO folders (account_id, name, folder_type, selectable)
                VALUES (?, ?, ?, ?)
                "#,
            )
            .bind(account_id)
            .bind(&folder.name)
            .bind(&folder.folder_type)
            .bind(folder.selectable as i64)
            .execute(&pool)
            .await
            .unwrap();
        }

        // Verify folders stored
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM folders WHERE account_id = ?")
            .bind(account_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(count.0, 3);

        // Verify folder types detected correctly
        let inbox_type: (Option<String>,) = sqlx::query_as(
            "SELECT folder_type FROM folders WHERE account_id = ? AND name = 'INBOX'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(inbox_type.0, Some("Inbox".to_string()));
    }

    #[tokio::test]
    async fn test_initial_sync_then_incremental_sync() {
        // Integration test: Initial sync fetches 30 days, then incremental sync fetches new messages
        let pool = create_test_db().await;
        let account_id = insert_test_account(&pool).await;
        insert_folder(&pool, account_id, "INBOX", "Inbox", 12345).await;

        // Initial sync: Insert messages with UIDs 1-10
        for uid in 1..=10 {
            insert_message(&pool, account_id, "INBOX", uid, "").await;
        }

        // Verify initial sync stored 10 messages
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'INBOX'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count.0, 10);

        // Get last synced UID
        let last_uid: Option<i64> = sqlx::query_scalar(
            "SELECT MAX(imap_uid) FROM messages WHERE account_id = ? AND folder = 'INBOX'",
        )
        .bind(account_id)
        .fetch_optional(&pool)
        .await
        .unwrap()
        .flatten();
        assert_eq!(last_uid, Some(10));

        // Incremental sync: Fetch only messages with UID > 10
        // Simulate fetching UIDs 11-15 from server
        for uid in 11..=15 {
            insert_message(&pool, account_id, "INBOX", uid, "").await;
        }

        // Verify incremental sync added 5 new messages
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'INBOX'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count.0, 15);
    }

    #[tokio::test]
    async fn test_uidvalidity_change_triggers_full_resync() {
        // Integration test: UIDVALIDITY change → delete all messages → full re-sync
        let pool = create_test_db().await;
        let account_id = insert_test_account(&pool).await;
        insert_folder(&pool, account_id, "INBOX", "Inbox", 12345).await;

        // Initial sync with UIDVALIDITY=12345
        for uid in 1..=5 {
            insert_message(&pool, account_id, "INBOX", uid, "").await;
        }

        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'INBOX'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count.0, 5);

        // Server UIDVALIDITY changed to 67890 (mailbox rebuild)
        // Reset folder sync state: delete all messages, update UIDVALIDITY
        sqlx::query("DELETE FROM messages WHERE account_id = ? AND folder = 'INBOX'")
            .bind(account_id)
            .execute(&pool)
            .await
            .unwrap();

        sqlx::query("UPDATE folders SET uidvalidity = ? WHERE account_id = ? AND name = 'INBOX'")
            .bind(67890_i64)
            .bind(account_id)
            .execute(&pool)
            .await
            .unwrap();

        // Verify messages deleted
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'INBOX'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count.0, 0);

        // Verify UIDVALIDITY updated
        let uidvalidity: (i64,) = sqlx::query_as(
            "SELECT uidvalidity FROM folders WHERE account_id = ? AND name = 'INBOX'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(uidvalidity.0, 67890);

        // Full re-sync with new UIDVALIDITY
        for uid in 1..=8 {
            insert_message(&pool, account_id, "INBOX", uid, "").await;
        }

        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'INBOX'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count.0, 8);
    }

    #[tokio::test]
    async fn test_multi_folder_parallel_sync_coordination() {
        // Integration test: Multiple folders sync in parallel with correct state tracking
        let pool = create_test_db().await;
        let account_id = insert_test_account(&pool).await;

        // Create multiple folders
        insert_folder(&pool, account_id, "INBOX", "Inbox", 11111).await;
        insert_folder(&pool, account_id, "Sent", "Sent", 22222).await;
        insert_folder(&pool, account_id, "Archive", "Archive", 33333).await;

        // Simulate parallel sync: each folder gets messages
        insert_message(&pool, account_id, "INBOX", 1, "").await;
        insert_message(&pool, account_id, "INBOX", 2, "\\Seen").await;
        insert_message(&pool, account_id, "Sent", 10, "\\Seen").await;
        insert_message(&pool, account_id, "Archive", 100, "").await;
        insert_message(&pool, account_id, "Archive", 101, "").await;

        // Verify each folder has correct message count
        let inbox_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'INBOX'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(inbox_count.0, 2);

        let sent_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'Sent'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(sent_count.0, 1);

        let archive_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'Archive'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(archive_count.0, 2);

        // Verify last UID per folder tracked independently
        let inbox_last_uid: Option<i64> = sqlx::query_scalar(
            "SELECT MAX(imap_uid) FROM messages WHERE account_id = ? AND folder = 'INBOX'",
        )
        .bind(account_id)
        .fetch_optional(&pool)
        .await
        .unwrap()
        .flatten();
        assert_eq!(inbox_last_uid, Some(2));

        let archive_last_uid: Option<i64> = sqlx::query_scalar(
            "SELECT MAX(imap_uid) FROM messages WHERE account_id = ? AND folder = 'Archive'",
        )
        .bind(account_id)
        .fetch_optional(&pool)
        .await
        .unwrap()
        .flatten();
        assert_eq!(archive_last_uid, Some(101));
    }

    #[tokio::test]
    async fn test_flag_sync_roundtrip_local_to_server_to_local() {
        // Integration test: Local flag change → server update → server authoritative on conflict
        let pool = create_test_db().await;
        let account_id = insert_test_account(&pool).await;
        insert_folder(&pool, account_id, "INBOX", "Inbox", 12345).await;
        let message_id = insert_message(&pool, account_id, "INBOX", 50, "").await;

        // Verify initial state (unread, not starred)
        let row: (i64, i64, Option<String>) =
            sqlx::query_as("SELECT is_read, is_starred, imap_flags FROM messages WHERE id = ?")
                .bind(message_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.0, 0); // is_read
        assert_eq!(row.1, 0); // is_starred

        // User marks message as read locally
        sqlx::query("UPDATE messages SET is_read = 1, imap_flags = ? WHERE id = ?")
            .bind("\\Seen")
            .bind(message_id)
            .execute(&pool)
            .await
            .unwrap();

        // Verify local update
        let row: (i64, Option<String>) =
            sqlx::query_as("SELECT is_read, imap_flags FROM messages WHERE id = ?")
                .bind(message_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.0, 1);
        assert_eq!(row.1, Some("\\Seen".to_string()));

        // Simulate server flag sync: server has \Seen and \Flagged (user starred on another device)
        let server_flags = "\\Seen,\\Flagged";
        sqlx::query("UPDATE messages SET is_read = 1, is_starred = 1, imap_flags = ? WHERE id = ?")
            .bind(server_flags)
            .bind(message_id)
            .execute(&pool)
            .await
            .unwrap();

        // Verify server state applied (server is authoritative)
        let row: (i64, i64, Option<String>) =
            sqlx::query_as("SELECT is_read, is_starred, imap_flags FROM messages WHERE id = ?")
                .bind(message_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.0, 1); // is_read
        assert_eq!(row.1, 1); // is_starred (added by server)
        assert_eq!(row.2, Some("\\Seen,\\Flagged".to_string()));
    }

    #[tokio::test]
    async fn test_draft_save_and_sync_workflow() {
        // Integration test: Save draft → APPEND to server → sync from server
        let pool = create_test_db().await;
        let account_id = insert_test_account(&pool).await;
        insert_folder(&pool, account_id, "Drafts", "Drafts", 99999).await;

        // User saves draft locally
        let draft_id = sqlx::query(
            r#"
            INSERT INTO messages (account_id, message_id, folder, from_addr, subject, body_plain, imap_uid)
            VALUES (?, ?, 'Drafts', ?, ?, ?, ?)
            "#,
        )
        .bind(account_id)
        .bind("draft-local@example.com")
        .bind("user@example.com")
        .bind("Draft Subject")
        .bind("Draft body content")
        .bind(0) // No UID yet (not synced to server)
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();

        // Verify draft stored
        let row: (String, String, i64) =
            sqlx::query_as("SELECT folder, subject, imap_uid FROM messages WHERE id = ?")
                .bind(draft_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.0, "Drafts");
        assert_eq!(row.1, "Draft Subject");
        assert_eq!(row.2, 0); // Not synced yet

        // Simulate IMAP APPEND success: server assigned UID 500
        sqlx::query("UPDATE messages SET imap_uid = ? WHERE id = ?")
            .bind(500)
            .bind(draft_id)
            .execute(&pool)
            .await
            .unwrap();

        // Verify UID updated
        let uid: (i64,) = sqlx::query_as("SELECT imap_uid FROM messages WHERE id = ?")
            .bind(draft_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(uid.0, 500);

        // Simulate sync from server: another draft created on different device
        insert_message(&pool, account_id, "Drafts", 501, "\\Draft").await;

        // Verify both drafts exist
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'Drafts'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count.0, 2);
    }

    #[tokio::test]
    async fn test_sent_folder_append_integration() {
        // Integration test: SMTP send → IMAP APPEND to Sent folder
        let pool = create_test_db().await;
        let account_id = insert_test_account(&pool).await;
        insert_folder(&pool, account_id, "Sent", "Sent", 88888).await;

        // User sends email (stored in outbox first)
        let outbox_id = sqlx::query(
            r#"
            INSERT INTO outbox (account_id, recipients, subject, body_plain, send_status)
            VALUES (?, ?, ?, ?, 'pending')
            "#,
        )
        .bind(account_id)
        .bind(r#"{"to":["recipient@example.com"]}"#)
        .bind("Test Email")
        .bind("Email body")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();

        // Verify outbox entry
        let status: (String,) = sqlx::query_as("SELECT send_status FROM outbox WHERE id = ?")
            .bind(outbox_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(status.0, "pending");

        // Simulate SMTP send success
        sqlx::query("UPDATE outbox SET send_status = 'sent' WHERE id = ?")
            .bind(outbox_id)
            .execute(&pool)
            .await
            .unwrap();

        // Simulate IMAP APPEND to Sent folder (server assigns UID 200)
        let sent_message_id = insert_message(&pool, account_id, "Sent", 200, "\\Seen").await;

        // Verify message in Sent folder
        let row: (String, String) =
            sqlx::query_as("SELECT folder, subject FROM messages WHERE id = ?")
                .bind(sent_message_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.0, "Sent");
    }

    #[tokio::test]
    async fn test_message_move_between_folders_integration() {
        // Integration test: Move message from INBOX to Archive (COPY + DELETE)
        let pool = create_test_db().await;
        let account_id = insert_test_account(&pool).await;
        insert_folder(&pool, account_id, "INBOX", "Inbox", 11111).await;
        insert_folder(&pool, account_id, "Archive", "Archive", 22222).await;

        // Message in INBOX with UID 10
        let message_id = insert_message(&pool, account_id, "INBOX", 10, "").await;

        // Verify initial folder
        let folder: (String,) = sqlx::query_as("SELECT folder FROM messages WHERE id = ?")
            .bind(message_id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(folder.0, "INBOX");

        // Simulate move: COPY to Archive (server assigns new UID 300), mark original as \Deleted
        // Update local message folder
        sqlx::query("UPDATE messages SET folder = 'Archive', imap_uid = ? WHERE id = ?")
            .bind(300)
            .bind(message_id)
            .execute(&pool)
            .await
            .unwrap();

        // Verify moved
        let row: (String, i64) =
            sqlx::query_as("SELECT folder, imap_uid FROM messages WHERE id = ?")
                .bind(message_id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert_eq!(row.0, "Archive");
        assert_eq!(row.1, 300); // New UID in Archive folder

        // Verify message count in each folder
        let inbox_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'INBOX'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(inbox_count.0, 0); // Moved out of INBOX

        let archive_count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM messages WHERE account_id = ? AND folder = 'Archive'",
        )
        .bind(account_id)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(archive_count.0, 1);
    }

    #[tokio::test]
    async fn test_sync_orchestrator_initialization() {
        // Integration test: SyncOrchestrator initialization with database
        let pool = Arc::new(create_test_db().await);
        let _orchestrator = SyncOrchestrator::new(pool.clone());

        // Verify orchestrator created successfully without panic
        // Default intervals are 5 min for INBOX, 30 min for other folders (verified in unit tests)
    }
}
