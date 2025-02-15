<script setup lang="ts">
import { Note, useNotesStore } from "@/stores/notes";
import { useReposStore } from "@/stores/repos";
import { useUiStore } from "@/stores/ui";
import {
  RiArrowLeftDoubleLine,
  RiDeleteBinFill,
  RiNodeTree,
  RiSettings3Fill,
} from "@remixicon/vue";
import { onMounted } from "vue";
import CNavbarButton from "./CNavbarButton.vue";
import CNavbarDropdown from "./CNavbarDropdown.vue";

const repos = useReposStore();
const notes = useNotesStore();
const ui = useUiStore();

async function mkNote(text: string, ...children: string[]): Promise<Note> {
  const note = await notes.createNote(text);
  for (const child of children) await notes.addChild(note.id, child, -1);
  return note;
}

async function createSomeNotes(): Promise<void> {
  await notes.clearNotes();

  const n2n1 = await mkNote("n2n1");
  const n2n2 = await mkNote("n2n2");
  const n2n3 = await mkNote("n2n3");

  const n1 = await mkNote("n1");
  const n2 = await mkNote("n2", n2n1.id, n2n2.id, n2n3.id);
  const n3 = await mkNote("n3", n2n1.id);
  const n4 = await mkNote("n4");
  const n5 = await mkNote("n5", "n0000000000000000");

  const root = await mkNote("root", n1.id, n2.id, n3.id, n4.id, n5.id, n2.id);

  ui.pushAnchorId(root.id);
  ui.history = [];

  // Shuffle children of root
  const rootChildren = (await notes.getNote(root.id))?.children ?? [];
  await notes.setChildren(
    root.id,
    rootChildren
      .map((it) => ({ it, rand: Math.random() }))
      .sort((a, b) => a.rand - b.rand)
      .map(({ it }) => it),
  );
}

onMounted(async () => {
  await createSomeNotes();
});
</script>

<template>
  <div class="flex gap-1 bg-black p-2 text-white">
    <!-- The div is necessary because of the delete button, otherwise I could
    just use justify-between -->
    <div class="mr-auto overflow-hidden">
      <CNavbarDropdown />
    </div>

    <!-- Temporary back button -->
    <CNavbarButton title="Go back" @click="ui.popAnchorId()">
      <RiArrowLeftDoubleLine size="16px" class="inline" />
    </CNavbarButton>

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
