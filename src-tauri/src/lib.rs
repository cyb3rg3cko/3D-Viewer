use tauri::{Emitter, Manager};
use std::sync::Mutex;

static PENDING_FILE: Mutex<Option<String>> = Mutex::new(None);

#[tauri::command]
fn get_pending_file() -> Option<String> {
    PENDING_FILE.lock().unwrap().take()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![get_pending_file])
        .setup(|app| {
            let args: Vec<String> = std::env::args().collect();
            let file_path = args.iter().skip(1).find(|a| !a.starts_with('-')).cloned();
            if let Some(path) = file_path {
                println!("File argument received: {}", path);
                *PENDING_FILE.lock().unwrap() = Some(path);
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
