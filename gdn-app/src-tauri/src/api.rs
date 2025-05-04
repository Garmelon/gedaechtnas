use std::sync::{Arc, Mutex};

use gdn::{ids::NoteId, store::Store};
use tauri::{AppHandle, Emitter, State};

use crate::types::{EventNotesStoreUpdate, Note};

// API methods are sorted alphabetically.

fn update_if_required(store: &mut Store, app: &AppHandle) {
    if let Some(store_id) = store.needs_update() {
        let payload = EventNotesStoreUpdate { store_id };
        app.emit("notes_store_update", payload).unwrap();
        store.update();
    }
}

#[tauri::command]
pub fn note_child_add(
    id: NoteId,
    child_id: NoteId,
    child_position: isize,
    app: AppHandle,
    store: State<'_, Arc<Mutex<Store>>>,
) {
    let mut guard = store.lock().unwrap();
    guard.add_child_at_position(id, child_id, child_position);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn note_child_move(
    child_id: NoteId,
    from_id: NoteId,
    from_iteration: usize,
    to_id: NoteId,
    to_position: isize,
    app: AppHandle,
    store: State<'_, Arc<Mutex<Store>>>,
) {
    let mut guard = store.lock().unwrap();
    guard.move_child_by_id_to_position(child_id, from_id, from_iteration, to_id, to_position);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn note_child_remove(
    id: NoteId,
    child_id: NoteId,
    child_iteration: usize,
    app: AppHandle,
    store: State<'_, Arc<Mutex<Store>>>,
) {
    let mut guard = store.lock().unwrap();
    guard.remove_child_by_id(id, child_id, child_iteration);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn note_children_set(
    id: NoteId,
    children: Vec<NoteId>,
    app: AppHandle,
    store: State<'_, Arc<Mutex<Store>>>,
) {
    let mut guard = store.lock().unwrap();
    guard.set_children(id, children);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn note_create(text: String, app: AppHandle, store: State<'_, Arc<Mutex<Store>>>) -> Note {
    let mut guard = store.lock().unwrap();
    let id = guard.create(text);
    update_if_required(&mut guard, &app);
    guard.get(id).unwrap().into()
}

#[tauri::command]
pub fn note_delete(id: NoteId, app: AppHandle, store: State<'_, Arc<Mutex<Store>>>) {
    let mut guard = store.lock().unwrap();
    guard.delete(id);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn note_get(id: NoteId, store: State<'_, Arc<Mutex<Store>>>) -> Option<Note> {
    let guard = store.lock().unwrap();
    guard.get(id).map(|it| it.into())
}

#[tauri::command]
pub fn note_text_set(
    id: NoteId,
    text: String,
    app: AppHandle,
    store: State<'_, Arc<Mutex<Store>>>,
) {
    let mut guard = store.lock().unwrap();
    guard.set_text(id, text);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn notes_clear(app: AppHandle, store: State<'_, Arc<Mutex<Store>>>) {
    let mut guard = store.lock().unwrap();
    guard.clear();
    update_if_required(&mut guard, &app);
}
