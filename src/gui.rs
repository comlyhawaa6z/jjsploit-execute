use tauri::{Window, Manager};

pub fn create_gui(window: &Window) {
    window.set_title("JJSploit Executor").unwrap();
    window.set_resizable(false).unwrap();
    window.set_size(800.0, 600.0).unwrap();
}

pub fn display_message(window: &Window, message: &str) {
    window.emit("display-message", message).unwrap();
}