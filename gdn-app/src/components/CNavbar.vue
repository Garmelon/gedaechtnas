<script setup lang="ts">
import { Note, useNotesStore } from "@/stores/notes";
import { useReposStore } from "@/stores/repos";
import { useUiStore } from "@/stores/ui";
import { RiDeleteBinFill, RiNodeTree, RiSettings3Fill } from "@remixicon/vue";
import { onMounted } from "vue";
import CNavbarButton from "./CNavbarButton.vue";
import CNavbarDropdown from "./CNavbarDropdown.vue";

const repos = useReposStore();
const notes = useNotesStore();
const ui = useUiStore();

function mkNote(id: string, ...children: string[]): Note {
  return notes.addNote({ id, text: id, children });
}

function createSomeNotes() {
  notes.clearNotes();

  const n2n1 = mkNote("n2n1");
  const n2n2 = mkNote("n2n2");
  const n2n3 = mkNote("n2n3");

  const n1 = mkNote("n1");
  const n2 = mkNote("n2", n2n1.id, n2n2.id, n2n3.id);
  const n3 = mkNote("n3");
  const n4 = mkNote("n4");
  const n5 = mkNote("n5", "NaN (not a note)");

  const root = mkNote("root", n1.id, n2.id, n3.id, n4.id, n5.id, n2.id);

  ui.anchorId = root.id;

  // Shuffle children of root
  root.children = root.children
    .map((it) => ({ it, rand: Math.random() }))
    .sort((a, b) => a.rand - b.rand)
    .map(({ it }) => it);
}

onMounted(() => createSomeNotes());
</script>

<template>
  <div class="flex gap-1 bg-black p-2 text-white">
    <!-- The div is necessary because of the delete button, otherwise I could
    just use justify-between -->
    <div class="mr-auto overflow-hidden">
      <CNavbarDropdown />
    </div>

    <!-- Temporary button for testing -->
    <CNavbarButton title="Create dummy note tree" @click="createSomeNotes">
      <RiNodeTree size="16px" class="inline" />
    </CNavbarButton>

    <!-- Temporary delete button until I add proper repo settings -->
    <CNavbarButton
      v-show="repos.selectedRepo !== undefined"
      title="Delete repo"
      @click="repos.removeRepo(repos.selectedRepoId)"
    >
      <RiDeleteBinFill size="16px" class="inline" />
    </CNavbarButton>

    <!-- Nothing hooked up yet -->
    <CNavbarButton title="Settings">
      <RiSettings3Fill size="16px" class="inline" />
    </CNavbarButton>
  </div>
</template>
