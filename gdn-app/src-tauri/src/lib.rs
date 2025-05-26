use serde_json as _; // Silence unused dependency warning
use std::sync::{Arc, Mutex};

use crate::state::AppState;

mod api;
mod state;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = Arc::new(Mutex::new(AppState::new()));

    // TODO Launch store loading task

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            api::note_child_add,
            api::note_child_move,
            api::note_child_remove,
            api::note_children_set,
            api::note_create,
            api::note_delete,
            api::note_get,
            api::note_text_set,
            api::notes_clear,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
