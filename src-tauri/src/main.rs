use tauri::{WebviewUrl, WebviewWindowBuilder};

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let _win = WebviewWindowBuilder::new(app, "main", WebviewUrl::App("index.html".into()))
        .inner_size(1280.0, 800.0)
        .title("Annas Browser")
        .build()?;
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
