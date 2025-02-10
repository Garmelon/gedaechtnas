import { defineStore } from "pinia";
import { computed, ref } from "vue";

export interface Note {
  readonly id: string;
  text: string;
  children: string[];
}

export const useNotesStore = defineStore("notes", () => {
  const notes = ref<Map<string, Note>>(new Map());

  const parents = computed(() => {
    const result = new Map<string, Set<string>>();
    for (const note of notes.value.values()) {
      for (const childId of note.children) {
        const parents = result.get(childId) ?? new Set();
        result.set(childId, parents);
        parents.add(note.id);
      }
    }
    return result;
  });

  function getNote(id: string): Note | undefined {
    return notes.value.get(id);
  }

  function getParents(id: string): ReadonlySet<string> {
    return parents.value.get(id) ?? new Set();
  }

  function createNote(text: string): Note {
    const id = crypto.randomUUID();
    notes.value.set(id, { id, text, children: [] });
    // eslint-disable-next-line @typescript-eslint/no-non-null-assertion
    return notes.value.get(id)!; // Re-getting so returned Note is reactive
  }

  function clearNotes(): void {
    notes.value.clear();
  }

  return {
    getNote,
    getParents,
    createNote,
    clearNotes,
  };
});
