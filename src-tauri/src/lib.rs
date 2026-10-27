use tauri::{Emitter, Manager};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let args: Vec<String> = std::env::args().collect();
            if args.len() > 1 {
                let file_path = args[1].clone();
                println!("File argument received: {}", file_path);
                let app_handle = app.handle().clone();
                std::thread::spawn(move || {
                    // Emit multiple times to catch the frontend whenever it's ready
                    for delay in [500, 1500, 3000, 5000] {
                        std::thread::sleep(std::time::Duration::from_millis(delay));
                        println!("Emitting open-file event (delay {}ms)", delay);
                        let _ = app_handle.emit("open-file", file_path.clone());
                    }
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
