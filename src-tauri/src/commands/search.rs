//! Search Commands
//!
//! Tauri commands for searching messages using FTS5

use serde::{Deserialize, Serialize};

/// Message search result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageSearchResult {
    pub id: i64,
    pub account_id: i64,
    pub message_id: String,
    pub subject: Option<String>,
    pub from_addr: String,
    pub to_addr: Option<String>,
    pub date: Option<String>,
    pub is_read: bool,
    pub is_starred: bool,
    pub snippet: String,
}

/// Search messages using FTS5
#[tauri::command]
pub async fn search_messages(
    query: String,
    account_id: Option<i64>,
) -> Result<Vec<MessageSearchResult>, String> {
    let pool = crate::db::get_pool().await.map_err(|e| e.to_string())?;

    #[derive(sqlx::FromRow)]
    struct SearchRow {
        id: i64,
        account_id: i64,
        message_id: String,
        subject: Option<String>,
        from_addr: String,
        to_addr: Option<String>,
        date: Option<String>,
        is_read: bool,
        is_starred: bool,
        snippet: String,
    }

    // Build query based on whether account filter is provided
    let rows: Vec<SearchRow> = if let Some(acc_id) = account_id {
        sqlx::query_as::<_, SearchRow>(
            r#"
            SELECT m.id, m.account_id, m.message_id, m.subject, m.from_addr, 
                   m.to_addr, m.date, m.is_read, m.is_starred,
                   snippet(message_fts, 1, '<mark>', '</mark>', '...', 32) as snippet
            FROM messages m
            INNER JOIN message_fts ON message_fts.rowid = m.id
            WHERE message_fts MATCH ? AND m.account_id = ?
            ORDER BY rank
            LIMIT 100
            "#,
        )
        .bind(&query)
        .bind(acc_id)
        .fetch_all(pool)
        .await
    } else {
        sqlx::query_as::<_, SearchRow>(
            r#"
            SELECT m.id, m.account_id, m.message_id, m.subject, m.from_addr, 
                   m.to_addr, m.date, m.is_read, m.is_starred,
                   snippet(message_fts, 1, '<mark>', '</mark>', '...', 32) as snippet
            FROM messages m
            INNER JOIN message_fts ON message_fts.rowid = m.id
            WHERE message_fts MATCH ?
            ORDER BY rank
            LIMIT 100
            "#,
        )
        .bind(&query)
        .fetch_all(pool)
        .await
    }
    .map_err(|e| format!("Search failed: {}", e))?;

    Ok(rows
        .into_iter()
        .map(|row| MessageSearchResult {
            id: row.id,
            account_id: row.account_id,
            message_id: row.message_id,
            subject: row.subject,
            from_addr: row.from_addr,
            to_addr: row.to_addr,
            date: row.date,
            is_read: row.is_read,
            is_starred: row.is_starred,
            snippet: row.snippet,
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_search_result_serialization() {
        let result = MessageSearchResult {
            id: 1,
            account_id: 1,
            message_id: "<msg1@example.com>".to_string(),
            subject: Some("Test Subject".to_string()),
            from_addr: "sender@example.com".to_string(),
            to_addr: Some("recipient@example.com".to_string()),
            date: Some("2024-01-01".to_string()),
            is_read: false,
            is_starred: false,
            snippet: "This is a <mark>test</mark> message".to_string(),
        };

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("test"));
    }
}
