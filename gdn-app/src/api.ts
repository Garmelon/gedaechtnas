import { invoke } from "@tauri-apps/api/core";
import { Note } from "./types";

export async function apiNoteChildAdd(
  id: string,
  childId: string,
  childPosition: number,
): Promise<void> {
  await invoke("note_child_add", { id, childId, childPosition });
}

export async function apiNoteChildMove(
  childId: string,
  fromId: string,
  fromIteration: number,
  toId: string,
  toPosition: number,
): Promise<void> {
  await invoke("note_child_move", { childId, fromId, fromIteration, toId, toPosition });
}

export async function apiNoteChildRemove(
  id: string,
  childId: string,
  childIteration: number,
): Promise<void> {
  await invoke("note_child_remove", { id, childId, childIteration });
}

export async function apiNoteChildrenSet(id: string, children: string[]): Promise<void> {
  await invoke("note_children_set", { id, children });
}

export async function apiNoteCreate(text: string): Promise<Note> {
  return Note.parse(await invoke("note_create", { text }));
}

export async function apiNoteDelete(id: string): Promise<void> {
  await invoke("note_delete", { id });
}

export async function apiNoteGet(id: string): Promise<Note | null> {
  return Note.nullable().parse(await invoke("note_get", { id }));
}

export async function apiNoteTextSet(id: string, text: string): Promise<void> {
  await invoke("note_text_set", { id, text });
}

export async function apiNotesClear(): Promise<void> {
  await invoke("notes_clear");
}
