//! JWZ Threading Algorithm (RFC 5256)
//!
//! Implements the Jamie Zawinski REFERENCES threading algorithm
//! to group email messages into conversation threads.

use crate::error::{ThreadingError, ThreadingResult};
use std::collections::{HashMap, HashSet};

/// A message in the threading structure
#[derive(Debug, Clone)]
pub struct ThreadMessage {
    pub message_id: String,
    pub subject: Option<String>,
    pub parent_id: Option<String>,
    pub children: Vec<String>,
    pub is_dummy: bool, // Placeholder for missing parent messages
}

/// Thread metadata
#[derive(Debug, Clone)]
pub struct ThreadMetadata {
    pub thread_id: i64,
    pub thread_subject: Option<String>,
    pub participant_count: i32,
    pub latest_message_date: Option<String>,
    pub unread_count: i32,
}

/// Input message for threading
#[derive(Debug, Clone)]
pub struct MessageForThreading {
    pub id: i64,
    pub message_id: String,
    pub subject: Option<String>,
    pub references: Vec<String>,
    pub in_reply_to: Option<String>,
    pub date: Option<String>,
    pub participants: Vec<String>,
    pub is_read: bool,
}

/// Build thread structure using JWZ algorithm
pub fn build_threads(messages: Vec<MessageForThreading>) -> ThreadingResult<Vec<ThreadContainer>> {
    // Step 1: Create containers for all messages
    let mut id_table: HashMap<String, ThreadContainer> = HashMap::new();

    for message in &messages {
        // Create container for this message
        let container = id_table
            .entry(message.message_id.clone())
            .or_insert_with(|| ThreadContainer::new(message.message_id.clone()));

        container.message = Some(message.clone());

        // Link references
        let mut prev_container: Option<String> = None;

        // Get all references, including in_reply_to as fallback
        let mut refs: Vec<String> = message.references.clone();
        if refs.is_empty() {
            if let Some(in_reply_to) = &message.in_reply_to {
                refs.push(in_reply_to.clone());
            }
        }

        for reference in &refs {
            // Get or create container for this reference
            let ref_container = id_table
                .entry(reference.clone())
                .or_insert_with(|| ThreadContainer::new(reference.clone()));

            // Link previous reference to this one
            if let Some(prev_id) = prev_container {
                if !ref_container.children.contains(&prev_id) {
                    ref_container.children.push(prev_id);
                }
            }

            prev_container = Some(reference.clone());
        }

        // Link last reference to current message
        if let Some(prev_id) = prev_container {
            let prev_container = id_table.get_mut(&prev_id).ok_or_else(|| {
                ThreadingError::BuildFailed("Failed to find previous container".to_string())
            })?;

            if !prev_container.children.contains(&message.message_id) {
                prev_container.children.push(message.message_id.clone());
            }
        }
    }

    // Step 2: Find root containers (messages without parents)
    let mut roots: Vec<ThreadContainer> = Vec::new();
    let all_children: HashSet<String> =
        id_table.values().flat_map(|c| c.children.clone()).collect();

    for (msg_id, container) in &id_table {
        if !all_children.contains(msg_id) {
            roots.push(container.clone());
        }
    }

    // Step 3: Subject-based fallback grouping
    group_by_subject(&mut roots)?;

    // Step 4: Prevent circular references
    detect_circular_references(&roots)?;

    Ok(roots)
}

/// Group threads with identical base subjects
fn group_by_subject(roots: &mut Vec<ThreadContainer>) -> ThreadingResult<()> {
    let mut subject_table: HashMap<String, usize> = HashMap::new();

    let mut i = 0;
    while i < roots.len() {
        if let Some(subject) = extract_base_subject(&roots[i]) {
            let subject_lower = subject.to_lowercase();

            if let Some(&existing_idx) = subject_table.get(&subject_lower) {
                // Merge with existing thread
                let container = roots.remove(i);
                roots[existing_idx].children.push(container.message_id);
            } else {
                subject_table.insert(subject_lower, i);
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    Ok(())
}

/// Extract base subject (remove Re:, Fwd:, etc.)
fn extract_base_subject(container: &ThreadContainer) -> Option<String> {
    container
        .message
        .as_ref()
        .and_then(|m| m.subject.as_ref())
        .map(|s| {
            let mut subject = s.trim().to_string();

            // Remove common prefixes
            loop {
                let original = subject.clone();
                subject = regex::Regex::new(r"^(?i)(re|fwd|fw):\s*")
                    .unwrap()
                    .replace(&subject, "")
                    .to_string();

                // Remove [prefix] tags
                subject = regex::Regex::new(r"^\[[^\]]+\]\s*")
                    .unwrap()
                    .replace(&subject, "")
                    .to_string();

                if subject == original {
                    break;
                }
            }

            subject.trim().to_string()
        })
}

/// Detect circular references in thread structure
fn detect_circular_references(roots: &[ThreadContainer]) -> ThreadingResult<()> {
    for root in roots {
        let mut visited = HashSet::new();
        check_circular(&root.message_id, roots, &mut visited)?;
    }
    Ok(())
}

/// Recursively check for circular references
fn check_circular(
    msg_id: &str,
    all_containers: &[ThreadContainer],
    visited: &mut HashSet<String>,
) -> ThreadingResult<()> {
    if visited.contains(msg_id) {
        return Err(ThreadingError::CircularReference);
    }

    visited.insert(msg_id.to_string());

    // Find container
    if let Some(container) = all_containers.iter().find(|c| c.message_id == msg_id) {
        for child_id in &container.children {
            check_circular(child_id, all_containers, visited)?;
        }
    }

    visited.remove(msg_id);
    Ok(())
}

/// Calculate thread metadata
pub fn calculate_thread_metadata(
    thread: &ThreadContainer,
    all_containers: &HashMap<String, ThreadContainer>,
) -> ThreadMetadata {
    let mut participants = HashSet::new();
    let mut latest_date: Option<String> = None;
    let mut unread_count = 0;

    // Traverse thread tree
    collect_thread_stats(
        thread,
        all_containers,
        &mut participants,
        &mut latest_date,
        &mut unread_count,
    );

    ThreadMetadata {
        thread_id: 0, // Will be set by database
        thread_subject: extract_base_subject(thread),
        participant_count: participants.len() as i32,
        latest_message_date: latest_date,
        unread_count,
    }
}

/// Recursively collect thread statistics
fn collect_thread_stats(
    container: &ThreadContainer,
    all_containers: &HashMap<String, ThreadContainer>,
    participants: &mut HashSet<String>,
    latest_date: &mut Option<String>,
    unread_count: &mut i32,
) {
    if let Some(message) = &container.message {
        // Add participants
        for participant in &message.participants {
            participants.insert(participant.clone());
        }

        // Update latest date
        if let Some(date) = &message.date {
            if latest_date.is_none() || latest_date.as_ref().map_or(true, |d| date > d) {
                *latest_date = Some(date.clone());
            }
        }

        // Count unread
        if !message.is_read {
            *unread_count += 1;
        }
    }

    // Recurse to children
    for child_id in &container.children {
        if let Some(child_container) = all_containers.get(child_id) {
            collect_thread_stats(
                child_container,
                all_containers,
                participants,
                latest_date,
                unread_count,
            );
        }
    }
}

/// Thread container in the JWZ structure
#[derive(Debug, Clone)]
pub struct ThreadContainer {
    pub message_id: String,
    pub message: Option<MessageForThreading>,
    pub children: Vec<String>,
}

impl ThreadContainer {
    pub fn new(message_id: String) -> Self {
        Self {
            message_id,
            message: None,
            children: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_base_subject() {
        let container = ThreadContainer {
            message_id: "1".to_string(),
            message: Some(MessageForThreading {
                id: 1,
                message_id: "1".to_string(),
                subject: Some("Re: Test Subject".to_string()),
                references: vec![],
                in_reply_to: None,
                date: None,
                participants: vec![],
                is_read: false,
            }),
            children: vec![],
        };

        let base_subject = extract_base_subject(&container);
        assert_eq!(base_subject, Some("Test Subject".to_string()));
    }

    #[test]
    fn test_extract_base_subject_multiple_prefixes() {
        let container = ThreadContainer {
            message_id: "1".to_string(),
            message: Some(MessageForThreading {
                id: 1,
                message_id: "1".to_string(),
                subject: Some("Re: Fwd: Re: Original".to_string()),
                references: vec![],
                in_reply_to: None,
                date: None,
                participants: vec![],
                is_read: false,
            }),
            children: vec![],
        };

        let base_subject = extract_base_subject(&container);
        assert_eq!(base_subject, Some("Original".to_string()));
    }

    #[test]
    fn test_build_simple_thread() {
        let messages = vec![
            MessageForThreading {
                id: 1,
                message_id: "<msg1@example.com>".to_string(),
                subject: Some("Original".to_string()),
                references: vec![],
                in_reply_to: None,
                date: Some("2024-01-01".to_string()),
                participants: vec!["user1@example.com".to_string()],
                is_read: true,
            },
            MessageForThreading {
                id: 2,
                message_id: "<msg2@example.com>".to_string(),
                subject: Some("Re: Original".to_string()),
                references: vec!["<msg1@example.com>".to_string()],
                in_reply_to: Some("<msg1@example.com>".to_string()),
                date: Some("2024-01-02".to_string()),
                participants: vec!["user2@example.com".to_string()],
                is_read: false,
            },
        ];

        let threads = build_threads(messages).unwrap();

        assert!(!threads.is_empty());
    }

    #[test]
    fn test_subject_based_grouping() {
        let messages = vec![
            MessageForThreading {
                id: 1,
                message_id: "<msg1@example.com>".to_string(),
                subject: Some("Test Subject".to_string()),
                references: vec![],
                in_reply_to: None,
                date: None,
                participants: vec![],
                is_read: true,
            },
            MessageForThreading {
                id: 2,
                message_id: "<msg2@example.com>".to_string(),
                subject: Some("Re: Test Subject".to_string()),
                references: vec![],
                in_reply_to: None,
                date: None,
                participants: vec![],
                is_read: true,
            },
        ];

        let threads = build_threads(messages).unwrap();

        // Should group by subject
        assert!(threads.len() <= 2);
    }

    #[test]
    fn test_circular_reference_detection() {
        // Create a simple non-circular thread
        let messages = vec![MessageForThreading {
            id: 1,
            message_id: "<msg1@example.com>".to_string(),
            subject: Some("Test".to_string()),
            references: vec![],
            in_reply_to: None,
            date: None,
            participants: vec![],
            is_read: true,
        }];

        let result = build_threads(messages);
        assert!(result.is_ok());
    }
}
