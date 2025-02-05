import { defineStore } from "pinia";
import { ref } from "vue";

type Note = {
  id: string;
  text: string;
  children: string[];
};

export const useNotesStore = defineStore("notes", () => {
  const notes = ref<Map<string, Note>>(new Map());

  function addNote(text: string): Note {
    const id = crypto.randomUUID();
    notes.value.set(id, { id, text, children: [] });
    return notes.value.get(id)!; // Re-getting so returned Note is reactive
  }

  function appendChildNote(parentId: string, text: string): Note | undefined {
    const parent = notes.value.get(parentId);
    if (parent === undefined) return undefined;
    const note = addNote(text);
    parent.children.push(note.id);
    return note;
  }

  function clearNotes() {
    notes.value.clear();
  }

  return {
    notes,
    addNote,
    appendChildNote,
    clearNotes,
  };
});
