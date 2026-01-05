//! OS Keychain integration for secure credential storage
//!
//! This module provides secure storage for email account credentials using the
//! operating system's native keychain/credential manager:
//! - macOS: Keychain
//! - Windows: Credential Manager
//! - Linux: Secret Service API

use crate::error::{KeychainError, KeychainResult};
use keyring::Entry;

const SERVICE_NAME: &str = "openhuman";

/// Build a keychain key for an email account
///
/// Format: `openhuman:<email>`
fn build_key(email: &str) -> String {
    format!("{}:{}", SERVICE_NAME, email)
}

/// Store credentials for an email account in the OS keychain
///
/// # Arguments
///
/// * `email` - The email address (used as part of the key)
/// * `password` - The password to store
///
/// # Returns
///
/// Ok(()) on success, KeychainError on failure
///
/// # Example
///
/// ```rust
/// use openhuman_lib::keychain::store_credentials;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// store_credentials("user@example.com", "secret_password")?;
/// # Ok(())
/// # }
/// ```
pub fn store_credentials(email: &str, password: &str) -> KeychainResult<()> {
    let key = build_key(email);
    let entry =
        Entry::new(SERVICE_NAME, &key).map_err(|e| KeychainError::AccessFailed(e.to_string()))?;

    entry
        .set_password(password)
        .map_err(|e| KeychainError::StoreFailed(e.to_string()))?;

    Ok(())
}

/// Retrieve credentials for an email account from the OS keychain
///
/// # Arguments
///
/// * `email` - The email address (used to lookup the key)
///
/// # Returns
///
/// The stored password on success, KeychainError if not found or access fails
///
/// # Example
///
/// ```rust
/// use openhuman_lib::keychain::get_credentials;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let password = get_credentials("user@example.com")?;
/// println!("Retrieved password (length: {})", password.len());
/// # Ok(())
/// # }
/// ```
pub fn get_credentials(email: &str) -> KeychainResult<String> {
    let key = build_key(email);
    let entry =
        Entry::new(SERVICE_NAME, &key).map_err(|e| KeychainError::AccessFailed(e.to_string()))?;

    entry.get_password().map_err(|e| match e {
        keyring::Error::NoEntry => {
            KeychainError::CredentialsNotFound(format!("No credentials found for {}", email))
        }
        _ => KeychainError::AccessFailed(e.to_string()),
    })
}

/// Delete credentials for an email account from the OS keychain
///
/// # Arguments
///
/// * `email` - The email address (used to lookup the key)
///
/// # Returns
///
/// Ok(()) on success, KeychainError on failure
///
/// # Example
///
/// ```rust
/// use openhuman_lib::keychain::delete_credentials;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// delete_credentials("user@example.com")?;
/// # Ok(())
/// # }
/// ```
pub fn delete_credentials(email: &str) -> KeychainResult<()> {
    let key = build_key(email);
    let entry =
        Entry::new(SERVICE_NAME, &key).map_err(|e| KeychainError::AccessFailed(e.to_string()))?;

    entry
        .delete_credential()
        .map_err(|e| KeychainError::DeleteFailed(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_retrieve_credentials() {
        let email = "test@example.com";
        let password = "super_secret_password";

        // Clean up any existing credentials
        let _ = delete_credentials(email);

        // Store credentials
        let store_result = store_credentials(email, password);
        assert!(
            store_result.is_ok(),
            "Should store credentials successfully"
        );

        // Retrieve credentials
        let retrieved = get_credentials(email);
        assert!(
            retrieved.is_ok(),
            "Should retrieve credentials successfully"
        );
        assert_eq!(
            retrieved.unwrap(),
            password,
            "Retrieved password should match stored password"
        );

        // Clean up
        let _ = delete_credentials(email);
    }

    #[test]
    fn test_get_nonexistent_credentials() {
        let email = "nonexistent@example.com";

        // Ensure no credentials exist
        let _ = delete_credentials(email);

        // Try to retrieve non-existent credentials
        let result = get_credentials(email);
        assert!(
            result.is_err(),
            "Should fail to retrieve non-existent credentials"
        );

        match result {
            Err(KeychainError::CredentialsNotFound(_)) => {
                // Expected error type
            }
            _ => panic!("Expected CredentialsNotFound error"),
        }
    }

    #[test]
    fn test_delete_credentials() {
        let email = "delete_test@example.com";
        let password = "temp_password";

        // Clean up any existing credentials
        let _ = delete_credentials(email);

        // Store and verify credentials exist
        store_credentials(email, password).expect("Should store credentials");
        assert!(get_credentials(email).is_ok(), "Credentials should exist");

        // Delete credentials
        let delete_result = delete_credentials(email);
        assert!(
            delete_result.is_ok(),
            "Should delete credentials successfully"
        );

        // Verify credentials are gone
        let result = get_credentials(email);
        assert!(result.is_err(), "Credentials should no longer exist");
    }

    #[test]
    fn test_key_format() {
        let email = "user@example.com";
        let key = build_key(email);
        assert_eq!(
            key, "openhuman:user@example.com",
            "Key format should be correct"
        );
    }
}
