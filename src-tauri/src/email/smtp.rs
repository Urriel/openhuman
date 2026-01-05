//! SMTP Send Engine
//!
//! This module implements email sending via SMTP with support for:
//! - Single and batch recipients (To/Cc/Bcc)
//! - Outbox queue processing
//! - Retry logic with exponential backoff
//! - MIME formatting (plain text + HTML)
//! - Progress events

use crate::error::{SmtpError, SmtpResult};
use crate::retry::retry_with_backoff;
use lettre::message::{header, Message, MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncTransport, Tokio1Executor};
use sqlx::SqlitePool;

/// SMTP client for sending emails
pub struct SmtpClient {
    host: String,
    port: u16,
    email: String,
    password: String,
}

/// Recipients for an email
#[derive(Debug, Clone)]
pub struct EmailRecipients {
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
}

/// Email to send
#[derive(Debug, Clone)]
pub struct Email {
    pub recipients: EmailRecipients,
    pub subject: String,
    pub body_plain: String,
    pub body_html: Option<String>,
}

/// Status of an email send operation
#[derive(Debug, Clone, PartialEq)]
pub enum SendStatus {
    Pending,
    Sending,
    Sent,
    Failed,
    Retry,
}

impl SendStatus {
    pub fn as_str(&self) -> &str {
        match self {
            SendStatus::Pending => "pending",
            SendStatus::Sending => "sending",
            SendStatus::Sent => "sent",
            SendStatus::Failed => "failed",
            SendStatus::Retry => "retry",
        }
    }
}

impl SmtpClient {
    /// Create a new SMTP client
    pub fn new(host: String, port: u16, email: String, password: String) -> Self {
        Self {
            host,
            port,
            email,
            password,
        }
    }

    /// Send an email with retry logic
    pub async fn send_email(&self, email: Email) -> SmtpResult<()> {
        retry_with_backoff(|| async { self.send_email_internal(&email).await }, 5).await
    }

    /// Internal send implementation without retry
    async fn send_email_internal(&self, email: &Email) -> SmtpResult<()> {
        // Build the email message
        let message = self.build_message(email)?;

        // Create SMTP transport
        let transport = self.create_transport()?;

        // Send the email
        transport
            .send(message)
            .await
            .map_err(|e| SmtpError::from(e))?;

        Ok(())
    }

    /// Build a MIME message from email data
    fn build_message(&self, email: &Email) -> SmtpResult<Message> {
        let mut message_builder = Message::builder()
            .from(self.email.parse().map_err(SmtpError::from)?)
            .subject(&email.subject);

        // Add To recipients
        for recipient in &email.recipients.to {
            message_builder = message_builder.to(recipient.parse().map_err(SmtpError::from)?);
        }

        // Add Cc recipients
        for recipient in &email.recipients.cc {
            message_builder = message_builder.cc(recipient.parse().map_err(SmtpError::from)?);
        }

        // Add Bcc recipients
        for recipient in &email.recipients.bcc {
            message_builder = message_builder.bcc(recipient.parse().map_err(SmtpError::from)?);
        }

        // Build multipart body (plain text + HTML)
        let body = if let Some(html) = &email.body_html {
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(email.body_plain.clone()),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(html.clone()),
                )
        } else {
            MultiPart::alternative().singlepart(
                SinglePart::builder()
                    .header(header::ContentType::TEXT_PLAIN)
                    .body(email.body_plain.clone()),
            )
        };

        message_builder
            .multipart(body)
            .map_err(|e| SmtpError::InvalidEmail(e.to_string()))
    }

    /// Create SMTP transport
    fn create_transport(&self) -> SmtpResult<AsyncSmtpTransport<Tokio1Executor>> {
        let creds = Credentials::new(self.email.clone(), self.password.clone());

        AsyncSmtpTransport::<Tokio1Executor>::relay(&self.host)
            .map_err(|e| SmtpError::ConnectionFailed(e.to_string()))?
            .port(self.port)
            .credentials(creds)
            .build()
            .pipe(Ok)
    }
}

/// Process the outbox queue
///
/// Queries pending emails from the outbox table and attempts to send them.
/// Updates the status and retry information based on results.
pub async fn process_outbox(pool: &SqlitePool) -> SmtpResult<OutboxStats> {
    let mut stats = OutboxStats::default();

    // Get pending or retry emails from outbox
    let outbox_items = get_pending_outbox_items(pool).await?;

    for item in outbox_items {
        stats.total += 1;

        // Get account credentials
        let account = get_account(pool, item.account_id).await?;

        // Create SMTP client
        let client = SmtpClient::new(
            account.smtp_host,
            account.smtp_port,
            account.email.clone(),
            account.password,
        );

        // Parse recipients
        let recipients = parse_recipients(&item.recipients)?;

        // Build email
        let email = Email {
            recipients,
            subject: item.subject,
            body_plain: item.body_plain,
            body_html: item.body_html,
        };

        // Update status to sending
        update_outbox_status(pool, item.id, SendStatus::Sending, None).await?;

        // Try to send
        match client.send_email(email).await {
            Ok(()) => {
                // Mark as sent
                update_outbox_status(pool, item.id, SendStatus::Sent, None).await?;
                stats.sent += 1;
            }
            Err(e) => {
                // Handle failure
                let new_retry_count = item.retry_count + 1;

                if new_retry_count >= 5 {
                    // Max retries reached, mark as failed
                    update_outbox_status(pool, item.id, SendStatus::Failed, Some(&e.to_string()))
                        .await?;
                    stats.failed += 1;
                } else {
                    // Schedule retry with exponential backoff
                    let retry_delay = calculate_retry_delay(new_retry_count);
                    let next_retry_at = chrono::Utc::now() + retry_delay;

                    update_outbox_retry(
                        pool,
                        item.id,
                        new_retry_count,
                        &next_retry_at.to_rfc3339(),
                        &e.to_string(),
                    )
                    .await?;
                    stats.retrying += 1;
                }
            }
        }
    }

    Ok(stats)
}

/// Statistics from processing outbox
#[derive(Debug, Default, Clone)]
pub struct OutboxStats {
    pub total: usize,
    pub sent: usize,
    pub failed: usize,
    pub retrying: usize,
}

/// Outbox item from database
#[derive(Debug, sqlx::FromRow)]
struct OutboxItem {
    id: i64,
    account_id: i64,
    recipients: String,
    subject: String,
    body_plain: String,
    body_html: Option<String>,
    retry_count: i32,
}

/// Account credentials
#[derive(Debug)]
struct AccountCredentials {
    email: String,
    smtp_host: String,
    smtp_port: u16,
    password: String,
}

/// Get pending outbox items
async fn get_pending_outbox_items(pool: &SqlitePool) -> SmtpResult<Vec<OutboxItem>> {
    let items = sqlx::query_as::<_, OutboxItem>(
        r#"
        SELECT id, account_id, recipients, subject, body_plain, body_html, retry_count
        FROM outbox
        WHERE send_status IN ('pending', 'retry')
        AND (next_retry_at IS NULL OR next_retry_at <= datetime('now'))
        ORDER BY created_at ASC
        "#,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| SmtpError::OperationFailed(e.to_string()))?;

    Ok(items)
}

/// Get account credentials (including from keychain)
async fn get_account(pool: &SqlitePool, account_id: i64) -> SmtpResult<AccountCredentials> {
    #[derive(sqlx::FromRow)]
    struct AccountRow {
        email: String,
        smtp_host: String,
        smtp_port: i64,
    }

    let account = sqlx::query_as::<_, AccountRow>(
        r#"
        SELECT email, smtp_host, smtp_port
        FROM accounts
        WHERE id = ?
        "#,
    )
    .bind(account_id)
    .fetch_one(pool)
    .await
    .map_err(|e| SmtpError::OperationFailed(e.to_string()))?;

    // Get password from keychain
    let password = crate::keychain::get_credentials(&account.email)
        .map_err(|_e| SmtpError::AuthenticationFailed)?;

    Ok(AccountCredentials {
        email: account.email,
        smtp_host: account.smtp_host,
        smtp_port: account.smtp_port as u16,
        password,
    })
}

/// Update outbox status
async fn update_outbox_status(
    pool: &SqlitePool,
    id: i64,
    status: SendStatus,
    error_message: Option<&str>,
) -> SmtpResult<()> {
    sqlx::query(
        r#"
        UPDATE outbox
        SET send_status = ?, error_message = ?
        WHERE id = ?
        "#,
    )
    .bind(status.as_str())
    .bind(error_message)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| SmtpError::OperationFailed(e.to_string()))?;

    Ok(())
}

/// Update outbox retry information
async fn update_outbox_retry(
    pool: &SqlitePool,
    id: i64,
    retry_count: i32,
    next_retry_at: &str,
    error_message: &str,
) -> SmtpResult<()> {
    sqlx::query(
        r#"
        UPDATE outbox
        SET send_status = 'retry',
            retry_count = ?,
            next_retry_at = ?,
            error_message = ?
        WHERE id = ?
        "#,
    )
    .bind(retry_count)
    .bind(next_retry_at)
    .bind(error_message)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| SmtpError::OperationFailed(e.to_string()))?;

    Ok(())
}

/// Parse recipients from JSON string
fn parse_recipients(recipients_json: &str) -> SmtpResult<EmailRecipients> {
    let recipients: serde_json::Value = serde_json::from_str(recipients_json)
        .map_err(|e| SmtpError::InvalidEmail(format!("Invalid recipients JSON: {}", e)))?;

    Ok(EmailRecipients {
        to: recipients
            .get("to")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        cc: recipients
            .get("cc")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
        bcc: recipients
            .get("bcc")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default(),
    })
}

/// Calculate retry delay based on attempt count
fn calculate_retry_delay(attempt: i32) -> chrono::Duration {
    let seconds = 2_i64.pow(attempt as u32);
    chrono::Duration::seconds(seconds)
}

// Helper trait for pipe operator
trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

impl<T> Pipe for T {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_status_as_str() {
        assert_eq!(SendStatus::Pending.as_str(), "pending");
        assert_eq!(SendStatus::Sent.as_str(), "sent");
        assert_eq!(SendStatus::Failed.as_str(), "failed");
        assert_eq!(SendStatus::Retry.as_str(), "retry");
    }

    #[test]
    fn test_parse_recipients_valid() {
        let json =
            r#"{"to":["user1@example.com","user2@example.com"],"cc":["cc@example.com"],"bcc":[]}"#;
        let recipients = parse_recipients(json).unwrap();

        assert_eq!(recipients.to.len(), 2);
        assert_eq!(recipients.cc.len(), 1);
        assert_eq!(recipients.bcc.len(), 0);
        assert_eq!(recipients.to[0], "user1@example.com");
    }

    #[test]
    fn test_parse_recipients_empty() {
        let json = r#"{"to":[],"cc":[],"bcc":[]}"#;
        let recipients = parse_recipients(json).unwrap();

        assert_eq!(recipients.to.len(), 0);
        assert_eq!(recipients.cc.len(), 0);
        assert_eq!(recipients.bcc.len(), 0);
    }

    #[test]
    fn test_calculate_retry_delay() {
        assert_eq!(calculate_retry_delay(1), chrono::Duration::seconds(2));
        assert_eq!(calculate_retry_delay(2), chrono::Duration::seconds(4));
        assert_eq!(calculate_retry_delay(3), chrono::Duration::seconds(8));
        assert_eq!(calculate_retry_delay(4), chrono::Duration::seconds(16));
    }

    #[tokio::test]
    async fn test_smtp_client_creation() {
        let client = SmtpClient::new(
            "smtp.example.com".to_string(),
            587,
            "test@example.com".to_string(),
            "password".to_string(),
        );

        assert_eq!(client.host, "smtp.example.com");
        assert_eq!(client.port, 587);
        assert_eq!(client.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_build_message_plain_text() {
        let client = SmtpClient::new(
            "smtp.example.com".to_string(),
            587,
            "sender@example.com".to_string(),
            "password".to_string(),
        );

        let email = Email {
            recipients: EmailRecipients {
                to: vec!["recipient@example.com".to_string()],
                cc: vec![],
                bcc: vec![],
            },
            subject: "Test Subject".to_string(),
            body_plain: "Plain text body".to_string(),
            body_html: None,
        };

        let message = client.build_message(&email);
        assert!(message.is_ok());
    }

    #[tokio::test]
    async fn test_build_message_with_html() {
        let client = SmtpClient::new(
            "smtp.example.com".to_string(),
            587,
            "sender@example.com".to_string(),
            "password".to_string(),
        );

        let email = Email {
            recipients: EmailRecipients {
                to: vec!["recipient@example.com".to_string()],
                cc: vec!["cc@example.com".to_string()],
                bcc: vec!["bcc@example.com".to_string()],
            },
            subject: "Test Subject".to_string(),
            body_plain: "Plain text body".to_string(),
            body_html: Some("<p>HTML body</p>".to_string()),
        };

        let message = client.build_message(&email);
        assert!(message.is_ok());
    }
}
