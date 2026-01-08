//! Email protocol implementations
//!
//! This module contains implementations for email protocols (IMAP, SMTP),
//! MIME parsing, threading, and sync orchestration.

pub mod imap;
pub mod mime_parser;
pub mod smtp;
pub mod sync_orchestrator;
pub mod threading;
