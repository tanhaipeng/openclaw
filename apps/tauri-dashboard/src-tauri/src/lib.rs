// Tauri 2 entry point. The main window loads the OpenClaw dashboard at
// http://127.0.0.1:18789/ with gateway token from config when present.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod config;

use tauri::WebviewUrl;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let url = config::dashboard_url_with_token();
            let webview_url = url.parse::<url::Url>().unwrap_or_else(|_| {
                "http://127.0.0.1:18789/".parse().expect("hardcoded base URL is valid")
            });
            tauri::WebviewWindowBuilder::new(app, "main", WebviewUrl::External(webview_url))
                .title("OpenClaw Dashboard")
                .inner_size(1200.0, 800.0)
                .resizable(true)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running OpenClaw Dashboard");
}
