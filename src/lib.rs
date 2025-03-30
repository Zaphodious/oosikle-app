use std::fmt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn retti() -> String {
    "<div>This should have replaced the thingy</div>".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let t = tauri::Builder::default()
        //.plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![retti])
        ;
    print!("We made it past creation");
    t.run(tauri::generate_context!())
        .expect("error while running tauri application");
    print!("We made it past running too");
}
