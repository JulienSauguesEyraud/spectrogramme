#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(target_os = "linux")]
            {
                use webkit2gtk::{PermissionRequestExt, WebViewExt};

                let window = app.get_webview_window("main").unwrap();
                window.with_webview(|webview| {
                    let wv = webview.inner();
                    wv.connect_permission_request(|_webview, request| {
                        request.allow();
                        true
                    });
                })?;
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}