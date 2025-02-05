import { defineStore } from "pinia";
import { ref } from "vue";

export type Note = {
  id: string;
  text: string;
  children: string[];
};

export const useNotesStore = defineStore("notes", () => {
  const notes = ref<Map<string, Note>>(new Map());

  function addNote(note: Note): Note {
    notes.value.set(note.id, note);
    return notes.value.get(note.id)!; // Re-getting so returned Note is reactive
  }

  function addNewNote(text: string): Note {
    return addNote({
      id: crypto.randomUUID(),
      text,
      children: [],
    });
  }

  function appendNewChildNote(
    parentId: string,
    text: string,
  ): Note | undefined {
    const parent = notes.value.get(parentId);
    if (parent === undefined) return undefined;
    const note = addNewNote(text);
    parent.children.push(note.id);
    return note;
  }

  function clearNotes() {
    notes.value.clear();
  }

  return {
    notes,
    addNote,
    addNewNote,
    appendNewChildNote,
    clearNotes,
  };
});
