pub mod db;
pub mod lua_api;
pub mod miko;
use crate::db::init_db;
use hypertext::{html_elements, maud, rsx, GlobalAttributes, Renderable};
use std::fmt;
use tauri::webview::WebviewWindowBuilder;
use tauri;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn retti() -> String {
    "<div>This should have replaced the thingy</div>".to_string()
}

#[tauri::command]
async fn window_open_test(app: tauri::AppHandle) -> String {
    let window =
        WebviewWindowBuilder::from_config(&app, &app.config().app.windows.get(1).unwrap().clone())
            .unwrap()
            .build()
            .unwrap();
    "<div>Did a new window open?</div>".to_string()
}

#[tauri::command]
async fn get_test_table(webview_window: tauri::WebviewWindow) -> String {
    // https://maud.lambda.xyz/
    let label = webview_window.label();
    rsx!(
            <table>
                <thead>
                    <tr>
                        <th>Song Title</th>
                        <th>Artist</th>
                        <th>Album</th>
                        <th>Year</th>
                    </tr>
                </thead>
                <tbody>
                    <tr draggable="true">
                        <td>Livin On A Prayer</td>
                        <td>Bon Jovi</td>
                        <td>{label}</td>
                        <td>1996</td>
                    </tr>
                    <tr draggable="true">
                        <td>The Fifth Angel</td>
                        <td>Beast In Black</td>
                        <td>Berserker</td>
                        <td>2017</td>
                    </tr>
                </tbody>
            </table>
    ).render().into()
}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // lua_api::demotest().unwrap();
    // lua_api::mt_test().unwrap();
    let t = tauri::Builder::default()
        //.plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![retti])
        .invoke_handler(tauri::generate_handler![window_open_test])
        .invoke_handler(tauri::generate_handler![get_test_table]);
    print!("We made it past creation");
    t.run(tauri::generate_context!())
        .expect("error while running tauri application");
    print!("We made it past running too");
}
