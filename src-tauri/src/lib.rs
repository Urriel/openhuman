// Command handlers module
mod commands;

// Core modules
pub mod db;
pub mod email;
pub mod error;
pub mod keychain;
pub mod retry;

// Tests
#[cfg(test)]
mod tests;

use commands::{
    add_account, apply_label, archive_messages, bulk_archive_messages, bulk_delete_messages,
    bulk_mark_read, create_label, delete_account, delete_draft, delete_label, delete_message,
    get_message, get_message_labels, greet, list_accounts, list_folders, list_labels,
    list_messages, mark_read, mark_unread, move_messages, remove_label, save_draft,
    search_messages, send_email, star_message, test_account_connection, unstar_message,
    update_account, update_account_sync_enabled,
};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Initialize database synchronously before app window opens
            let app_handle = app.handle().clone();

            tauri::async_runtime::block_on(async move {
                // Get app data directory
                let app_dir = app_handle
                    .path()
                    .app_data_dir()
                    .expect("Failed to get app data directory");

                // Create directory if it doesn't exist
                std::fs::create_dir_all(&app_dir).expect("Failed to create app data directory");

                // Initialize database
                let db_path = app_dir.join("openhuman.db");
                let db_url = format!("sqlite://{}", db_path.display());

                let pool = db::init_db(&db_url)
                    .await
                    .expect("Failed to initialize database");

                db::set_pool(pool).expect("Failed to set database pool");

                println!("Database initialized at: {}", db_path.display());
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            // Account management
            add_account,
            list_accounts,
            delete_account,
            update_account,
            update_account_sync_enabled,
            test_account_connection,
            // Label operations
            create_label,
            list_labels,
            delete_label,
            apply_label,
            remove_label,
            get_message_labels,
            archive_messages,
            // Email list operations
            list_messages,
            list_folders,
            get_message,
            // Email operations
            mark_read,
            mark_unread,
            star_message,
            unstar_message,
            delete_message,
            move_messages,
            save_draft,
            delete_draft,
            search_messages,
            send_email,
            bulk_mark_read,
            bulk_archive_messages,
            bulk_delete_messages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
