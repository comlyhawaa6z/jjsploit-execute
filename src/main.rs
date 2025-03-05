use std::sync::{Arc, Mutex};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, Manager};

#[derive(serde::Deserialize)]
struct Script {
    code: String,
}

struct AppState {
    scripts: Vec<Script>,
}

fn main() {
    let state = Arc::new(Mutex::new(AppState { scripts: Vec::new() }));
    let tray_menu = SystemTray::new().with_menu(tauri::SystemTrayMenu::new().add_item(CustomMenuItem::new("quit", "Quit")));
    
    tauri::Builder::default()
        .system_tray(tray_menu)
        .on_system_tray_event(|app, event| {
            if let SystemTrayEvent::MenuItemClick { id, .. } = event {
                if id.as_str() == "quit" {
                    std::process::exit(0);
                }
            }
        })
        .manage(state)
        .invoke_handler(tauri::generate_handler![execute_script])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn execute_script(script_code: String) {
    let output = std::process::Command::new("cmd")
        .arg("/C")
        .arg(format!("start cmd /k {}", script_code))
        .output()
        .expect("Failed to execute script");
    println!("Script executed: {:?}", output);
}