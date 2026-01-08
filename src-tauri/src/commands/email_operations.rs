//! Email Operations Commands
//!
//! Tauri commands for email operations (send, mark read/unread, star, delete)
//! Includes bidirectional sync with IMAP server for flag updates and message moves

use crate::email::imap::{FlagAction, ImapClient};
use serde::Deserialize;

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
        // Get message info for IMAP sync
        let message_info: Option<(i64, String, i64)> =
            sqlx::query_as("SELECT account_id, folder, imap_uid FROM messages WHERE id = ?")
                .bind(message_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| format!("Failed to get message info: {}", e))?;

        // Update local database
        sqlx::query("UPDATE messages SET is_read = 1 WHERE id = ?")
            .bind(message_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to mark message as read: {}", e))?;

        // Sync to server in background (non-blocking)
        if let Some((account_id, folder, imap_uid)) = message_info {
            if imap_uid > 0 {
                tokio::spawn(async move {
                    if let Err(e) =
                        sync_flag_to_server(account_id, &folder, imap_uid as u32, "\\Seen", true)
                            .await
                    {
                        eprintln!("Failed to sync read flag to server: {}", e);
                    }
                });
            }
        }
    }

    Ok(())
}

/// Mark messages as unread
#[tauri::command]
pub async fn mark_unread(message_ids: Vec<i64>) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    for message_id in message_ids {
        // Get message info for IMAP sync
        let message_info: Option<(i64, String, i64)> =
            sqlx::query_as("SELECT account_id, folder, imap_uid FROM messages WHERE id = ?")
                .bind(message_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| format!("Failed to get message info: {}", e))?;

        // Update local database
        sqlx::query("UPDATE messages SET is_read = 0 WHERE id = ?")
            .bind(message_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to mark message as unread: {}", e))?;

        // Sync to server in background (non-blocking)
        if let Some((account_id, folder, imap_uid)) = message_info {
            if imap_uid > 0 {
                tokio::spawn(async move {
                    if let Err(e) =
                        sync_flag_to_server(account_id, &folder, imap_uid as u32, "\\Seen", false)
                            .await
                    {
                        eprintln!("Failed to sync unread flag to server: {}", e);
                    }
                });
            }
        }
    }

    Ok(())
}

/// Star a message
#[tauri::command]
pub async fn star_message(message_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Get message info for IMAP sync
    let message_info: Option<(i64, String, i64)> =
        sqlx::query_as("SELECT account_id, folder, imap_uid FROM messages WHERE id = ?")
            .bind(message_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Failed to get message info: {}", e))?;

    // Update local database
    sqlx::query("UPDATE messages SET is_starred = 1 WHERE id = ?")
        .bind(message_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to star message: {}", e))?;

    // Sync to server in background (non-blocking)
    if let Some((account_id, folder, imap_uid)) = message_info {
        if imap_uid > 0 {
            tokio::spawn(async move {
                if let Err(e) =
                    sync_flag_to_server(account_id, &folder, imap_uid as u32, "\\Flagged", true)
                        .await
                {
                    eprintln!("Failed to sync star flag to server: {}", e);
                }
            });
        }
    }

    Ok(())
}

/// Unstar a message
#[tauri::command]
pub async fn unstar_message(message_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Get message info for IMAP sync
    let message_info: Option<(i64, String, i64)> =
        sqlx::query_as("SELECT account_id, folder, imap_uid FROM messages WHERE id = ?")
            .bind(message_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Failed to get message info: {}", e))?;

    // Update local database
    sqlx::query("UPDATE messages SET is_starred = 0 WHERE id = ?")
        .bind(message_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to unstar message: {}", e))?;

    // Sync to server in background (non-blocking)
    if let Some((account_id, folder, imap_uid)) = message_info {
        if imap_uid > 0 {
            tokio::spawn(async move {
                if let Err(e) =
                    sync_flag_to_server(account_id, &folder, imap_uid as u32, "\\Flagged", false)
                        .await
                {
                    eprintln!("Failed to sync unstar flag to server: {}", e);
                }
            });
        }
    }

    Ok(())
}

/// Delete a message
#[tauri::command]
pub async fn delete_message(message_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Get message info for IMAP sync
    let message_info: Option<(i64, String, i64)> =
        sqlx::query_as("SELECT account_id, folder, imap_uid FROM messages WHERE id = ?")
            .bind(message_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Failed to get message info: {}", e))?;

    // Delete from local database
    sqlx::query("DELETE FROM messages WHERE id = ?")
        .bind(message_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete message: {}", e))?;

    // Sync to server in background (non-blocking)
    if let Some((account_id, folder, imap_uid)) = message_info {
        if imap_uid > 0 {
            tokio::spawn(async move {
                if let Err(e) = delete_message_on_server(account_id, &folder, imap_uid as u32).await
                {
                    eprintln!("Failed to delete message on server: {}", e);
                }
            });
        }
    }

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
        // Get message info for IMAP sync
        let message_info: Option<(i64, String, i64)> =
            sqlx::query_as("SELECT account_id, folder, imap_uid FROM messages WHERE id = ?")
                .bind(message_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| format!("Failed to get message info: {}", e))?;

        // Delete from local database
        sqlx::query("DELETE FROM messages WHERE id = ?")
            .bind(message_id)
            .execute(pool)
            .await
            .map_err(|e| format!("Failed to delete message: {}", e))?;

        // Sync to server in background
        if let Some((account_id, folder, imap_uid)) = message_info {
            if imap_uid > 0 {
                tokio::spawn(async move {
                    if let Err(e) =
                        delete_message_on_server(account_id, &folder, imap_uid as u32).await
                    {
                        eprintln!("Failed to delete message on server: {}", e);
                    }
                });
            }
        }
    }

    Ok(())
}

/// Move messages between folders
#[tauri::command]
pub async fn move_messages(
    message_ids: Vec<i64>,
    destination_folder: String,
) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    for message_id in message_ids {
        // Get message info for IMAP sync
        let message_info: Option<(i64, String, i64)> =
            sqlx::query_as("SELECT account_id, folder, imap_uid FROM messages WHERE id = ?")
                .bind(message_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| format!("Failed to get message info: {}", e))?;

        if let Some((account_id, source_folder, imap_uid)) = message_info {
            // Update local database first
            sqlx::query("UPDATE messages SET folder = ? WHERE id = ?")
                .bind(&destination_folder)
                .bind(message_id)
                .execute(pool)
                .await
                .map_err(|e| format!("Failed to move message: {}", e))?;

            // Sync to server in background (non-blocking)
            if imap_uid > 0 {
                let dest = destination_folder.clone();
                tokio::spawn(async move {
                    if let Err(e) =
                        move_message_on_server(account_id, &source_folder, &dest, imap_uid as u32)
                            .await
                    {
                        eprintln!("Failed to move message on server: {}", e);
                    }
                });
            }
        }
    }

    Ok(())
}

/// Save a draft (saves locally and appends to server's Drafts folder)
#[tauri::command]
pub async fn save_draft(
    account_id: i64,
    subject: String,
    body_plain: String,
    body_html: Option<String>,
    to: Vec<String>,
    cc: Vec<String>,
    bcc: Vec<String>,
) -> Result<i64, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Create message ID for draft
    let message_id = format!("draft-{}-{}", account_id, chrono::Utc::now().timestamp());

    // Store draft in local database
    let draft_id = sqlx::query(
        r#"
        INSERT INTO messages (account_id, message_id, folder, subject, body_plain, body_html, to_addr, cc_addr, bcc_addr, from_addr)
        VALUES (?, ?, 'Drafts', ?, ?, ?, ?, ?, ?, '')
        "#,
    )
    .bind(account_id)
    .bind(&message_id)
    .bind(&subject)
    .bind(&body_plain)
    .bind(&body_html)
    .bind(to.join(", "))
    .bind(cc.join(", "))
    .bind(bcc.join(", "))
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to save draft: {}", e))?
    .last_insert_rowid();

    // Append to server's Drafts folder in background
    tokio::spawn(async move {
        if let Err(e) = append_draft_to_server(
            account_id,
            &subject,
            &body_plain,
            body_html.as_deref(),
            &to,
            &cc,
            &bcc,
        )
        .await
        {
            eprintln!("Failed to append draft to server: {}", e);
        }
    });

    Ok(draft_id)
}

/// Delete draft (from local and server)
#[tauri::command]
pub async fn delete_draft(message_id: i64) -> Result<(), String> {
    // For drafts, we delete from both local and server
    // This reuses delete_message which already handles server deletion
    delete_message(message_id).await
}

/// Helper: Sync a flag to the IMAP server
async fn sync_flag_to_server(
    account_id: i64,
    folder: &str,
    uid: u32,
    flag: &str,
    add: bool,
) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Get account credentials
    let account: Option<(String, String, i64)> =
        sqlx::query_as("SELECT email, imap_host, imap_port FROM accounts WHERE id = ?")
            .bind(account_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Failed to get account: {}", e))?;

    let (email, imap_host, imap_port) = account.ok_or("Account not found")?;

    // Get password from keychain
    let password = crate::keychain::get_credentials(&email)
        .map_err(|_| "Failed to get credentials from keychain")?;

    // Connect to IMAP server
    let mut client = ImapClient::connect(&imap_host, imap_port as u16, &email, &password)
        .await
        .map_err(|e| format!("Failed to connect to IMAP: {}", e))?;

    // Select folder
    client
        .select_folder(folder)
        .await
        .map_err(|e| format!("Failed to select folder: {}", e))?;

    // Set flag
    let action = if add {
        FlagAction::Add
    } else {
        FlagAction::Remove
    };
    client
        .set_flags(uid, &[flag], action)
        .await
        .map_err(|e| format!("Failed to set flag: {}", e))?;

    Ok(())
}

/// Helper: Delete a message on the IMAP server
async fn delete_message_on_server(account_id: i64, folder: &str, uid: u32) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Get account credentials
    let account: Option<(String, String, i64)> =
        sqlx::query_as("SELECT email, imap_host, imap_port FROM accounts WHERE id = ?")
            .bind(account_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Failed to get account: {}", e))?;

    let (email, imap_host, imap_port) = account.ok_or("Account not found")?;

    // Get password from keychain
    let password = crate::keychain::get_credentials(&email)
        .map_err(|_| "Failed to get credentials from keychain")?;

    // Connect to IMAP server
    let mut client = ImapClient::connect(&imap_host, imap_port as u16, &email, &password)
        .await
        .map_err(|e| format!("Failed to connect to IMAP: {}", e))?;

    // Select folder
    client
        .select_folder(folder)
        .await
        .map_err(|e| format!("Failed to select folder: {}", e))?;

    // Mark as deleted
    client
        .set_flags(uid, &["\\Deleted"], FlagAction::Add)
        .await
        .map_err(|e| format!("Failed to mark as deleted: {}", e))?;

    // Expunge
    client
        .expunge()
        .await
        .map_err(|e| format!("Failed to expunge: {}", e))?;

    Ok(())
}

/// Helper: Move a message on the IMAP server using COPY + DELETE
async fn move_message_on_server(
    account_id: i64,
    source_folder: &str,
    dest_folder: &str,
    uid: u32,
) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Get account credentials
    let account: Option<(String, String, i64)> =
        sqlx::query_as("SELECT email, imap_host, imap_port FROM accounts WHERE id = ?")
            .bind(account_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Failed to get account: {}", e))?;

    let (email, imap_host, imap_port) = account.ok_or("Account not found")?;

    // Get password from keychain
    let password = crate::keychain::get_credentials(&email)
        .map_err(|_| "Failed to get credentials from keychain")?;

    // Connect to IMAP server
    let mut client = ImapClient::connect(&imap_host, imap_port as u16, &email, &password)
        .await
        .map_err(|e| format!("Failed to connect to IMAP: {}", e))?;

    // Select source folder
    client
        .select_folder(source_folder)
        .await
        .map_err(|e| format!("Failed to select source folder: {}", e))?;

    // Copy message to destination folder
    client
        .copy_message(uid, dest_folder)
        .await
        .map_err(|e| format!("Failed to copy message: {}", e))?;

    // Mark original as deleted
    client
        .set_flags(uid, &["\\Deleted"], FlagAction::Add)
        .await
        .map_err(|e| format!("Failed to mark as deleted: {}", e))?;

    // Expunge
    client
        .expunge()
        .await
        .map_err(|e| format!("Failed to expunge: {}", e))?;

    Ok(())
}

/// Helper: Append draft to server's Drafts folder
async fn append_draft_to_server(
    account_id: i64,
    subject: &str,
    body_plain: &str,
    body_html: Option<&str>,
    to: &[String],
    cc: &[String],
    bcc: &[String],
) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Get account credentials
    let account: Option<(String, String, i64)> =
        sqlx::query_as("SELECT email, imap_host, imap_port FROM accounts WHERE id = ?")
            .bind(account_id)
            .fetch_optional(pool)
            .await
            .map_err(|e| format!("Failed to get account: {}", e))?;

    let (email, imap_host, imap_port) = account.ok_or("Account not found")?;

    // Get password from keychain
    let password = crate::keychain::get_credentials(&email)
        .map_err(|_| "Failed to get credentials from keychain")?;

    // Connect to IMAP server
    let mut client = ImapClient::connect(&imap_host, imap_port as u16, &email, &password)
        .await
        .map_err(|e| format!("Failed to connect to IMAP: {}", e))?;

    // Detect Drafts folder
    let folders = client
        .list_folders()
        .await
        .map_err(|e| format!("Failed to list folders: {}", e))?;

    let drafts_folder = folders
        .iter()
        .find(|f| {
            let name_lower = f.name.to_lowercase();
            name_lower.contains("draft")
        })
        .map(|f| f.name.clone())
        .unwrap_or_else(|| "Drafts".to_string());

    // Build draft message as RFC822 format
    let mut message = format!("From: {}\r\n", email);
    if !to.is_empty() {
        message.push_str(&format!("To: {}\r\n", to.join(", ")));
    }
    if !cc.is_empty() {
        message.push_str(&format!("Cc: {}\r\n", cc.join(", ")));
    }
    if !bcc.is_empty() {
        message.push_str(&format!("Bcc: {}\r\n", bcc.join(", ")));
    }
    message.push_str(&format!("Subject: {}\r\n\r\n", subject));

    if let Some(html) = body_html {
        message.push_str(&format!("{}\r\n\r\n{}", body_plain, html));
    } else {
        message.push_str(body_plain);
    }

    // Append to Drafts folder
    client
        .append_message(&drafts_folder, message.as_bytes())
        .await
        .map_err(|e| format!("Failed to append draft: {}", e))?;

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
