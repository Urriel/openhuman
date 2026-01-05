//! Email protocol implementations
//!
//! This module contains implementations for email protocols (POP3, SMTP),
//! MIME parsing, threading, and sync orchestration.

pub mod mime_parser;
pub mod pop3;
pub mod smtp;
pub mod sync_orchestrator;
pub mod threading;
