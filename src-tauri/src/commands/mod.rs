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
pub mod account_management;
pub mod email_list;
pub mod email_operations;
pub mod email_sync;
pub mod greet;
pub mod label_operations;
pub mod search;

pub use account_management::{
    add_account, delete_account, list_accounts, test_account_connection, update_account,
    update_account_sync_enabled,
};
pub use email_list::{get_message, list_folders, list_messages};
pub use email_operations::{
    bulk_archive_messages, bulk_delete_messages, bulk_mark_read, delete_draft, delete_message,
    mark_read, mark_unread, move_messages, save_draft, send_email, star_message, unstar_message,
};
pub use email_sync::{cancel_sync, get_sync_status, sync_emails};
pub use greet::greet;
pub use label_operations::{
    apply_label, archive_messages, create_label, delete_label, get_message_labels, list_labels,
    remove_label,
};
pub use search::search_messages;
