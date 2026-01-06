//! Email Operations Commands
//!
//! Tauri commands for email operations (send, mark read/unread, star, delete)

use serde::{Deserialize, Serialize};

/// Send email parameters
#[derive(Debug, Deserialize)]
pub struct SendEmailParams {
    pub account_id: i64,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub subject: String,
    pub body_plain: String,
    pub body_html: Option<String>,
}

/// Send an email (queues in outbox)
#[tauri::command]
pub async fn send_email(params: SendEmailParams) -> Result<i64, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Serialize recipients to JSON
    let recipients = serde_json::json!({
        "to": params.to,
        "cc": params.cc,
        "bcc": params.bcc,
    });

    let result = sqlx::query(
        r#"
        INSERT INTO outbox (account_id, recipients, subject, body_plain, body_html, send_status)
        VALUES (?, ?, ?, ?, ?, 'pending')
        "#,
    )
    .bind(params.account_id)
    .bind(recipients.to_string())
    .bind(&params.subject)
    .bind(&params.body_plain)
    .bind(&params.body_html)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to queue email: {}", e))?;

    Ok(result.last_insert_rowid())
}

/// Mark messages as read
#[tauri::command]
pub async fn mark_read(message_ids: Vec<i64>) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    for message_id in message_ids {
        sqlx::query("UPDATE messages SET is_read = 1 WHERE id = ?")
            .bind(message_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to mark message as read: {}", e))?;
    }

    Ok(())
}

/// Mark messages as unread
#[tauri::command]
pub async fn mark_unread(message_ids: Vec<i64>) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    for message_id in message_ids {
        sqlx::query("UPDATE messages SET is_read = 0 WHERE id = ?")
            .bind(message_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to mark message as unread: {}", e))?;
    }

    Ok(())
}

/// Star a message
#[tauri::command]
pub async fn star_message(message_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    sqlx::query("UPDATE messages SET is_starred = 1 WHERE id = ?")
        .bind(message_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to star message: {}", e))?;

    Ok(())
}

/// Unstar a message
#[tauri::command]
pub async fn unstar_message(message_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    sqlx::query("UPDATE messages SET is_starred = 0 WHERE id = ?")
        .bind(message_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to unstar message: {}", e))?;

    Ok(())
}

/// Delete a message
#[tauri::command]
pub async fn delete_message(message_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(message_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete message: {}", e))?;

    Ok(())
}

/// Bulk mark messages as read or unread
#[tauri::command]
pub async fn bulk_mark_read(message_ids: Vec<i64>, is_read: bool) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    let read_value = if is_read { 1 } else { 0 };

    for message_id in message_ids {
        sqlx::query("UPDATE messages SET is_read = ? WHERE id = ?")
            .bind(read_value)
            .bind(message_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to update message read status: {}", e))?;
    }

    Ok(())
}

/// Bulk archive messages (move to Archive folder)
#[tauri::command]
pub async fn bulk_archive_messages(message_ids: Vec<i64>) -> Result<(), String> {
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

/// Bulk delete messages
#[tauri::command]
pub async fn bulk_delete_messages(message_ids: Vec<i64>) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    for message_id in message_ids {
        sqlx::query("DELETE FROM messages WHERE id = ?")
            .bind(message_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to delete message: {}", e))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_email_params_deserialization() {
        let json = r#"{
            "account_id": 1,
            "to": ["recipient@example.com"],
            "cc": [],
            "bcc": [],
            "subject": "Test",
            "body_plain": "Test body",
            "body_html": null
        }"#;

        let params: SendEmailParams = serde_json::from_str(json).unwrap();
        assert_eq!(params.account_id, 1);
        assert_eq!(params.to.len(), 1);
        assert_eq!(params.subject, "Test");
    }
}
