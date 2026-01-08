//! Email Sync Commands
//!
//! Tauri commands for managing email synchronization

use serde::{Deserialize, Serialize};

/// Sync statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStats {
    pub messages_synced: i32,
    pub new_messages: i32,
    pub errors: Vec<String>,
}

/// Sync status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncStatusInfo {
    pub account_id: i64,
    pub sync_status: String,
    pub last_sync_at: Option<String>,
    pub messages_fetched: i32,
    pub error_message: Option<String>,
}

/// Sync emails for one or all accounts
#[tauri::command]
pub async fn sync_emails(account_id: Option<i64>) -> Result<SyncStats, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    let stats = if let Some(acc_id) = account_id {
        // Sync single account
        sync_single_account(&pool, acc_id).await?
    } else {
        // Sync all accounts
        sync_all_accounts(&pool).await?
    };

    Ok(stats)
}

/// Start background sync scheduler
#[tauri::command]
pub async fn start_sync_scheduler() -> Result<(), String> {
    use crate::email::sync_orchestrator::SyncOrchestrator;
    use std::sync::Arc;

    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;
    let pool = Arc::new(pool.clone());

    let orchestrator = SyncOrchestrator::new(pool);

    // Spawn background sync task
    tokio::spawn(async move {
        if let Err(e) = orchestrator.start_scheduler().await {
            eprintln!("Sync scheduler error: {}", e);
        }
    });

    Ok(())
}

/// Get sync status for an account
#[tauri::command]
pub async fn get_sync_status(account_id: i64) -> Result<SyncStatusInfo, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    #[derive(sqlx::FromRow)]
    struct StatusRow {
        account_id: i64,
        sync_status: String,
        last_sync_at: Option<String>,
        messages_fetched: i32,
        error_message: Option<String>,
    }

    let row = sqlx::query_as::<_, StatusRow>(
        r#"
        SELECT account_id, sync_status, last_sync_at, 
               messages_fetched, error_message
        FROM sync_state
        WHERE account_id = ?
        "#,
    )
    .bind(account_id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Failed to get sync status: {}", e))?;

    Ok(SyncStatusInfo {
        account_id: row.account_id,
        sync_status: row.sync_status,
        last_sync_at: row.last_sync_at,
        messages_fetched: row.messages_fetched,
        error_message: row.error_message,
    })
}

/// Cancel ongoing sync for an account
#[tauri::command]
pub async fn cancel_sync(account_id: i64) -> Result<(), String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    // Update sync status to cancelled
    sqlx::query("UPDATE sync_state SET sync_status = 'cancelled' WHERE account_id = ?")
        .bind(account_id)
        .execute(pool)
        .await
        .map_err(|e| format!("Failed to cancel sync: {}", e))?;

    Ok(())
}

/// Helper: Sync a single account
async fn sync_single_account(
    pool: &sqlx::SqlitePool,
    account_id: i64,
) -> Result<SyncStats, String> {
    use crate::email::sync_orchestrator::SyncOrchestrator;
    use std::sync::Arc;

    let pool = Arc::new(pool.clone());
    let orchestrator = SyncOrchestrator::new(pool);

    let sync_stats = orchestrator
        .sync_account(account_id)
        .await
        .map_err(|e| e.to_string())?;

    Ok(SyncStats {
        messages_synced: sync_stats.messages_synced,
        new_messages: sync_stats.new_messages,
        errors: sync_stats.errors,
    })
}

/// Helper: Sync all enabled accounts
async fn sync_all_accounts(pool: &sqlx::SqlitePool) -> Result<SyncStats, String> {
    use crate::email::sync_orchestrator::SyncOrchestrator;
    use std::sync::Arc;

    let pool = Arc::new(pool.clone());
    let orchestrator = SyncOrchestrator::new(pool);

    let all_stats = orchestrator
        .sync_all_accounts()
        .await
        .map_err(|e| e.to_string())?;

    // Aggregate stats from all accounts
    let mut total = SyncStats {
        messages_synced: 0,
        new_messages: 0,
        errors: vec![],
    };

    for stats in all_stats {
        total.messages_synced += stats.messages_synced;
        total.new_messages += stats.new_messages;
        total.errors.extend(stats.errors);
    }

    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_stats_serialization() {
        let stats = SyncStats {
            messages_synced: 10,
            new_messages: 5,
            errors: vec![],
        };

        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("10"));
    }

    #[test]
    fn test_sync_status_info_serialization() {
        let status = SyncStatusInfo {
            account_id: 1,
            sync_status: "completed".to_string(),
            last_sync_at: Some("2024-01-01T12:00:00Z".to_string()),
            messages_fetched: 100,
            error_message: None,
        };

        let json = serde_json::to_string(&status).unwrap();
        assert!(json.contains("completed"));
    }
}
