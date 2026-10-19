use std::path::PathBuf;
use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_single_instance::init(|app, args, _cwd| {
            // Existing instance: focus window and emit file path if present
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.show();
                let _ = window.set_focus();
            }
            if args.len() > 1 {
                let path = args[1].clone();
                let _ = app.emit("open-file", path);
            }
        }))
        .setup(|app| {
            // Cold start: check command-line arguments (Windows/Linux)
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let path = args[1].clone();
                // Emit after a short delay so the frontend is ready
                let app_handle = app.handle().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    let _ = app_handle.emit("open-file", path);
                });
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|app, event| {
            // Mobile / macOS Opened event
            #[cfg(any(target_os = "macos", target_os = "ios", target_os = "android"))]
            if let tauri::RunEvent::Opened { urls } = event {
                for url in urls {
                    if let Ok(path) = url.to_file_path() {
                        let _ = app.emit("open-file", path.to_string_lossy().to_string());
                    }
                }
            }
        });
}
