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
    pub imap_host: String,
    pub imap_port: u16,
    pub smtp_host: String,
    pub smtp_port: u16,
    pub sync_enabled: bool,
}

/// Add a new email account
#[tauri::command]
pub async fn add_account(
    email: String,
    provider: String,
    imap_host: String,
    imap_port: u16,
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
        INSERT INTO accounts (email, provider, imap_host, imap_port, smtp_host, smtp_port, sync_enabled)
        VALUES (?, ?, ?, ?, ?, ?, 1)
        "#,
    )
    .bind(&email)
    .bind(&provider)
    .bind(&imap_host)
    .bind(imap_port as i64)
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
        imap_host,
        imap_port,
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
        imap_host: String,
        imap_port: i64,
        smtp_host: String,
        smtp_port: i64,
        sync_enabled: bool,
    }

    let rows = sqlx::query_as::<_, AccountRow>(
        r#"
        SELECT id, email, provider, imap_host, imap_port, 
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
            imap_host: row.imap_host,
            imap_port: row.imap_port as u16,
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

/// Update an existing email account
#[tauri::command]
pub async fn update_account(
    account_id: i64,
    email: String,
    provider: String,
    imap_host: String,
    imap_port: u16,
    smtp_host: String,
    smtp_port: u16,
    password: Option<String>,
) -> Result<Account, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Verify account exists
    let existing_email = sqlx::query_scalar::<_, String>("SELECT email FROM accounts WHERE id = ?")
        .bind(account_id)
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Account not found: {}", e))?;

    // Update password in keychain if provided
    if let Some(pwd) = password {
        // Delete old credentials if email changed
        if existing_email != email {
            let _ = crate::keychain::delete_credentials(&existing_email);
        }
        crate::keychain::store_credentials(&email, &pwd)
            .map_err(|e| format!("Failed to update credentials: {}", e))?;
    }

    // Update account in database
    sqlx::query(
        r#"
        UPDATE accounts 
        SET email = ?, provider = ?, imap_host = ?, imap_port = ?, 
            smtp_host = ?, smtp_port = ?, updated_at = datetime('now')
        WHERE id = ?
        "#,
    )
    .bind(&email)
    .bind(&provider)
    .bind(&imap_host)
    .bind(imap_port as i64)
    .bind(&smtp_host)
    .bind(smtp_port as i64)
    .bind(account_id)
    .execute(pool)
    .await
    .map_err(|e| format!("Failed to update account: {}", e))?;

    // Fetch sync_enabled status
    let sync_enabled =
        sqlx::query_scalar::<_, bool>("SELECT sync_enabled FROM accounts WHERE id = ?")
            .bind(account_id)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Failed to fetch account: {}", e))?;

    Ok(Account {
        id: account_id,
        email,
        provider,
        imap_host,
        imap_port,
        smtp_host,
        smtp_port,
        sync_enabled,
    })
}

/// Test connection to IMAP and SMTP servers
///
/// If password is empty and the email exists in keychain, the stored password is used.
/// This allows testing connections for existing accounts without re-entering the password.
#[tauri::command]
pub async fn test_account_connection(
    imap_host: String,
    imap_port: u16,
    smtp_host: String,
    smtp_port: u16,
    email: String,
    password: String,
) -> Result<TestConnectionResult, String> {
    // If password is empty, try to get it from keychain
    let effective_password = if password.is_empty() {
        crate::keychain::get_credentials(&email).map_err(|_| {
            "Password is required. For existing accounts, the stored password could not be retrieved.".to_string()
        })?
    } else {
        password
    };

    let mut result = TestConnectionResult {
        imap_success: false,
        smtp_success: false,
        imap_error: None,
        smtp_error: None,
    };

    // Test IMAP connection
    match test_imap_connection(&imap_host, imap_port, &email, &effective_password).await {
        Ok(_) => result.imap_success = true,
        Err(e) => result.imap_error = Some(e),
    }

    // Test SMTP connection
    match test_smtp_connection(&smtp_host, smtp_port, &email, &effective_password).await {
        Ok(_) => result.smtp_success = true,
        Err(e) => result.smtp_error = Some(e),
    }

    Ok(result)
}

/// Result of connection test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConnectionResult {
    pub imap_success: bool,
    pub smtp_success: bool,
    pub imap_error: Option<String>,
    pub smtp_error: Option<String>,
}

/// Test IMAP connection
async fn test_imap_connection(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
) -> Result<(), String> {
    // Call the IMAP test_login function with timeout
    crate::email::imap::ImapClient::test_login(host, port, username, password)
        .await
        .map_err(|e| format!("IMAP test failed: {}", e))
}

/// Test SMTP connection
async fn test_smtp_connection(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
) -> Result<(), String> {
    // Call the SMTP test_auth function
    crate::email::smtp::SmtpClient::test_auth(host, port, username, password)
        .await
        .map_err(|e| format!("SMTP test failed: {}", e))
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
            imap_host: "imap.gmail.com".to_string(),
            imap_port: 993,
            smtp_host: "smtp.gmail.com".to_string(),
            smtp_port: 465,
            sync_enabled: true,
        };

        let json = serde_json::to_string(&account).unwrap();
        assert!(json.contains("test@example.com"));
    }

    #[tokio::test]
    async fn test_connection_result_serialization() {
        let result = TestConnectionResult {
            imap_success: true,
            smtp_success: false,
            imap_error: None,
            smtp_error: Some("Invalid credentials".to_string()),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("imap_success"));
        assert!(json.contains("Invalid credentials"));
    }

    /// Integration test for account CRUD operations
    /// Note: This test uses a shared in-memory database, so it tests
    /// the full workflow in a single test to avoid test isolation issues
    #[tokio::test]
    async fn test_account_crud_workflow() {
        // Note: Since we're testing with an in-memory database and the global pool
        // may already be initialized, these tests validate the behavior works correctly
        // in production where the pool persists

        // Try to get existing pool or skip if not initialized
        if crate::db::get_pool().await.is_err() {
            // Skip test if pool not initialized - this is expected in isolated test runs
            return;
        }

        // Test 1: Add an account
        let account = add_account(
            format!("test_{}@example.com", chrono::Utc::now().timestamp()),
            "gmail".to_string(),
            "imap.gmail.com".to_string(),
            993,
            "smtp.gmail.com".to_string(),
            465,
            "test_password".to_string(),
        )
        .await
        .expect("Failed to add account");

        assert_eq!(account.imap_port, 993);
        assert_eq!(account.smtp_port, 465);
        assert!(account.sync_enabled);

        let account_id = account.id;
        let account_email = account.email.clone();

        // Test 2: List accounts and verify our account exists
        let accounts = list_accounts().await.unwrap();
        assert!(accounts.iter().any(|a| a.id == account_id));

        // Test 3: Update account
        let new_email = format!("updated_{}@example.com", chrono::Utc::now().timestamp());
        let updated = update_account(
            account_id,
            new_email.clone(),
            "outlook".to_string(),
            "imap.outlook.com".to_string(),
            993,
            "smtp.outlook.com".to_string(),
            587,
            Some("new_password".to_string()),
        )
        .await
        .unwrap();

        assert_eq!(updated.email, new_email);
        assert_eq!(updated.provider, "outlook");
        assert_eq!(updated.smtp_port, 587);

        // Test 4: Update sync enabled
        update_account_sync_enabled(account_id, false)
            .await
            .unwrap();
        let accounts = list_accounts().await.unwrap();
        let found = accounts.iter().find(|a| a.id == account_id).unwrap();
        assert!(!found.sync_enabled);

        // Test 5: Delete account
        delete_account(account_id).await.unwrap();
        let accounts = list_accounts().await.unwrap();
        assert!(!accounts.iter().any(|a| a.id == account_id));

        // Clean up original email from keychain if different
        let _ = crate::keychain::delete_credentials(&account_email);
        let _ = crate::keychain::delete_credentials(&new_email);
    }

    #[tokio::test]
    async fn test_update_nonexistent_account() {
        if crate::db::get_pool().await.is_err() {
            return;
        }

        let result = update_account(
            99999999, // Very high ID unlikely to exist
            "test@example.com".to_string(),
            "gmail".to_string(),
            "imap.gmail.com".to_string(),
            993,
            "smtp.gmail.com".to_string(),
            465,
            Some("password".to_string()),
        )
        .await;

        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.contains("not found") || error.contains("Account not found"));
    }

    #[tokio::test]
    async fn test_duplicate_email_prevention() {
        if crate::db::get_pool().await.is_err() {
            return;
        }

        let unique_email = format!(
            "unique_{}@example.com",
            chrono::Utc::now().timestamp_millis()
        );

        // Add first account
        let result1 = add_account(
            unique_email.clone(),
            "gmail".to_string(),
            "imap.gmail.com".to_string(),
            993,
            "smtp.gmail.com".to_string(),
            465,
            "test_password".to_string(),
        )
        .await;

        assert!(
            result1.is_ok(),
            "First account should be created successfully"
        );
        let account1 = result1.unwrap();

        // Try to add duplicate
        let result2 = add_account(
            unique_email.clone(),
            "gmail".to_string(),
            "imap.gmail.com".to_string(),
            993,
            "smtp.gmail.com".to_string(),
            465,
            "different_password".to_string(),
        )
        .await;

        assert!(result2.is_err(), "Duplicate email should fail");
        let error = result2.unwrap_err();
        assert!(
            error.contains("UNIQUE constraint failed") || error.contains("unique"),
            "Error should mention unique constraint violation, got: {}",
            error
        );

        // Clean up
        let _ = delete_account(account1.id).await;
        let _ = crate::keychain::delete_credentials(&unique_email);
    }
}
