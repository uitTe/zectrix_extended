mod api;
mod commands;
mod keystore;

use commands::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            // API Key 管理
            commands::api_key_status,
            commands::save_api_key,
            commands::clear_api_key,
            commands::verify_api_key,
            // 设备
            commands::list_devices,
            // 待办
            commands::list_todos,
            commands::create_todo,
            commands::update_todo,
            commands::toggle_todo,
            commands::delete_todo,
            // 显示推送
            commands::push_text,
            commands::push_structured_text,
            commands::push_image,
            commands::clear_pages,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
