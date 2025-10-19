mod message;





#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn run() {

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            message::send::send::send_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
