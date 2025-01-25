use gdn::Paths;
use tauri::{AppHandle, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(app_handle: AppHandle) -> String {
    let mut lines = vec![];

    lines.push(format!(
        "      Local data dir: {}",
        app_handle.path().app_local_data_dir().unwrap().display()
    ));

    lines.push(format!(
        "  [Linux] State file: {:?}",
        Paths::on_linux().map(|p| p.state_file())
    ));

    lines.push(format!(
        "[Windows] State file: {:?}",
        Paths::on_windows().map(|p| p.state_file())
    ));

    lines.join("\n")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
