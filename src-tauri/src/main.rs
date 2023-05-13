// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[tauri::command]
fn count(status: &str) -> String {
    match status {
        "initial" => "1".to_string(),
        "started" => "9".to_string(),
        _ => "0".to_string(),
    }
}

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![greet])
      .invoke_handler(tauri::generate_handler![count])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}
