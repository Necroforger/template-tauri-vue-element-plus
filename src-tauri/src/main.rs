// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{process::Command, time::Duration, env, collections::VecDeque, fs};
use tauri::Manager;

// Clean up the cache directory used by the application.
fn cleanup(directory: impl AsRef<str>) {
    std::thread::sleep(Duration::from_millis(100));
    println!("Removing directory: {}", directory.as_ref());
    fs::remove_dir_all(directory.as_ref()).unwrap();
}

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    while !args.is_empty() {
        let cur = args.pop_front().unwrap(); 
        match cur.as_str() {
            "-cleanup" => {
                let dir = args.pop_front().expect("You need to provide a directory to cleanup");
                if !dir.contains("AppData") { panic!("This should only be invoked inside AppData. Something probably went wrong...")}
                cleanup(dir);
                return
            }
            _ => {}
        }
    }
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();

                    let window = event.window().clone();
                    let cache_dir = window.app_handle().path_resolver().app_cache_dir().unwrap().to_str().unwrap().to_owned();
                    Command::new(
                        std::env::current_exe().unwrap().to_str().unwrap() 
                    )
                    .arg("-cleanup")
                    .arg(cache_dir)
                    .spawn()
                    .expect("Could not re-invoke executable");
                
                    window.close().unwrap();
                    std::process::exit(0)
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
