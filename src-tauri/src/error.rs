//! Custom error types for the OpenHuman email client
//!
//! This module defines domain-specific error types using thiserror to provide
//! user-friendly error messages while maintaining type safety throughout the application.

use thiserror::Error;

/// Database-related errors
#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Failed to connect to database: {0}")]
    ConnectionError(String),

    #[error("Failed to run database migration: {0}")]
    MigrationError(String),

    #[error("Failed to execute query: {0}")]
    QueryError(String),

    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),

    #[error("Database operation failed: {0}")]
    OperationFailed(String),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => DatabaseError::NotFound("Record not found".to_string()),
            sqlx::Error::Database(db_err) => {
                DatabaseError::ConstraintViolation(db_err.message().to_string())
            }
            _ => DatabaseError::QueryError(err.to_string()),
        }
    }
}

impl From<sqlx::migrate::MigrateError> for DatabaseError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        DatabaseError::MigrationError(err.to_string())
    }
}

/// IMAP protocol errors
#[derive(Error, Debug)]
pub enum ImapError {
    #[error(
        "Failed to connect to IMAP server: Check your internet connection and server settings"
    )]
    ConnectionFailed(String),

    #[error("Authentication failed: Please check your email and password")]
    AuthenticationFailed,

    #[error("Failed to fetch messages: {0}")]
    FetchFailed(String),

    #[error("Invalid server response: {0}")]
    InvalidResponse(String),

    #[error("Connection timeout: Please check your internet connection")]
    Timeout,

    #[error("Folder not found: {0}")]
    FolderNotFound(String),

    #[error("UIDVALIDITY changed - folder requires resync")]
    UidValidityChanged,

    #[error("Invalid folder name: {0}")]
    InvalidFolderName(String),

    #[error("IMAP operation failed: {0}")]
    OperationFailed(String),
}

/// SMTP protocol errors
#[derive(Error, Debug)]
pub enum SmtpError {
    #[error("Failed to connect to SMTP server: {0}")]
    ConnectionFailed(String),

    #[error("Authentication failed: Please check your email and password")]
    AuthenticationFailed,

    #[error("Failed to send email: {0}")]
    SendFailed(String),

    #[error("Invalid email format: {0}")]
    InvalidEmail(String),

    #[error("Connection timeout: Please check your internet connection")]
    Timeout,

    #[error("SMTP operation failed: {0}")]
    OperationFailed(String),
}

impl From<lettre::transport::smtp::Error> for SmtpError {
    fn from(err: lettre::transport::smtp::Error) -> Self {
        if err.is_timeout() {
            SmtpError::Timeout
        } else if err.is_client() {
            SmtpError::InvalidEmail(err.to_string())
        } else {
            SmtpError::SendFailed(err.to_string())
        }
    }
}

impl From<lettre::address::AddressError> for SmtpError {
    fn from(err: lettre::address::AddressError) -> Self {
        SmtpError::InvalidEmail(err.to_string())
    }
}

/// Email sync errors
#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Sync failed: {0}")]
    Failed(String),

    #[error("Authentication required: Please re-enter your credentials")]
    AuthRequired,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Database error during sync: {0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("IMAP error during sync: {0}")]
    ImapError(#[from] ImapError),

    #[error("MIME parsing error: {0}")]
    ParseError(#[from] MimeParseError),

    #[error("Sync cancelled by user")]
    Cancelled,
}

/// MIME parsing errors
#[derive(Error, Debug)]
pub enum MimeParseError {
    #[error("Failed to parse MIME message: {0}")]
    ParseFailed(String),

    #[error("Invalid message format: {0}")]
    InvalidFormat(String),

    #[error("Failed to extract message headers: {0}")]
    HeaderExtractionFailed(String),

    #[error("Failed to parse attachment: {0}")]
    AttachmentParseError(String),
}

impl From<mailparse::MailParseError> for MimeParseError {
    fn from(err: mailparse::MailParseError) -> Self {
        MimeParseError::ParseFailed(err.to_string())
    }
}

/// Keychain access errors
#[derive(Error, Debug)]
pub enum KeychainError {
    #[error("Failed to access keychain: {0}")]
    AccessFailed(String),

    #[error("Credentials not found for account: {0}")]
    CredentialsNotFound(String),

    #[error("Failed to store credentials: {0}")]
    StoreFailed(String),

    #[error("Failed to delete credentials: {0}")]
    DeleteFailed(String),
}

impl From<keyring::Error> for KeychainError {
    fn from(err: keyring::Error) -> Self {
        match err {
            keyring::Error::NoEntry => {
                KeychainError::CredentialsNotFound("No credentials stored".to_string())
            }
            _ => KeychainError::AccessFailed(err.to_string()),
        }
    }
}

/// Threading algorithm errors
#[derive(Error, Debug)]
pub enum ThreadingError {
    #[error("Failed to build thread structure: {0}")]
    BuildFailed(String),

    #[error("Circular reference detected in message thread")]
    CircularReference,

    #[error("Invalid message ID: {0}")]
    InvalidMessageId(String),
}

// Type aliases for Results
pub type DbResult<T> = Result<T, DatabaseError>;
pub type ImapResult<T> = Result<T, ImapError>;
pub type SmtpResult<T> = Result<T, SmtpError>;
pub type SyncResult<T> = Result<T, SyncError>;
pub type MimeResult<T> = Result<T, MimeParseError>;
pub type KeychainResult<T> = Result<T, KeychainError>;
pub type ThreadingResult<T> = Result<T, ThreadingError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_error_display() {
        let err = DatabaseError::NotFound("Account not found".to_string());
        assert_eq!(err.to_string(), "Record not found: Account not found");
    }

    #[test]
    fn test_imap_error_authentication_message() {
        let err = ImapError::AuthenticationFailed;
        assert!(err
            .to_string()
            .contains("Please check your email and password"));
    }

    #[test]
    fn test_smtp_error_user_friendly() {
        let err = SmtpError::Timeout;
        assert!(err.to_string().contains("internet connection"));
    }

    #[test]
    fn test_keychain_error_conversion() {
        let keyring_err = keyring::Error::NoEntry;
        let err: KeychainError = keyring_err.into();
        assert!(matches!(err, KeychainError::CredentialsNotFound(_)));
    }

    #[test]
    fn test_sync_error_wraps_database_error() {
        let db_err = DatabaseError::QueryError("Query failed".to_string());
        let sync_err: SyncError = db_err.into();
        assert!(matches!(sync_err, SyncError::DatabaseError(_)));
    }

    #[test]
    fn test_imap_error_folder_not_found() {
        let err = ImapError::FolderNotFound("Sent".to_string());
        assert!(err.to_string().contains("Folder not found"));
    }

    #[test]
    fn test_imap_error_uidvalidity_changed() {
        let err = ImapError::UidValidityChanged;
        assert!(err.to_string().contains("UIDVALIDITY changed"));
    }
}
