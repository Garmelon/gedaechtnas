import {
  apiNoteChildAdd,
  apiNoteChildMove,
  apiNoteChildRemove,
  apiNoteChildrenSet,
  apiNoteCreate,
  apiNoteDelete,
  apiNoteGet,
  apiNotesClear,
  apiNoteTextSet,
} from "@/api";
import { Segment } from "@/lib/path";
import { EventNoteStoreUpdate } from "@/types";
import { listen } from "@tauri-apps/api/event";
import { defineStore } from "pinia";
import { ref } from "vue";

export interface Note {
  readonly id: string;
  readonly text: string;
  readonly children: readonly string[];
  readonly parents: ReadonlySet<string>;
}

export const useNotesStore = defineStore("notes", () => {
  const storeId = ref<number>();

  async function initialize(): Promise<void> {
    await listen("notes_store_update", (ev) => {
      const data = EventNoteStoreUpdate.parse(ev.payload);
      if (storeId.value === undefined || storeId.value < data.storeId) storeId.value = data.storeId;
    });
  }

  function dependOnStoreId(): void {
    // eslint-disable-next-line @typescript-eslint/no-unused-expressions
    storeId.value;
  }

  async function getNote(id: string): Promise<Note | null> {
    dependOnStoreId();
    return apiNoteGet(id);
  }

  async function createNote(text: string): Promise<Note> {
    dependOnStoreId();
    return apiNoteCreate(text);
  }

  async function deleteNote(id: string): Promise<void> {
    return apiNoteDelete(id);
  }

  async function setText(id: string, text: string): Promise<void> {
    return apiNoteTextSet(id, text);
  }

  async function setChildren(id: string, children: string[]): Promise<void> {
    return apiNoteChildrenSet(id, children);
  }

  async function addChild(id: string, childId: string, childPosition: number): Promise<void> {
    return apiNoteChildAdd(id, childId, childPosition);
  }

  async function removeChild(id: string, segment: Segment): Promise<void> {
    return apiNoteChildRemove(id, segment.id, segment.iteration);
  }

  async function moveChild(
    fromId: string,
    segment: Segment,
    toId: string,
    toPosition: number,
  ): Promise<void> {
    return apiNoteChildMove(segment.id, fromId, segment.iteration, toId, toPosition);
  }

  async function clearNotes(): Promise<void> {
    return apiNotesClear();
  }

  return {
    storeId,
    initialize,
    getNote,
    createNote,
    deleteNote,
    setText,
    setChildren,
    addChild,
    removeChild,
    moveChild,
    clearNotes,
  };
});
