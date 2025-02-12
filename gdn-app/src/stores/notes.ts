import { Segment } from "@/lib/path";
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

  function deleteNote(id: string): void {
    for (const note of notes.value.values()) {
      note.children = note.children.filter((it) => it !== id);
    }

    notes.value.delete(id);
  }

  function addChild(id: string, childId: string, index: number): void {
    const note = getNote(id);
    if (!note) return;

    note.children.splice(index, 0, childId);
  }

  function removeChild(id: string, segment: Segment): void {
    const note = getNote(id);
    if (!note) return;

    let index = note.children.indexOf(segment.id);
    for (let i = 0; i < segment.iteration; i++) {
      index = note.children.indexOf(segment.id, index + 1);
    }

    if (index < 0) return;
    note.children.splice(index, 1);
  }

  function moveChild(fromId: string, segment: Segment, toId: string, toIndex: number): void {
    const from = getNote(fromId);
    if (!from) return;

    const to = getNote(toId);
    if (!to) return;

    // Find child index
    let fromIndex = from.children.indexOf(segment.id);
    for (let i = 0; i < segment.iteration; i++) {
      fromIndex = from.children.indexOf(segment.id, fromIndex + 1);
    }
    if (fromIndex < 0) return;

    // Fix off-by-one caused by the deletion
    if (fromId === toId && fromIndex < toIndex) toIndex--;

    from.children.splice(fromIndex, 1);
    to.children.splice(toIndex, 0, segment.id);
  }

  function clearNotes(): void {
    notes.value.clear();
  }

  return {
    getNote,
    getParents,
    createNote,
    deleteNote,
    addChild,
    removeChild,
    moveChild,
    clearNotes,
  };
});
