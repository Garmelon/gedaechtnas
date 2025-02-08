import { defineStore } from "pinia";
import { ref } from "vue";

export type Note = {
  id: string;
  text: string;
  children: string[];
};

export const useNotesStore = defineStore("notes", () => {
  const notes = ref<Map<string, Note>>(new Map());

  function getNote(id: string): Note | undefined {
    return notes.value.get(id);
  }

  function createNote(text: string): Note {
    const id = crypto.randomUUID();
    notes.value.set(id, { id, text, children: [] });
    return notes.value.get(id)!; // Re-getting so returned Note is reactive
  }

  function clearNotes() {
    notes.value.clear();
  }

  return {
    getNote,
    createNote,
    clearNotes,
  };
});
