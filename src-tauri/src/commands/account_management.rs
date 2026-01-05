//! Account Management Commands
//!
//! Tauri commands for managing email accounts

use serde::{Deserialize, Serialize};

/// Account data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: i64,
    pub email: String,
    pub provider: String,
    pub pop3_host: String,
    pub pop3_port: u16,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub sync_enabled: bool,
}

/// Add a new email account
#[tauri::command]
pub async fn add_account(
    email: String,
    provider: String,
    pop3_host: String,
    pop3_port: u16,
    smtp_host: String,
    smtp_port: u16,
    password: String,
) -> Result<Account, String> {
    // Store credentials in keychain
    crate::keychain::store_credentials(&email, &password)
        .map_err(|e| format!("Failed to store credentials: {}", e))?;

    // Add account to database
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    let result = sqlx::query(
        r#"
        INSERT INTO accounts (email, provider, pop3_host, pop3_port, smtp_host, smtp_port, sync_enabled)
        VALUES (?, ?, ?, ?, ?, ?, 1)
        "#,
    )
    .bind(&email)
    .bind(&provider)
    .bind(&pop3_host)
    .bind(pop3_port as i64)
    .bind(&smtp_host)
    .bind(smtp_port as i64)
    .execute(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    let account_id = result.last_insert_rowid();

    // Initialize sync state for new account
    sqlx::query(
        r#"
        INSERT INTO sync_state (account_id, sync_status)
        VALUES (?, 'idle')
        "#,
    )
    .bind(account_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to initialize sync state: {}", e))?;

    Ok(Account {
        id: account_id,
        email,
        provider,
        pop3_host,
        pop3_port,
        smtp_host,
        smtp_port,
        sync_enabled: true,
    })
}

/// List all email accounts
#[tauri::command]
pub async fn list_accounts() -> Result<Vec<Account>, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    #[derive(sqlx::FromRow)]
    struct AccountRow {
        id: i64,
        email: String,
        provider: String,
        pop3_host: String,
        pop3_port: i64,
        smtp_host: String,
        smtp_port: i64,
        sync_enabled: bool,
    }

    let rows = sqlx::query_as::<_, AccountRow>(
        r#"
        SELECT id, email, provider, pop3_host, pop3_port, 
               smtp_host, smtp_port, sync_enabled
        FROM accounts
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| format!("Database error: {}", e))?;

    Ok(rows
        .into_iter()
        .map(|row| Account {
            id: row.id,
            email: row.email,
            provider: row.provider,
            pop3_host: row.pop3_host,
            pop3_port: row.pop3_port as u16,
            smtp_host: row.smtp_host,
            smtp_port: row.smtp_port as u16,
            sync_enabled: row.sync_enabled,
        })
        .collect())
}

/// Delete an email account
#[tauri::command]
pub async fn delete_account(account_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Get email before deleting
    let email = sqlx::query_scalar::<_, String>("SELECT email FROM accounts WHERE id = ?")
        .bind(account_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Account not found: {}", e))?;

    // Delete account (will cascade to related records)
    sqlx::query("DELETE FROM accounts WHERE id = ?")
        .bind(account_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to delete account: {}", e))?;

    // Delete credentials from keychain
    let _ = crate::keychain::delete_credentials(&email);

    Ok(())
}

/// Update account sync enabled status
#[tauri::command]
pub async fn update_account_sync_enabled(
    account_id: i64,
    sync_enabled: bool,
) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    sqlx::query("UPDATE accounts SET sync_enabled = ? WHERE id = ?")
        .bind(sync_enabled)
        .bind(account_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to update account: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_account_serialization() {
        let account = Account {
            id: 1,
            email: "test@example.com".to_string(),
            provider: "gmail".to_string(),
            pop3_host: "pop.gmail.com".to_string(),
            pop3_port: 995,
            smtp_host: "smtp.gmail.com".to_string(),
            smtp_port: 465,
            sync_enabled: true,
        };

        let json = serde_json::to_string(&account).unwrap();
        assert!(json.contains("test@example.com"));
    }
}
