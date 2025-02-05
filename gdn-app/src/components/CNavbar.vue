<script setup lang="ts">
import { useReposStore } from "@/stores/repos";
import { RiDeleteBinFill, RiNodeTree, RiSettings3Fill } from "@remixicon/vue";
import CNavbarButton from "./CNavbarButton.vue";
import CNavbarDropdown from "./CNavbarDropdown.vue";
import { useUiStore } from "@/stores/ui";
import { useNotesStore } from "@/stores/notes";

const repos = useReposStore();
const notes = useNotesStore();
const ui = useUiStore();

function createSomeNodes() {
  notes.clearNotes();

  const root = notes.addNote("root");
  ui.anchor = root.id;

  notes.appendChildNote(root.id, "n1")!;
  const n2 = notes.appendChildNote(root.id, "n2")!;
  notes.appendChildNote(root.id, "n3")!;
  notes.appendChildNote(root.id, "n4")!;
  const n5 = notes.appendChildNote(root.id, "n5")!;

  notes.appendChildNote(n2.id, "n2n1")!;
  notes.appendChildNote(n2.id, "n2n2")!;
  notes.appendChildNote(n2.id, "n2n3")!;

  n5.children.push("NaN (Not a Note)");
}
</script>

<template>
  <div class="flex gap-1 bg-black p-2 text-white">
    <!-- The div is necessary because of the delete button, otherwise I could
    just use justify-between -->
    <div class="mr-auto overflow-hidden">
      <CNavbarDropdown />
    </div>

    <!-- Temporary button for testing -->
    <CNavbarButton @click="createSomeNodes">
      <RiNodeTree size="16px" class="inline" />
    </CNavbarButton>

    <!-- Temporary delete button until I add proper repo settings -->
    <CNavbarButton
      v-show="repos.selectedRepo !== undefined"
      @click="repos.removeRepo(repos.selectedRepoId)"
    >
      <RiDeleteBinFill size="16px" class="inline" />
    </CNavbarButton>

    <!-- Nothing hooked up yet -->
    <CNavbarButton>
      <RiSettings3Fill size="16px" class="inline" />
    </CNavbarButton>
  </div>
</template>
