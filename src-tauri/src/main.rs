#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder, LogicalPosition, Position};
use tauri_plugin_shell::ShellExt;
#[tauri::command]
fn download_video(app: tauri::AppHandle, url: String) {
    let shell = app.shell();
    let _ = shell.command("yt-dlp").args([url, "-o".to_string(), "%(title)s.%(ext)s".to_string()]).spawn();
}
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let win = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
                .title("Annas Browser").inner_size(1280.0,800.0)
                .position(Position::Logical(LogicalPosition { x:100.0, y:100.0 })).build()?;
            win.show()?; Ok(()) })
        .invoke_handler(tauri::generate_handler![download_video])
        .run(tauri::generate_context!()).expect("error");
}
