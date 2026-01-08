//! IMAP client implementation with full IMAP support
//!
//! This module provides an IMAP client for email synchronization with:
//! - UID-based incremental sync
//! - Multi-folder support
//! - Flag synchronization (\Seen, \Flagged, \Deleted)
//! - IDLE support for real-time notifications
//! - APPEND for sent messages and drafts

use crate::error::{ImapError, ImapResult};
use async_imap::types::{Fetch, Mailbox, Name};
use futures::StreamExt;
use std::collections::HashMap;
use tokio::net::TcpStream;

/// IMAP client for email retrieval and synchronization
pub struct ImapClient {
    host: String,
    port: u16,
    email: String,
    session: Option<async_imap::Session<tokio_native_tls::TlsStream<TcpStream>>>,
}

/// IMAP folder information
#[derive(Debug, Clone)]
pub struct ImapFolder {
    pub name: String,
    pub folder_type: Option<String>,
    pub selectable: bool,
    pub flags: Vec<String>,
}

/// IMAP message with UID and flags
#[derive(Debug, Clone)]
pub struct ImapMessage {
    pub uid: u32,
    pub message_id: Option<String>,
    pub flags: Vec<String>,
    pub body: Option<Vec<u8>>,
}

/// Flag action for STORE command
pub enum FlagAction {
    Add,
    Remove,
    Set,
}

impl ImapClient {
    /// Connect to an IMAP server and authenticate
    ///
    /// # Arguments
    ///
    /// * `host` - IMAP server hostname
    /// * `port` - IMAP server port (default: 993 for SSL)
    /// * `email` - Email address for authentication
    /// * `password` - Password for authentication
    ///
    /// # Returns
    ///
    /// Connected and authenticated IMAP client
    pub async fn connect(host: &str, port: u16, email: &str, password: &str) -> ImapResult<Self> {
        // Connect via TCP
        let addr = format!("{}:{}", host, port);
        let tcp_stream = TcpStream::connect(&addr)
            .await
            .map_err(|e| ImapError::ConnectionFailed(format!("TCP connection failed: {}", e)))?;

        // Upgrade to TLS
        let tls = tokio_native_tls::native_tls::TlsConnector::new().map_err(|e| {
            ImapError::ConnectionFailed(format!("TLS connector creation failed: {}", e))
        })?;
        let tls = tokio_native_tls::TlsConnector::from(tls);
        let tls_stream = tls
            .connect(host, tcp_stream)
            .await
            .map_err(|e| ImapError::ConnectionFailed(format!("TLS handshake failed: {}", e)))?;

        // Create IMAP client
        let client = async_imap::Client::new(tls_stream);

        // Authenticate
        let session = client
            .login(email, password)
            .await
            .map_err(|(err, _client)| match err {
                async_imap::error::Error::No(msg) | async_imap::error::Error::Bad(msg) => {
                    if msg.contains("authentication") || msg.contains("login") {
                        ImapError::AuthenticationFailed
                    } else {
                        ImapError::InvalidResponse(msg)
                    }
                }
                _ => ImapError::ConnectionFailed(err.to_string()),
            })?;

        Ok(Self {
            host: host.to_string(),
            port,
            email: email.to_string(),
            session: Some(session),
        })
    }

    /// List all folders on the IMAP server
    ///
    /// # Returns
    ///
    /// Vector of folder information
    pub async fn list_folders(&mut self) -> ImapResult<Vec<ImapFolder>> {
        let session = self
            .session
            .as_mut()
            .ok_or_else(|| ImapError::ConnectionFailed("Not connected".to_string()))?;

        let mut mailboxes_stream = session
            .list(Some(""), Some("*"))
            .await
            .map_err(|e| ImapError::OperationFailed(format!("LIST command failed: {}", e)))?;

        let mut folders = Vec::new();
        while let Some(name_result) = mailboxes_stream.next().await {
            match name_result {
                Ok(name) => folders.push(Self::parse_folder_info(&name)),
                Err(e) => {
                    return Err(ImapError::OperationFailed(format!(
                        "LIST parsing failed: {}",
                        e
                    )))
                }
            }
        }

        Ok(folders)
    }

    /// Parse folder information from IMAP LIST response
    fn parse_folder_info(name: &Name) -> ImapFolder {
        let folder_name = name.name();

        // Detect folder type based on name and attributes
        let folder_type = Self::detect_folder_type(folder_name, &name.attributes());

        // Check if folder is selectable (\NoSelect attribute)
        let selectable = !name
            .attributes()
            .iter()
            .any(|attr| matches!(attr, async_imap::types::NameAttribute::NoSelect));

        // Convert attributes to string flags
        let flags: Vec<String> = name
            .attributes()
            .iter()
            .map(|attr| format!("{:?}", attr))
            .collect();

        ImapFolder {
            name: folder_name.to_string(),
            folder_type,
            selectable,
            flags,
        }
    }

    /// Detect standard folder type from name and attributes
    fn detect_folder_type(
        name: &str,
        _attributes: &[async_imap::types::NameAttribute],
    ) -> Option<String> {
        let name_lower = name.to_lowercase();

        if name_lower == "inbox" {
            Some("Inbox".to_string())
        } else if name_lower.contains("sent") {
            Some("Sent".to_string())
        } else if name_lower.contains("draft") {
            Some("Drafts".to_string())
        } else if name_lower.contains("trash") || name_lower.contains("deleted") {
            Some("Trash".to_string())
        } else if name_lower.contains("archive") {
            Some("Archive".to_string())
        } else if name_lower.contains("spam") || name_lower.contains("junk") {
            Some("Spam".to_string())
        } else {
            None
        }
    }

    /// Select a folder for operations
    ///
    /// # Arguments
    ///
    /// * `folder_name` - Name of the folder to select (e.g., "INBOX")
    ///
    /// # Returns
    ///
    /// Mailbox information including UIDVALIDITY and UIDNEXT
    pub async fn select_folder(&mut self, folder_name: &str) -> ImapResult<Mailbox> {
        let session = self
            .session
            .as_mut()
            .ok_or_else(|| ImapError::ConnectionFailed("Not connected".to_string()))?;

        session.select(folder_name).await.map_err(|e| {
            if e.to_string().contains("does not exist") {
                ImapError::FolderNotFound(folder_name.to_string())
            } else {
                ImapError::OperationFailed(format!("SELECT failed: {}", e))
            }
        })
    }

    /// Fetch new messages from a folder since a given UID
    ///
    /// # Arguments
    ///
    /// * `since_uid` - Fetch messages with UID greater than this value (None for all)
    ///
    /// # Returns
    ///
    /// Vector of messages with UIDs and flags
    pub async fn fetch_new_messages(
        &mut self,
        since_uid: Option<u32>,
    ) -> ImapResult<Vec<ImapMessage>> {
        let session = self
            .session
            .as_mut()
            .ok_or_else(|| ImapError::ConnectionFailed("Not connected".to_string()))?;

        // Build UID range query
        let uid_range = if let Some(uid) = since_uid {
            format!("{}:*", uid + 1)
        } else {
            "1:*".to_string()
        };

        // Fetch messages with UID, FLAGS, RFC822 (full message)
        let mut fetches_stream = session
            .uid_fetch(&uid_range, "(UID FLAGS RFC822)")
            .await
            .map_err(|e| ImapError::FetchFailed(format!("FETCH failed: {}", e)))?;

        let mut messages = Vec::new();
        while let Some(fetch_result) = fetches_stream.next().await {
            match fetch_result {
                Ok(fetch) => {
                    if let Some(msg) = Self::parse_fetch_response(&fetch) {
                        messages.push(msg);
                    }
                }
                Err(e) => {
                    return Err(ImapError::FetchFailed(format!(
                        "FETCH parsing failed: {}",
                        e
                    )))
                }
            }
        }

        Ok(messages)
    }

    /// Parse FETCH response into ImapMessage
    fn parse_fetch_response(fetch: &Fetch) -> Option<ImapMessage> {
        let uid = fetch.uid?;

        let flags: Vec<String> = fetch.flags().map(|flag| format!("{:?}", flag)).collect();

        let body = fetch.body().map(|b| b.to_vec());

        Some(ImapMessage {
            uid,
            message_id: None, // Will be parsed from body
            flags,
            body,
        })
    }

    /// Fetch only flags for messages (for flag sync)
    ///
    /// # Arguments
    ///
    /// * `uid_range` - Range of UIDs to fetch (e.g., "1:*" for all)
    ///
    /// # Returns
    ///
    /// Map of UID to flags
    pub async fn fetch_message_flags(
        &mut self,
        uid_range: &str,
    ) -> ImapResult<HashMap<u32, Vec<String>>> {
        let session = self
            .session
            .as_mut()
            .ok_or_else(|| ImapError::ConnectionFailed("Not connected".to_string()))?;

        let mut fetches_stream = session
            .uid_fetch(uid_range, "(UID FLAGS)")
            .await
            .map_err(|e| ImapError::FetchFailed(format!("FLAG FETCH failed: {}", e)))?;

        let mut flag_map = HashMap::new();
        while let Some(fetch_result) = fetches_stream.next().await {
            match fetch_result {
                Ok(fetch) => {
                    if let Some(uid) = fetch.uid {
                        let flags: Vec<String> =
                            fetch.flags().map(|flag| format!("{:?}", flag)).collect();
                        flag_map.insert(uid, flags);
                    }
                }
                Err(e) => {
                    return Err(ImapError::FetchFailed(format!(
                        "FLAG FETCH parsing failed: {}",
                        e
                    )))
                }
            }
        }

        Ok(flag_map)
    }

    /// Set flags on a message using STORE command
    ///
    /// # Arguments
    ///
    /// * `uid` - UID of the message
    /// * `flags` - Flags to set (e.g., ["\\Seen", "\\Flagged"])
    /// * `action` - Add, Remove, or Set flags
    pub async fn set_flags(
        &mut self,
        uid: u32,
        flags: &[&str],
        action: FlagAction,
    ) -> ImapResult<()> {
        let session = self
            .session
            .as_mut()
            .ok_or_else(|| ImapError::ConnectionFailed("Not connected".to_string()))?;

        let flags_str = flags.join(" ");
        let store_query = match action {
            FlagAction::Add => format!("+FLAGS ({})", flags_str),
            FlagAction::Remove => format!("-FLAGS ({})", flags_str),
            FlagAction::Set => format!("FLAGS ({})", flags_str),
        };

        // uid_store returns a stream, we need to consume it
        let mut stream = session
            .uid_store(format!("{}", uid), &store_query)
            .await
            .map_err(|e| ImapError::OperationFailed(format!("STORE failed: {}", e)))?;

        // Consume the stream (collect all results)
        use futures::TryStreamExt;
        let _results: Vec<_> = stream.try_collect().await.map_err(|e| {
            ImapError::OperationFailed(format!("Failed to process STORE results: {}", e))
        })?;

        Ok(())
    }

    /// Expunge deleted messages from current folder
    pub async fn expunge(&mut self) -> ImapResult<()> {
        let session = self
            .session
            .as_mut()
            .ok_or_else(|| ImapError::ConnectionFailed("Not connected".to_string()))?;

        // expunge returns a stream, we need to consume it
        let mut stream = session
            .expunge()
            .await
            .map_err(|e| ImapError::OperationFailed(format!("EXPUNGE failed: {}", e)))?;

        // Consume the stream (collect all expunged sequence numbers)
        use futures::TryStreamExt;
        let _results: Vec<_> = stream.try_collect().await.map_err(|e| {
            ImapError::OperationFailed(format!("Failed to process EXPUNGE results: {}", e))
        })?;

        Ok(())
    }

    /// Append a message to a folder (for sent messages and drafts)
    ///
    /// # Arguments
    ///
    /// * `folder_name` - Name of the folder (e.g., "Sent")
    /// * `message_bytes` - Raw RFC822 message data
    pub async fn append_message(
        &mut self,
        folder_name: &str,
        message_bytes: &[u8],
    ) -> ImapResult<()> {
        let session = self
            .session
            .as_mut()
            .ok_or_else(|| ImapError::ConnectionFailed("Not connected".to_string()))?;

        session
            .append(folder_name, None, None, message_bytes)
            .await
            .map_err(|e| ImapError::OperationFailed(format!("APPEND failed: {}", e)))?;

        Ok(())
    }

    /// Copy a message to another folder (for message moves)
    ///
    /// # Arguments
    ///
    /// * `uid` - UID of the message to copy
    /// * `dest_folder` - Name of the destination folder
    pub async fn copy_message(&mut self, uid: u32, dest_folder: &str) -> ImapResult<()> {
        let session = self
            .session
            .as_mut()
            .ok_or_else(|| ImapError::ConnectionFailed("Not connected".to_string()))?;

        session
            .uid_copy(format!("{}", uid), dest_folder)
            .await
            .map_err(|e| ImapError::OperationFailed(format!("COPY failed: {}", e)))?;

        Ok(())
    }

    /// Start IDLE mode for real-time notifications
    ///
    /// Returns when server sends notification or idle is stopped
    /// NOTE: IDLE implementation is deferred to Task Group 3
    pub async fn start_idle(&mut self) -> ImapResult<()> {
        // Placeholder for IDLE implementation
        // Will be implemented in Task Group 3: Multi-Folder Sync Orchestration
        Err(ImapError::OperationFailed(
            "IDLE not yet implemented".to_string(),
        ))
    }

    /// Test IMAP connection and authentication
    ///
    /// Lightweight connection test that verifies:
    /// 1. TCP connection to the IMAP server
    /// 2. TLS handshake
    /// 3. LOGIN authentication
    ///
    /// # Arguments
    ///
    /// * `host` - IMAP server hostname
    /// * `port` - IMAP server port (default: 993 for SSL)
    /// * `email` - Email address for authentication
    /// * `password` - Password for authentication
    ///
    /// # Returns
    ///
    /// Result indicating success or detailed error message
    ///
    /// # Example (TypeScript)
    /// ```typescript
    /// const result = await invoke<string>('test_imap_connection', {
    ///   host: 'imap.gmail.com',
    ///   port: 993,
    ///   email: 'user@gmail.com',
    ///   password: 'app-password'
    /// });
    /// ```
    pub async fn test_login(host: &str, port: u16, email: &str, password: &str) -> ImapResult<()> {
        use std::time::Duration;
        use tokio::time::timeout;

        // Try to connect and authenticate with 15 second timeout
        let client = timeout(
            Duration::from_secs(15),
            Self::connect(host, port, email, password),
        )
        .await
        .map_err(|_| ImapError::Timeout)?
        .map_err(|e| e)?;

        // Logout gracefully
        drop(client);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_folder_type_inbox() {
        let folder_type = ImapClient::detect_folder_type("INBOX", &[]);
        assert_eq!(folder_type, Some("Inbox".to_string()));
    }

    #[test]
    fn test_detect_folder_type_sent() {
        assert_eq!(
            ImapClient::detect_folder_type("Sent", &[]),
            Some("Sent".to_string())
        );
        assert_eq!(
            ImapClient::detect_folder_type("Sent Items", &[]),
            Some("Sent".to_string())
        );
        assert_eq!(
            ImapClient::detect_folder_type("Sent Mail", &[]),
            Some("Sent".to_string())
        );
    }

    #[test]
    fn test_detect_folder_type_drafts() {
        assert_eq!(
            ImapClient::detect_folder_type("Drafts", &[]),
            Some("Drafts".to_string())
        );
    }

    #[test]
    fn test_detect_folder_type_trash() {
        assert_eq!(
            ImapClient::detect_folder_type("Trash", &[]),
            Some("Trash".to_string())
        );
        assert_eq!(
            ImapClient::detect_folder_type("Deleted Items", &[]),
            Some("Trash".to_string())
        );
    }

    #[test]
    fn test_detect_folder_type_archive() {
        assert_eq!(
            ImapClient::detect_folder_type("Archive", &[]),
            Some("Archive".to_string())
        );
    }

    #[test]
    fn test_detect_folder_type_custom() {
        assert_eq!(
            ImapClient::detect_folder_type("My Custom Folder", &[]),
            None
        );
    }

    #[test]
    fn test_imap_folder_structure() {
        let folder = ImapFolder {
            name: "INBOX".to_string(),
            folder_type: Some("Inbox".to_string()),
            selectable: true,
            flags: vec!["\\HasNoChildren".to_string()],
        };

        assert_eq!(folder.name, "INBOX");
        assert!(folder.selectable);
    }

    #[test]
    fn test_imap_message_structure() {
        let message = ImapMessage {
            uid: 123,
            message_id: Some("msg-abc@example.com".to_string()),
            flags: vec!["\\Seen".to_string(), "\\Flagged".to_string()],
            body: None,
        };

        assert_eq!(message.uid, 123);
        assert_eq!(message.flags.len(), 2);
    }
}
