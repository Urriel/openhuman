//! Email List Commands
//!
//! Tauri commands for fetching and filtering email messages for the list view

use serde::{Deserialize, Serialize};

/// Message list item for display in email list
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageListItem {
    pub id: i64,
    pub subject: Option<String>,
    pub from_addr: String,
    pub preview: String,
    pub date: Option<String>,
    pub is_read: bool,
    pub is_starred: bool,
    pub has_attachments: bool,
}

/// List messages with optional filters
#[tauri::command]
pub async fn list_messages(
    account_id: Option<i64>,
    folder: Option<String>,
    label_id: Option<i64>,
    is_read: Option<bool>,
    is_starred: Option<bool>,
    limit: Option<i64>,
) -> Result<Vec<MessageListItem>, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;
    let limit = limit.unwrap_or(10000);

    // Build dynamic query based on filters
    let mut query = String::from(
        r#"
        SELECT 
            m.id,
            m.subject,
            m.from_addr,
            COALESCE(SUBSTR(m.body_plain, 1, 100), '') as preview,
            m.date,
            m.is_read,
            m.is_starred,
            (SELECT COUNT(*) FROM attachments WHERE message_id = m.id) > 0 as has_attachments
        FROM messages m
        WHERE 1=1
        "#,
    );

    let mut bindings: Vec<String> = Vec::new();

    if let Some(acc_id) = account_id {
        query.push_str(" AND m.account_id = ?");
        bindings.push(acc_id.to_string());
    }

    if let Some(fld) = folder {
        query.push_str(" AND m.folder = ?");
        bindings.push(fld);
    }

    if let Some(lbl_id) = label_id {
        query.push_str(" AND m.id IN (SELECT message_id FROM message_labels WHERE label_id = ?)");
        bindings.push(lbl_id.to_string());
    }

    if let Some(read) = is_read {
        query.push_str(" AND m.is_read = ?");
        bindings.push(if read { "1" } else { "0" }.to_string());
    }

    if let Some(starred) = is_starred {
        query.push_str(" AND m.is_starred = ?");
        bindings.push(if starred { "1" } else { "0" }.to_string());
    }

    query.push_str(" ORDER BY m.date DESC LIMIT ?");
    bindings.push(limit.to_string());

    // Execute query with dynamic bindings
    let mut query_builder = sqlx::query_as::<
        _,
        (
            i64,
            Option<String>,
            String,
            String,
            Option<String>,
            i64,
            i64,
            i64,
        ),
    >(&query);

    for binding in &bindings {
        query_builder = query_builder.bind(binding);
    }

    let rows = query_builder
        .fetch_all(pool)
        .await
        .map_err(|e| format!("Failed to list messages: {}", e))?;

    let messages = rows
        .into_iter()
        .map(
            |(id, subject, from_addr, preview, date, is_read, is_starred, has_attachments)| {
                MessageListItem {
                    id,
                    subject,
                    from_addr,
                    preview,
                    date,
                    is_read: is_read == 1,
                    is_starred: is_starred == 1,
                    has_attachments: has_attachments == 1,
                }
            },
        )
        .collect();

    Ok(messages)
}

/// Get a single message by ID with full details
#[tauri::command]
pub async fn get_message(message_id: i64) -> Result<MessageDetails, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    let row = sqlx::query_as::<
        _,
        (
            i64,
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            Option<String>,
            i64,
            i64,
        ),
    >(
        r#"
        SELECT 
            id, subject, from_addr, to_addr, cc_addr, bcc_addr,
            date, body_plain, body_html, is_read, is_starred
        FROM messages
        WHERE id = ?
        "#,
    )
    .bind(message_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("Failed to get message: {}", e))?
    .ok_or_else(|| "Message not found".to_string())?;

    let (
        id,
        subject,
        from_addr,
        to_addr,
        cc_addr,
        bcc_addr,
        date,
        body_plain,
        body_html,
        is_read,
        is_starred,
    ) = row;

    // Get attachments
    let attachments = sqlx::query_as::<_, (i64, String, i64, Option<String>)>(
        r#"
        SELECT id, filename, size, mime_type
        FROM attachments
        WHERE message_id = ?
        "#,
    )
    .bind(message_id)
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Failed to get attachments: {}", e))?
    .into_iter()
    .map(|(id, filename, size, mime_type)| Attachment {
        id,
        filename,
        size,
        mime_type,
    })
    .collect();

    Ok(MessageDetails {
        id,
        subject,
        from_addr,
        to_addr,
        cc_addr,
        bcc_addr,
        date,
        body_plain,
        body_html,
        is_read: is_read == 1,
        is_starred: is_starred == 1,
        attachments,
    })
}

/// Message details for reader view
#[derive(Debug, Serialize, Deserialize)]
pub struct MessageDetails {
    pub id: i64,
    pub subject: Option<String>,
    pub from_addr: String,
    pub to_addr: Option<String>,
    pub cc_addr: Option<String>,
    pub bcc_addr: Option<String>,
    pub date: Option<String>,
    pub body_plain: Option<String>,
    pub body_html: Option<String>,
    pub is_read: bool,
    pub is_starred: bool,
    pub attachments: Vec<Attachment>,
}

/// Attachment information
#[derive(Debug, Serialize, Deserialize)]
pub struct Attachment {
    pub id: i64,
    pub filename: String,
    pub size: i64,
    pub mime_type: Option<String>,
}

/// Folder information
#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub message_count: i64,
}

/// List all folders with message counts
#[tauri::command]
pub async fn list_folders(account_id: Option<i64>) -> Result<Vec<Folder>, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    let query = if let Some(acc_id) = account_id {
        sqlx::query_as::<_, (i64, i64, String, i64)>(
            r#"
            SELECT 
                f.id,
                f.account_id,
                f.name,
                (SELECT COUNT(*) FROM messages WHERE folder = f.name AND account_id = f.account_id) as message_count
            FROM folders f
            WHERE f.account_id = ?
            ORDER BY f.name
            "#,
        )
        .bind(acc_id)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, (i64, i64, String, i64)>(
            r#"
            SELECT 
                f.id,
                f.account_id,
                f.name,
                (SELECT COUNT(*) FROM messages WHERE folder = f.name AND account_id = f.account_id) as message_count
            FROM folders f
            ORDER BY f.name
            "#,
        )
        .fetch_all(pool)
        .await
    };

    let rows = query.map_err(|e| format!("Failed to list folders: {}", e))?;

    let folders = rows
        .into_iter()
        .map(|(id, account_id, name, message_count)| Folder {
            id,
            account_id,
            name,
            message_count,
        })
        .collect();

    Ok(folders)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_db;

    async fn setup_test_data(pool: &sqlx::SqlitePool) -> (i64, i64) {
        // Create account
        let account_result = sqlx::query(
            "INSERT INTO accounts (email, provider, pop3_host, smtp_host) VALUES (?, ?, ?, ?)",
        )
        .bind("test@example.com")
        .bind("custom")
        .bind("pop.example.com")
        .bind("smtp.example.com")
        .execute(pool)
        .await
        .expect("Failed to create account");
        let account_id = account_result.last_insert_rowid();

        // Create message
        let message_result = sqlx::query(
            r#"
            INSERT INTO messages (account_id, message_id, from_addr, subject, body_plain, folder, is_read)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(account_id)
        .bind("msg-123")
        .bind("sender@example.com")
        .bind("Test Subject")
        .bind("This is a test message body")
        .bind("INBOX")
        .bind(0)
        .execute(pool)
        .await
        .expect("Failed to create message");
        let message_id = message_result.last_insert_rowid();

        (account_id, message_id)
    }

    #[tokio::test]
    async fn test_list_messages() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let (account_id, _) = setup_test_data(&pool).await;

        // Query messages
        let query = r#"
        SELECT 
            m.id,
            m.subject,
            m.from_addr,
            COALESCE(SUBSTR(m.body_plain, 1, 100), '') as preview,
            m.date,
            m.is_read,
            m.is_starred,
            (SELECT COUNT(*) FROM attachments WHERE message_id = m.id) > 0 as has_attachments
        FROM messages m
        WHERE m.account_id = ?
        ORDER BY m.date DESC
        LIMIT 10000
        "#;

        let rows = sqlx::query_as::<
            _,
            (
                i64,
                Option<String>,
                String,
                String,
                Option<String>,
                i64,
                i64,
                i64,
            ),
        >(query)
        .bind(account_id)
        .fetch_all(&pool)
        .await
        .expect("Failed to query messages");

        assert_eq!(rows.len(), 1, "Should have one message");
        let (_, subject, from_addr, preview, _, _, _, _) = &rows[0];
        assert_eq!(subject.as_deref(), Some("Test Subject"));
        assert_eq!(from_addr, "sender@example.com");
        assert!(preview.contains("test message"));
    }

    #[tokio::test]
    async fn test_get_message_details() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let (_, message_id) = setup_test_data(&pool).await;

        // Query message details
        let row = sqlx::query_as::<
            _,
            (
                i64,
                Option<String>,
                String,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                Option<String>,
                i64,
                i64,
            ),
        >(
            r#"
            SELECT 
                id, subject, from_addr, to_addr, cc_addr, bcc_addr,
                date, body_plain, body_html, is_read, is_starred
            FROM messages
            WHERE id = ?
            "#,
        )
        .bind(message_id)
        .fetch_one(&pool)
        .await
        .expect("Failed to get message");

        let (_, subject, from_addr, _, _, _, _, body_plain, _, is_read, _) = row;
        assert_eq!(subject.as_deref(), Some("Test Subject"));
        assert_eq!(from_addr, "sender@example.com");
        assert_eq!(body_plain.as_deref(), Some("This is a test message body"));
        assert_eq!(is_read, 0);
    }

    #[tokio::test]
    async fn test_filter_by_folder() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");
        let (account_id, _) = setup_test_data(&pool).await;

        // Create another message in Archive folder
        sqlx::query(
            r#"
            INSERT INTO messages (account_id, message_id, from_addr, subject, folder)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(account_id)
        .bind("msg-456")
        .bind("sender2@example.com")
        .bind("Archived Message")
        .bind("Archive")
        .execute(&pool)
        .await
        .expect("Failed to create archived message");

        // Query only INBOX messages
        let inbox_messages = sqlx::query_as::<_, (i64, Option<String>)>(
            "SELECT id, subject FROM messages WHERE account_id = ? AND folder = 'INBOX'",
        )
        .bind(account_id)
        .fetch_all(&pool)
        .await
        .expect("Failed to query inbox");

        assert_eq!(inbox_messages.len(), 1);
        assert_eq!(inbox_messages[0].1.as_deref(), Some("Test Subject"));

        // Query only Archive messages
        let archive_messages = sqlx::query_as::<_, (i64, Option<String>)>(
            "SELECT id, subject FROM messages WHERE account_id = ? AND folder = 'Archive'",
        )
        .bind(account_id)
        .fetch_all(&pool)
        .await
        .expect("Failed to query archive");

        assert_eq!(archive_messages.len(), 1);
        assert_eq!(archive_messages[0].1.as_deref(), Some("Archived Message"));
    }
}
