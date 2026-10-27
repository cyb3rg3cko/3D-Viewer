use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_single_instance::Builder::new()
                .callback(|app, args, _cwd| {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.set_focus();
                    }
                    if args.len() > 1 {
                        let file_path = args[1].clone();
                        let _ = app.emit("open-file", file_path);
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let file_path = args[1].clone();
                let app_handle = app.handle().clone();
                std::thread::spawn(move || {
                    std::thread::sleep(std::time::Duration::from_millis(800));
                    let _ = app_handle.emit("open-file", file_path);
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
