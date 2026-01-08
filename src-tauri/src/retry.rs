//! Retry utilities with exponential backoff
//!
//! This module provides retry logic for handling transient failures in network
//! operations and external service calls.

use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts (default: 5)
    pub max_attempts: u32,
    /// Initial delay in milliseconds (default: 1000ms = 1s)
    pub initial_delay_ms: u64,
    /// Whether to use exponential backoff (default: true)
    pub exponential_backoff: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_delay_ms: 1000,
            exponential_backoff: true,
        }
    }
}

/// Retry an async operation with exponential backoff
///
/// The delay pattern is: 1s, 2s, 4s, 8s, 16s (for max 5 attempts)
///
/// # Arguments
///
/// * `operation` - Async function to retry
/// * `max_attempts` - Maximum number of attempts (including initial try)
///
/// # Returns
///
/// Result from the operation if successful within max_attempts, otherwise the last error
///
/// # Example
///
/// ```rust,no_run
/// use openhuman_lib::retry::retry_with_backoff;
///
/// async fn example() {
///     async fn fetch_data() -> Result<String, String> {
///         // Network operation that might fail
///         Ok("data".to_string())
///     }
///
///     let result = retry_with_backoff(|| fetch_data(), 5).await;
/// }
/// ```
pub async fn retry_with_backoff<F, Fut, T, E>(operation: F, max_attempts: u32) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let config = RetryConfig {
        max_attempts,
        ..Default::default()
    };
    retry_with_config(operation, config).await
}

/// Retry an async operation with custom configuration
///
/// # Arguments
///
/// * `operation` - Async function to retry
/// * `config` - Retry configuration
///
/// # Returns
///
/// Result from the operation if successful, otherwise the last error
pub async fn retry_with_config<F, Fut, T, E>(mut operation: F, config: RetryConfig) -> Result<T, E>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    let mut attempt = 0;
    let mut delay_ms = config.initial_delay_ms;

    loop {
        attempt += 1;

        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if attempt >= config.max_attempts {
                    return Err(error);
                }

                // Wait before next retry
                sleep(Duration::from_millis(delay_ms)).await;

                // Exponential backoff: double the delay for next attempt
                if config.exponential_backoff {
                    delay_ms *= 2;
                }
            }
        }
    }
}

/// Check if an error should trigger a retry (vs immediate failure)
///
/// Authentication errors should not be retried, as they require user intervention.
/// Network errors should be retried with backoff.
pub fn should_retry<E>(error: &E, attempt: u32, max_attempts: u32) -> bool
where
    E: std::fmt::Display,
{
    if attempt >= max_attempts {
        return false;
    }

    let error_str = error.to_string().to_lowercase();

    // Don't retry authentication failures
    if error_str.contains("authentication") || error_str.contains("unauthorized") {
        return false;
    }

    // Retry network-related errors
    error_str.contains("timeout")
        || error_str.contains("connection")
        || error_str.contains("network")
        || error_str.contains("dns")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_succeeds_on_first_attempt() {
        let result = retry_with_backoff(|| async { Ok::<i32, String>(42) }, 5).await;

        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn test_retry_succeeds_after_failures() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let result = retry_with_backoff(
            || {
                let c = counter_clone.clone();
                async move {
                    let attempt = c.fetch_add(1, Ordering::SeqCst);
                    if attempt < 2 {
                        Err("Temporary failure".to_string())
                    } else {
                        Ok(42)
                    }
                }
            },
            5,
        )
        .await;

        assert_eq!(result, Ok(42));
        assert_eq!(counter.load(Ordering::SeqCst), 3); // 2 failures + 1 success
    }

    #[tokio::test]
    async fn test_retry_fails_after_max_attempts() {
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let result = retry_with_backoff(
            || {
                let c = counter_clone.clone();
                async move {
                    c.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, String>("Persistent failure".to_string())
                }
            },
            3,
        )
        .await;

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_exponential_backoff_timing() {
        let start = tokio::time::Instant::now();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let config = RetryConfig {
            max_attempts: 3,
            initial_delay_ms: 100, // Use shorter delays for testing
            exponential_backoff: true,
        };

        let _result = retry_with_config(
            || {
                let c = counter_clone.clone();
                async move {
                    c.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, String>("Failure".to_string())
                }
            },
            config,
        )
        .await;

        let elapsed = start.elapsed();

        // Should have delays of 100ms + 200ms = 300ms minimum
        assert!(
            elapsed >= Duration::from_millis(300),
            "Expected at least 300ms, got {:?}",
            elapsed
        );
    }

    #[test]
    fn test_should_retry_network_errors() {
        assert!(should_retry(&"Connection timeout", 1, 5));
        assert!(should_retry(&"Network unreachable", 1, 5));
        assert!(should_retry(&"DNS resolution failed", 2, 5));
    }

    #[test]
    fn test_should_not_retry_auth_errors() {
        assert!(!should_retry(&"Authentication failed", 1, 5));
        assert!(!should_retry(&"Unauthorized access", 1, 5));
    }

    #[test]
    fn test_should_not_retry_after_max_attempts() {
        assert!(!should_retry(&"Connection timeout", 5, 5));
        assert!(!should_retry(&"Network error", 6, 5));
    }
}
