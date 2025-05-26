use std::sync::{Arc, Mutex};

use gdn::ids::NoteId;
use tauri::{AppHandle, Emitter, State};

use crate::{
    state::AppState,
    types::{EventNotesStoreUpdate, Note},
};

// API methods are sorted alphabetically.

fn update_if_required(state: &mut AppState, app: &AppHandle) {
    let store_id = state.store.id();
    if state.store_last_id == Some(store_id) {
        // No update necessary if the id hasn't changed
        return;
    }

    let payload = EventNotesStoreUpdate { store_id };
    app.emit("notes_store_update", payload).unwrap();
    state.store_last_id = Some(store_id)
}

#[tauri::command]
pub fn note_child_add(
    id: NoteId,
    child_id: NoteId,
    child_position: isize,
    app: AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) {
    let mut guard = state.lock().unwrap();
    guard
        .store
        .add_child_at_position(id, child_id, child_position);
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
    state: State<'_, Arc<Mutex<AppState>>>,
) {
    let mut guard = state.lock().unwrap();
    guard
        .store
        .move_child_by_id_to_position(child_id, from_id, from_iteration, to_id, to_position);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn note_child_remove(
    id: NoteId,
    child_id: NoteId,
    child_iteration: usize,
    app: AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) {
    let mut guard = state.lock().unwrap();
    guard
        .store
        .remove_child_by_id(id, child_id, child_iteration);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn note_children_set(
    id: NoteId,
    children: Vec<NoteId>,
    app: AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) {
    let mut guard = state.lock().unwrap();
    guard.store.set_children(id, children);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn note_create(text: String, app: AppHandle, state: State<'_, Arc<Mutex<AppState>>>) -> Note {
    let mut guard = state.lock().unwrap();
    let id = guard.store.create(text);
    let note = guard.store.get(id).unwrap().into();
    update_if_required(&mut guard, &app);
    note
}

#[tauri::command]
pub fn note_delete(id: NoteId, app: AppHandle, state: State<'_, Arc<Mutex<AppState>>>) {
    let mut guard = state.lock().unwrap();
    guard.store.delete(id);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn note_get(id: NoteId, state: State<'_, Arc<Mutex<AppState>>>) -> Option<Note> {
    let guard = state.lock().unwrap();
    guard.store.get(id).map(|it| it.into())
}

#[tauri::command]
pub fn note_text_set(
    id: NoteId,
    text: String,
    app: AppHandle,
    state: State<'_, Arc<Mutex<AppState>>>,
) {
    let mut guard = state.lock().unwrap();
    guard.store.set_text(id, text);
    update_if_required(&mut guard, &app);
}

#[tauri::command]
pub fn notes_clear(app: AppHandle, state: State<'_, Arc<Mutex<AppState>>>) {
    let mut guard = state.lock().unwrap();
    guard.store.clear();
    update_if_required(&mut guard, &app);
}
