/// Example Tauri command that demonstrates type-safe IPC pattern.
///
/// This command accepts a name parameter and returns a greeting string.
/// It serves as a template for creating additional commands.
///
/// # Example Usage (from frontend)
/// ```typescript
/// import { invoke } from '@tauri-apps/api/core';
/// const response = await invoke<string>('greet', { name: 'World' });
/// console.log(response); // "Hello, World! You've been greeted from Rust!"
/// ```
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_returns_greeting() {
        let result = greet("Test User");
        assert_eq!(result, "Hello, Test User! You've been greeted from Rust!");
    }

    #[test]
    fn test_greet_handles_empty_name() {
        let result = greet("");
        assert_eq!(result, "Hello, ! You've been greeted from Rust!");
    }
}
