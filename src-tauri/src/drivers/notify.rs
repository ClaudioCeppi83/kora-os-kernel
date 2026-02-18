use tauri::{AppHandle, Runtime};
use tauri_plugin_notification::NotificationExt;

#[tauri::command]
pub fn send_notification<R: Runtime>(app: AppHandle<R>, title: String, body: String) {
    let _ = app.notification().builder().title(title).body(body).show();
}
