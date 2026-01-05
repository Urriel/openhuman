//! Database initialization and connection management
//!
//! This module handles SQLite database setup, migrations, and connection pooling.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::ConnectOptions;
use std::str::FromStr;
use std::sync::OnceLock;
use std::time::Duration;

use crate::error::{DatabaseError, DbResult};

/// Global database pool
static DB_POOL: OnceLock<SqlitePool> = OnceLock::new();

/// Get the global database pool
///
/// Returns an error if the pool hasn't been initialized yet
pub async fn get_pool() -> DbResult<&'static SqlitePool> {
    DB_POOL
        .get()
        .ok_or_else(|| DatabaseError::ConnectionError("Database not initialized".to_string()))
}

/// Initialize and store the global database pool
pub fn set_pool(pool: SqlitePool) -> DbResult<()> {
    DB_POOL
        .set(pool)
        .map_err(|_| DatabaseError::ConnectionError("Database already initialized".to_string()))
}

/// Initialize the database connection pool and run migrations
///
/// This function:
/// 1. Creates the database file if it doesn't exist
/// 2. Establishes a connection pool
/// 3. Runs all pending migrations
/// 4. Returns the connection pool for use throughout the application
///
/// # Arguments
///
/// * `database_url` - Path to the SQLite database file (e.g., "sqlite://data/openhuman.db")
///
/// # Returns
///
/// A connection pool ready for database operations
pub async fn init_db(database_url: &str) -> DbResult<SqlitePool> {
    // Configure SQLite connection options
    let options = SqliteConnectOptions::from_str(database_url)
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?
        .create_if_missing(true)
        .foreign_keys(true) // Enable foreign key constraints
        .busy_timeout(Duration::from_secs(30))
        .disable_statement_logging();

    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

    // Run migrations
    run_migrations(&pool).await?;

    Ok(pool)
}

/// Run all pending database migrations
///
/// Uses SQLx's compile-time embedded migrations from the migrations/ directory
async fn run_migrations(pool: &SqlitePool) -> DbResult<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(DatabaseError::from)?;

    Ok(())
}

/// Get a database connection from the pool
///
/// This is a convenience function for getting a connection when needed
pub async fn get_connection(
    pool: &SqlitePool,
) -> DbResult<sqlx::pool::PoolConnection<sqlx::Sqlite>> {
    pool.acquire()
        .await
        .map_err(|e| DatabaseError::ConnectionError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::Row;

    #[tokio::test]
    async fn test_init_db_creates_tables() {
        // Use in-memory database for testing
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");

        // Verify accounts table exists
        let result: (i32,) = sqlx::query_as(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='accounts'",
        )
        .fetch_one(&pool)
        .await
        .expect("Failed to query tables");

        assert_eq!(result.0, 1, "accounts table should exist");
    }

    #[tokio::test]
    async fn test_foreign_keys_enabled() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");

        // Check that foreign keys are enabled
        let result: (i32,) = sqlx::query_as("PRAGMA foreign_keys")
            .fetch_one(&pool)
            .await
            .expect("Failed to check foreign keys");

        assert_eq!(result.0, 1, "Foreign keys should be enabled");
    }

    #[tokio::test]
    async fn test_account_creation() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");

        // Insert a test account
        let result = sqlx::query(
            "INSERT INTO accounts (email, provider, pop3_host, smtp_host) VALUES (?, ?, ?, ?)",
        )
        .bind("test@example.com")
        .bind("custom")
        .bind("pop.example.com")
        .bind("smtp.example.com")
        .execute(&pool)
        .await;

        assert!(result.is_ok(), "Should insert account successfully");

        // Verify the account was created
        let count: (i32,) = sqlx::query_as("SELECT COUNT(*) FROM accounts WHERE email = ?")
            .bind("test@example.com")
            .fetch_one(&pool)
            .await
            .expect("Failed to query accounts");

        assert_eq!(count.0, 1, "Account should be created");
    }

    #[tokio::test]
    async fn test_message_foreign_key_constraint() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");

        // Create an account first
        sqlx::query(
            "INSERT INTO accounts (email, provider, pop3_host, smtp_host) VALUES (?, ?, ?, ?)",
        )
        .bind("test@example.com")
        .bind("custom")
        .bind("pop.example.com")
        .bind("smtp.example.com")
        .execute(&pool)
        .await
        .expect("Failed to create account");

        let account_id: (i64,) = sqlx::query_as("SELECT id FROM accounts WHERE email = ?")
            .bind("test@example.com")
            .fetch_one(&pool)
            .await
            .expect("Failed to get account ID");

        // Insert a message with valid foreign key
        let result = sqlx::query(
            "INSERT INTO messages (account_id, message_id, from_addr) VALUES (?, ?, ?)",
        )
        .bind(account_id.0)
        .bind("msg-123")
        .bind("sender@example.com")
        .execute(&pool)
        .await;

        assert!(
            result.is_ok(),
            "Should insert message with valid account_id"
        );

        // Try to insert a message with invalid foreign key (should fail)
        let result = sqlx::query(
            "INSERT INTO messages (account_id, message_id, from_addr) VALUES (?, ?, ?)",
        )
        .bind(99999) // Non-existent account
        .bind("msg-456")
        .bind("sender@example.com")
        .execute(&pool)
        .await;

        assert!(
            result.is_err(),
            "Should fail to insert message with invalid account_id"
        );
    }

    #[tokio::test]
    async fn test_thread_creation_and_relationship() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");

        // Create an account
        sqlx::query(
            "INSERT INTO accounts (email, provider, pop3_host, smtp_host) VALUES (?, ?, ?, ?)",
        )
        .bind("test@example.com")
        .bind("custom")
        .bind("pop.example.com")
        .bind("smtp.example.com")
        .execute(&pool)
        .await
        .expect("Failed to create account");

        let account_id: (i64,) = sqlx::query_as("SELECT id FROM accounts WHERE email = ?")
            .bind("test@example.com")
            .fetch_one(&pool)
            .await
            .expect("Failed to get account ID");

        // Create a thread
        let thread_result = sqlx::query("INSERT INTO threads (thread_subject) VALUES (?)")
            .bind("Test Thread")
            .execute(&pool)
            .await
            .expect("Failed to create thread");

        let thread_id = thread_result.last_insert_rowid();

        // Create a message in the thread
        let result = sqlx::query(
            "INSERT INTO messages (account_id, message_id, thread_id, from_addr) VALUES (?, ?, ?, ?)"
        )
        .bind(account_id.0)
        .bind("msg-123")
        .bind(thread_id)
        .bind("sender@example.com")
        .execute(&pool)
        .await;

        assert!(result.is_ok(), "Should insert message with thread_id");
    }

    #[tokio::test]
    async fn test_sync_state_tracking() {
        let pool = init_db("sqlite::memory:")
            .await
            .expect("Failed to initialize database");

        // Create an account
        sqlx::query(
            "INSERT INTO accounts (email, provider, pop3_host, smtp_host) VALUES (?, ?, ?, ?)",
        )
        .bind("test@example.com")
        .bind("custom")
        .bind("pop.example.com")
        .bind("smtp.example.com")
        .execute(&pool)
        .await
        .expect("Failed to create account");

        let account_id: (i64,) = sqlx::query_as("SELECT id FROM accounts WHERE email = ?")
            .bind("test@example.com")
            .fetch_one(&pool)
            .await
            .expect("Failed to get account ID");

        // Create sync state
        let result = sqlx::query(
            "INSERT INTO sync_state (account_id, uidl_mappings, sync_status) VALUES (?, ?, ?)",
        )
        .bind(account_id.0)
        .bind(r#"{"uidl1": 1, "uidl2": 2}"#)
        .bind("syncing")
        .execute(&pool)
        .await;

        assert!(result.is_ok(), "Should create sync state");

        // Verify sync state
        let row =
            sqlx::query("SELECT sync_status, uidl_mappings FROM sync_state WHERE account_id = ?")
                .bind(account_id.0)
                .fetch_one(&pool)
                .await
                .expect("Failed to fetch sync state");

        let sync_status: String = row.get("sync_status");
        assert_eq!(sync_status, "syncing");
    }
}
