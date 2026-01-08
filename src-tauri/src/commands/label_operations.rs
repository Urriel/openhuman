//! Label Operations Commands
//!
//! Tauri commands for Gmail-style label management (create, apply, remove, list)

use serde::{Deserialize, Serialize};

/// Label data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub color: Option<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<i64>,
}

/// Create a new label
#[tauri::command]
pub async fn create_label(
    account_id: i64,
    name: String,
    color: Option<String>,
) -> Result<i64, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    let result = sqlx::query(
        r#"
        INSERT INTO labels (account_id, name, color)
        VALUES (?, ?, ?)
        "#,
    )
    .bind(account_id)
    .bind(&name)
    .bind(&color)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to create label: {}", e))?;

    Ok(result.last_insert_rowid())
}

/// List all labels for an account with optional message counts
#[tauri::command]
pub async fn list_labels(account_id: i64, include_counts: bool) -> Result<Vec<Label>, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    let labels = if include_counts {
        sqlx::query_as::<_, (i64, i64, String, Option<String>, String, i64)>(
            r#"
            SELECT 
                l.id, 
                l.account_id, 
                l.name, 
                l.color, 
                l.created_at,
                COUNT(ml.message_id) as message_count
            FROM labels l
            LEFT JOIN message_labels ml ON l.id = ml.label_id
            WHERE l.account_id = ?
            GROUP BY l.id
            ORDER BY l.name ASC
            "#,
        )
        .bind(account_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to list labels: {}", e))?
        .into_iter()
        .map(
            |(id, account_id, name, color, created_at, message_count)| Label {
                id,
                account_id,
                name,
                color,
                created_at,
                message_count: Some(message_count),
            },
        )
        .collect()
    } else {
        sqlx::query_as::<_, (i64, i64, String, Option<String>, String)>(
            r#"
            SELECT id, account_id, name, color, created_at
            FROM labels
            WHERE account_id = ?
            ORDER BY name ASC
            "#,
        )
        .bind(account_id)
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to list labels: {}", e))?
        .into_iter()
        .map(|(id, account_id, name, color, created_at)| Label {
            id,
            account_id,
            name,
            color,
            created_at,
            message_count: None,
        })
        .collect()
    };

    Ok(labels)
}

/// Delete a label
#[tauri::command]
pub async fn delete_label(label_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM labels WHERE id = ?")
        .bind(label_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete label: {}", e))?;

    Ok(())
}

/// Apply a label to a message
#[tauri::command]
pub async fn apply_label(message_id: i64, label_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    sqlx::query(
        r#"
        INSERT OR IGNORE INTO message_labels (message_id, label_id)
        VALUES (?, ?)
        "#,
    )
    .bind(message_id)
    .bind(label_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to apply label: {}", e))?;

    Ok(())
}

/// Remove a label from a message
#[tauri::command]
pub async fn remove_label(message_id: i64, label_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM message_labels WHERE message_id = ? AND label_id = ?")
        .bind(message_id)
        .bind(label_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to remove label: {}", e))?;

    Ok(())
}

/// Get all labels for a specific message
#[tauri::command]
pub async fn get_message_labels(message_id: i64) -> Result<Vec<Label>, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    let labels = sqlx::query_as::<_, (i64, i64, String, Option<String>, String)>(
        r#"
        SELECT l.id, l.account_id, l.name, l.color, l.created_at
        FROM labels l
        INNER JOIN message_labels ml ON l.id = ml.label_id
        WHERE ml.message_id = ?
        ORDER BY l.name ASC
        "#,
    )
    .bind(message_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to get message labels: {}", e))?
    .into_iter()
    .map(|(id, account_id, name, color, created_at)| Label {
        id,
        account_id,
        name,
        color,
        created_at,
        message_count: None,
    })
    .collect();

    Ok(labels)
}

/// Archive messages (move to Archive folder)
#[tauri::command]
pub async fn archive_messages(message_ids: Vec<i64>) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    for message_id in message_ids {
        sqlx::query("UPDATE messages SET folder = 'Archive' WHERE id = ?")
            .bind(message_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to archive message: {}", e))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;

    async fn setup_test_account(pool: &sqlx::SqlitePool) -> i64 {
        let result = sqlx::query(
            "INSERT INTO accounts (email, provider, imap_host, smtp_host) VALUES (?, ?, ?, ?)",
        )
        .bind("test@example.com")
        .bind("custom")
        .bind("imap.example.com")
        .bind("smtp.example.com")
        .execute(pool)
        .await
        .expect("Failed to create test account");

        result.last_insert_rowid()
    }

    async fn setup_test_message(pool: &sqlx::SqlitePool, account_id: i64) -> i64 {
        let result = sqlx::query(
            "INSERT INTO messages (account_id, message_id, from_addr) VALUES (?, ?, ?)",
        )
        .bind(account_id)
        .bind("msg-123")
        .bind("sender@example.com")
        .execute(pool)
        .await
        .expect("Failed to create test message");

        result.last_insert_rowid()
    }

    #[tokio::test]
    async fn test_create_label() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let account_id = setup_test_account(&pool).await;

        let result = sqlx::query("INSERT INTO labels (account_id, name, color) VALUES (?, ?, ?)")
            .bind(account_id)
            .bind("Work")
            .bind(Some("#ff0000"))
            .execute(&pool)
            .await;

        assert!(result.is_ok(), "Should create label successfully");

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM labels WHERE account_id = ?")
            .bind(account_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to count labels");

        assert_eq!(count.0, 1, "Should have one label");
    }

    #[tokio::test]
    async fn test_label_unique_constraint() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let account_id = setup_test_account(&pool).await;

        // Create first label
        sqlx::query("INSERT INTO labels (account_id, name) VALUES (?, ?)")
            .bind(account_id)
            .bind("Work")
            .execute(&pool)
            .await
            .expect("First label should succeed");

        // Try to create duplicate label (should fail)
        let result = sqlx::query("INSERT INTO labels (account_id, name) VALUES (?, ?)")
            .bind(account_id)
            .bind("Work")
            .execute(&pool)
            .await;

        assert!(result.is_err(), "Duplicate label should fail");
    }

    #[tokio::test]
    async fn test_apply_and_remove_label() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let account_id = setup_test_account(&pool).await;
        let message_id = setup_test_message(&pool, account_id).await;

        // Create label
        let label_result = sqlx::query("INSERT INTO labels (account_id, name) VALUES (?, ?)")
            .bind(account_id)
            .bind("Important")
            .execute(&pool)
            .await
            .expect("Failed to create label");
        let label_id = label_result.last_insert_rowid();

        // Apply label to message
        let apply_result =
            sqlx::query("INSERT INTO message_labels (message_id, label_id) VALUES (?, ?)")
                .bind(message_id)
                .bind(label_id)
                .execute(&pool)
                .await;

        assert!(apply_result.is_ok(), "Should apply label successfully");

        // Verify label was applied
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM message_labels WHERE message_id = ?")
                .bind(message_id)
                .fetch_one(&pool)
                .await
                .expect("Failed to count message labels");

        assert_eq!(count.0, 1, "Message should have one label");

        // Remove label
        let remove_result =
            sqlx::query("DELETE FROM message_labels WHERE message_id = ? AND label_id = ?")
                .bind(message_id)
                .bind(label_id)
                .execute(&pool)
                .await;

        assert!(remove_result.is_ok(), "Should remove label successfully");

        // Verify label was removed
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM message_labels WHERE message_id = ?")
                .bind(message_id)
                .fetch_one(&pool)
                .await
                .expect("Failed to count message labels");

        assert_eq!(count.0, 0, "Message should have no labels");
    }

    #[tokio::test]
    async fn test_message_label_cascade_delete_on_message() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let account_id = setup_test_account(&pool).await;
        let message_id = setup_test_message(&pool, account_id).await;

        // Create label and apply to message
        let label_result = sqlx::query("INSERT INTO labels (account_id, name) VALUES (?, ?)")
            .bind(account_id)
            .bind("Test")
            .execute(&pool)
            .await
            .expect("Failed to create label");
        let label_id = label_result.last_insert_rowid();

        sqlx::query("INSERT INTO message_labels (message_id, label_id) VALUES (?, ?)")
            .bind(message_id)
            .bind(label_id)
            .execute(&pool)
            .await
            .expect("Failed to apply label");

        // Delete message
        sqlx::query("DELETE FROM messages WHERE id = ?")
            .bind(message_id)
            .execute(&pool)
            .await
            .expect("Failed to delete message");

        // Verify message_label was cascade deleted
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM message_labels WHERE message_id = ?")
                .bind(message_id)
                .fetch_one(&pool)
                .await
                .expect("Failed to count message labels");

        assert_eq!(count.0, 0, "Message labels should be cascade deleted");
    }

    #[tokio::test]
    async fn test_message_label_cascade_delete_on_label() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let account_id = setup_test_account(&pool).await;
        let message_id = setup_test_message(&pool, account_id).await;

        // Create label and apply to message
        let label_result = sqlx::query("INSERT INTO labels (account_id, name) VALUES (?, ?)")
            .bind(account_id)
            .bind("Test")
            .execute(&pool)
            .await
            .expect("Failed to create label");
        let label_id = label_result.last_insert_rowid();

        sqlx::query("INSERT INTO message_labels (message_id, label_id) VALUES (?, ?)")
            .bind(message_id)
            .bind(label_id)
            .execute(&pool)
            .await
            .expect("Failed to apply label");

        // Delete label
        sqlx::query("DELETE FROM labels WHERE id = ?")
            .bind(label_id)
            .execute(&pool)
            .await
            .expect("Failed to delete label");

        // Verify message_label was cascade deleted
        let count: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM message_labels WHERE label_id = ?")
                .bind(label_id)
                .fetch_one(&pool)
                .await
                .expect("Failed to count message labels");

        assert_eq!(count.0, 0, "Message labels should be cascade deleted");
    }

    #[tokio::test]
    async fn test_label_count_query() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let account_id = setup_test_account(&pool).await;
        let msg1 = setup_test_message(&pool, account_id).await;

        // Create second message
        let msg2 = sqlx::query(
            "INSERT INTO messages (account_id, message_id, from_addr) VALUES (?, ?, ?)",
        )
        .bind(account_id)
        .bind("msg-456")
        .bind("sender2@example.com")
        .execute(&pool)
        .await
        .expect("Failed to create second message")
        .last_insert_rowid();

        // Create label
        let label_result = sqlx::query("INSERT INTO labels (account_id, name) VALUES (?, ?)")
            .bind(account_id)
            .bind("Work")
            .execute(&pool)
            .await
            .expect("Failed to create label");
        let label_id = label_result.last_insert_rowid();

        // Apply label to both messages
        sqlx::query("INSERT INTO message_labels (message_id, label_id) VALUES (?, ?)")
            .bind(msg1)
            .bind(label_id)
            .execute(&pool)
            .await
            .expect("Failed to apply label to msg1");

        sqlx::query("INSERT INTO message_labels (message_id, label_id) VALUES (?, ?)")
            .bind(msg2)
            .bind(label_id)
            .execute(&pool)
            .await
            .expect("Failed to apply label to msg2");

        // Query label with count
        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(ml.message_id)
            FROM labels l
            LEFT JOIN message_labels ml ON l.id = ml.label_id
            WHERE l.id = ?
            GROUP BY l.id
            "#,
        )
        .bind(label_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to query label count");

        assert_eq!(result.0, 2, "Label should have 2 messages");
    }

    #[tokio::test]
    async fn test_archive_messages() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let account_id = setup_test_account(&pool).await;
        let message_id = setup_test_message(&pool, account_id).await;

        // Verify message is in INBOX
        let folder: (String,) = sqlx::query_as("SELECT folder FROM messages WHERE id = ?")
            .bind(message_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to get folder");

        assert_eq!(folder.0, "INBOX", "Message should start in INBOX");

        // Archive message
        sqlx::query("UPDATE messages SET folder = 'Archive' WHERE id = ?")
            .bind(message_id)
            .execute(&pool)
            .await
            .expect("Failed to archive message");

        // Verify message is now in Archive
        let folder: (String,) = sqlx::query_as("SELECT folder FROM messages WHERE id = ?")
            .bind(message_id)
            .fetch_one(&pool)
            .await
            .expect("Failed to get folder");

        assert_eq!(folder.0, "Archive", "Message should be in Archive");
    }
}
