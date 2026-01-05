/// Command module - houses all Tauri command handlers
///
/// # Adding New Commands
///
/// To add a new command:
/// 1. Create a new file in `src-tauri/src/commands/` (e.g., `my_command.rs`)
/// 2. Define your command function with the `#[tauri::command]` attribute
/// 3. Add `pub mod my_command;` to this file
/// 4. Export the command: `pub use my_command::my_command;`
/// 5. Register it in `lib.rs` using `tauri::generate_handler![greet, my_command]`
/// 6. Create corresponding TypeScript types in `src/types/commands.ts`
pub mod greet;

pub use greet::greet;
