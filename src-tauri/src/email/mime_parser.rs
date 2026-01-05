//! MIME Message Parser
//!
//! This module parses raw email messages into structured data:
//! - Extract headers (From, To, Cc, Bcc, Subject, Date, Message-ID, References, In-Reply-To)
//! - Extract plain text and HTML bodies
//! - Parse attachments with metadata
//! - Handle inline images
//! - Parse iCal calendar invites
//! - Sanitize HTML to prevent XSS

use crate::error::MimeResult;
use mailparse::{parse_mail, MailHeaderMap, ParsedMail};

/// Parsed email message
#[derive(Debug, Clone)]
pub struct ParsedMessage {
    pub subject: Option<String>,
    pub from: Option<String>,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub bcc: Vec<String>,
    pub date: Option<String>,
    pub message_id: Option<String>,
    pub references: Vec<String>,
    pub in_reply_to: Option<String>,
    pub body_plain: Option<String>,
    pub body_html: Option<String>,
    pub attachments: Vec<Attachment>,
}

/// Email attachment
#[derive(Debug, Clone)]
pub struct Attachment {
    pub filename: String,
    pub size: usize,
    pub mime_type: String,
    pub content: Vec<u8>,
    pub is_inline: bool,
    pub content_id: Option<String>,
    pub is_calendar: bool,
}

/// Parse a raw email message
pub fn parse_message(raw: &[u8]) -> MimeResult<ParsedMessage> {
    let parsed = parse_mail(raw)?;

    let mut message = ParsedMessage {
        subject: extract_header(&parsed, "Subject"),
        from: extract_header(&parsed, "From"),
        to: extract_header_list(&parsed, "To"),
        cc: extract_header_list(&parsed, "Cc"),
        bcc: extract_header_list(&parsed, "Bcc"),
        date: extract_header(&parsed, "Date"),
        message_id: extract_header(&parsed, "Message-ID"),
        references: extract_references(&parsed),
        in_reply_to: extract_header(&parsed, "In-Reply-To"),
        body_plain: None,
        body_html: None,
        attachments: Vec::new(),
    };

    // Extract body and attachments
    extract_body_and_attachments(&parsed, &mut message)?;

    // Sanitize HTML if present
    if let Some(html) = &message.body_html {
        message.body_html = Some(sanitize_html(html));
    }

    Ok(message)
}

/// Extract a single header value
fn extract_header(mail: &ParsedMail, header_name: &str) -> Option<String> {
    mail.headers.get_first_value(header_name)
}

/// Extract a comma-separated header list (e.g., To, Cc, Bcc)
fn extract_header_list(mail: &ParsedMail, header_name: &str) -> Vec<String> {
    extract_header(mail, header_name)
        .map(|value| {
            value
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

/// Extract References header as a list of message IDs
fn extract_references(mail: &ParsedMail) -> Vec<String> {
    extract_header(mail, "References")
        .map(|value| {
            value
                .split_whitespace()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

/// Extract body and attachments from parsed mail
fn extract_body_and_attachments(mail: &ParsedMail, message: &mut ParsedMessage) -> MimeResult<()> {
    // Check if this part is the body or an attachment
    let content_type = mail.ctype.mimetype.to_lowercase();
    let disposition = mail.get_content_disposition();
    let is_attachment = matches!(
        disposition.disposition,
        mailparse::DispositionType::Attachment
    );
    let is_inline = matches!(disposition.disposition, mailparse::DispositionType::Inline);

    // Handle multipart messages recursively
    if content_type.starts_with("multipart/") {
        for subpart in &mail.subparts {
            extract_body_and_attachments(subpart, message)?;
        }
        return Ok(());
    }

    // Extract body content
    if !is_attachment && content_type == "text/plain" && message.body_plain.is_none() {
        message.body_plain = Some(mail.get_body().unwrap_or_default());
    } else if !is_attachment && content_type == "text/html" && message.body_html.is_none() {
        message.body_html = Some(mail.get_body().unwrap_or_default());
    }
    // Extract attachments
    else if is_attachment || is_inline {
        let filename = mail
            .get_content_disposition()
            .params
            .get("filename")
            .cloned()
            .unwrap_or_else(|| "untitled".to_string());

        let content = mail.get_body_raw().unwrap_or_default();
        let content_id = extract_header(mail, "Content-ID");

        // Check if this is a calendar invite
        let is_calendar = content_type.contains("calendar")
            || content_type.contains("ics")
            || filename.ends_with(".ics");

        message.attachments.push(Attachment {
            filename,
            size: content.len(),
            mime_type: content_type.clone(),
            content,
            is_inline,
            content_id,
            is_calendar,
        });
    }

    Ok(())
}

/// Sanitize HTML content to prevent XSS attacks
///
/// This is a basic sanitization that removes script tags and dangerous attributes.
/// For production use, consider using a more robust HTML sanitization library.
fn sanitize_html(html: &str) -> String {
    let mut sanitized = html.to_string();

    // Remove script tags and their content
    let script_regex = regex::Regex::new(r"(?i)<script\b[^>]*>[\s\S]*?</script>").unwrap();
    sanitized = script_regex.replace_all(&sanitized, "").to_string();

    // Remove inline event handlers (onclick, onload, etc.)
    let event_regex = regex::Regex::new(r#"(?i)\s*on\w+\s*=\s*["'][^"']*["']"#).unwrap();
    sanitized = event_regex.replace_all(&sanitized, "").to_string();

    // Remove javascript: protocol from href and src
    let js_protocol_regex = regex::Regex::new(r"(?i)javascript:").unwrap();
    sanitized = js_protocol_regex.replace_all(&sanitized, "").to_string();

    sanitized
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLAIN_TEXT_EMAIL: &[u8] = b"From: sender@example.com\r
To: recipient@example.com\r
Subject: Test Subject\r
Message-ID: <123@example.com>\r
Date: Mon, 1 Jan 2024 12:00:00 +0000\r
\r
This is a plain text email body.";

    const HTML_EMAIL: &[u8] = b"From: sender@example.com\r
To: recipient@example.com\r
Subject: HTML Email\r
Message-ID: <456@example.com>\r
Content-Type: text/html\r
\r
<html><body><p>This is an <b>HTML</b> email.</p></body></html>";

    const EMAIL_WITH_ATTACHMENT: &[u8] = b"From: sender@example.com\r
To: recipient@example.com\r
Subject: Email with Attachment\r
Message-ID: <789@example.com>\r
Content-Type: multipart/mixed; boundary=boundary123\r
\r
--boundary123\r
Content-Type: text/plain\r
\r
Email body with attachment.\r
--boundary123\r
Content-Type: application/pdf\r
Content-Disposition: attachment; filename=\"document.pdf\"\r
\r
PDF content here\r
--boundary123--";

    #[test]
    fn test_parse_plain_text_email() {
        let message = parse_message(PLAIN_TEXT_EMAIL).unwrap();

        assert_eq!(message.subject, Some("Test Subject".to_string()));
        assert_eq!(message.from, Some("sender@example.com".to_string()));
        assert_eq!(message.to, vec!["recipient@example.com".to_string()]);
        assert_eq!(message.message_id, Some("<123@example.com>".to_string()));
        assert!(message.body_plain.is_some());
        assert!(message
            .body_plain
            .unwrap()
            .contains("plain text email body"));
    }

    #[test]
    fn test_parse_html_email() {
        let message = parse_message(HTML_EMAIL).unwrap();

        assert_eq!(message.subject, Some("HTML Email".to_string()));
        assert!(message.body_html.is_some());
        let html = message.body_html.unwrap();
        assert!(html.contains("<p>"));
        assert!(html.contains("HTML"));
    }

    #[test]
    fn test_parse_email_with_attachment() {
        let message = parse_message(EMAIL_WITH_ATTACHMENT).unwrap();

        assert_eq!(message.subject, Some("Email with Attachment".to_string()));
        assert_eq!(message.attachments.len(), 1);

        let attachment = &message.attachments[0];
        assert_eq!(attachment.filename, "document.pdf");
        assert_eq!(attachment.mime_type, "application/pdf");
        assert!(attachment.size > 0);
    }

    #[test]
    fn test_extract_references() {
        let raw = b"From: sender@example.com\r
References: <msg1@example.com> <msg2@example.com> <msg3@example.com>\r
In-Reply-To: <msg3@example.com>\r
\r
Body";

        let message = parse_message(raw).unwrap();

        assert_eq!(message.references.len(), 3);
        assert_eq!(message.references[0], "<msg1@example.com>");
        assert_eq!(message.in_reply_to, Some("<msg3@example.com>".to_string()));
    }

    #[test]
    fn test_sanitize_html_removes_scripts() {
        let malicious_html = "<p>Hello</p><script>alert('xss')</script><p>World</p>";
        let sanitized = sanitize_html(malicious_html);

        assert!(!sanitized.contains("<script"));
        assert!(!sanitized.contains("alert"));
        assert!(sanitized.contains("<p>Hello</p>"));
    }

    #[test]
    fn test_sanitize_html_removes_event_handlers() {
        let malicious_html = r##"<a href="#" onclick="alert('xss')">Click</a>"##;
        let sanitized = sanitize_html(malicious_html);

        assert!(!sanitized.contains("onclick"));
        assert!(sanitized.contains("<a"));
    }

    #[test]
    fn test_sanitize_html_removes_javascript_protocol() {
        let malicious_html = r##"<a href="javascript:alert('xss')">Click</a>"##;
        let sanitized = sanitize_html(malicious_html);

        assert!(!sanitized.contains("javascript:"));
    }
}
