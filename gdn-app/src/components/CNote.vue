<script setup lang="ts">
import { useNotesStore } from "@/stores/notes";
import { useUiStore } from "@/stores/ui";
import { pathAppend } from "@/util";
import {
  RiArrowDownSLine,
  RiArrowRightSLine,
  RiStickyNoteAddLine,
} from "@remixicon/vue";
import { computed, ref, watchEffect } from "vue";
import CNoteButton from "./CNoteButton.vue";
import CNoteCreator from "./CNoteCreator.vue";

const notes = useNotesStore();
const ui = useUiStore();

const props = defineProps<{
  noteId: string;
  path: string; // From root to here
  forceOpen?: boolean;
}>();

const note = computed(() => notes.notes.get(props.noteId));

// Our children and their locally unique keys.
const children = computed(() => {
  if (note.value === undefined) return [];
  const seen = new Map<string, number>();
  const children: [string, string][] = [];
  for (const id of note.value.children) {
    const n = seen.get(id) || 0;
    seen.set(id, n + 1);
    const key = `${id}-${n}`;
    children.push([id, key]);
  }
  return children;
});

const mayOpen = computed(() => children.value.length > 0);
const open = computed(() => mayOpen.value && ui.openPaths.has(props.path));
const focused = computed(() => ui.focusPath === props.path);
const creating = ref(false);

// Ensure we're open if we need to be.
watchEffect(() => {
  if (props.forceOpen || creating.value) {
    if (!ui.openPaths.has(props.path)) {
      ui.openPaths.add(props.path);
    }
  }
});

// Abort creating whenever we stop being focused.
watchEffect(() => {
  if (!focused.value) creating.value = false;
});

function focusOnThis() {
  ui.focusPath = props.path;
}

function toggleOpen() {
  if (open.value) {
    ui.openPaths.delete(props.path);
  } else {
    ui.openPaths.add(props.path);
  }
}

function onClick() {
  if (!focused.value) {
    focusOnThis();
    return;
  }

  toggleOpen();
}

function onCreatorClose() {
  creating.value = false;
}

function onCreatorFinish(text: string) {
  notes.appendNewChildNote(props.noteId, text);
  ui.focusPath = pathAppend(props.path, children.value.length - 1);
  onCreatorClose();
}
</script>

<template>
  <div class="relative flex flex-col">
    <div
      class="flex min-h-6 flex-row gap-1 pl-1"
      :class="focused ? ['bg-neutral-200'] : ['hover:bg-neutral-100']"
      @click="onClick"
    >
      <!-- Fold/unfold symbol -->
      <div class="flex items-center">
        <div
          class="rounded"
          :class="focused ? ['hover:bg-neutral-300'] : ['hover:bg-neutral-200']"
          @click.stop="toggleOpen()"
        >
          <RiArrowDownSLine
            v-if="open && props.forceOpen"
            size="16px"
            class="text-neutral-400"
          />
          <RiArrowDownSLine v-else-if="open" size="16px" />
          <RiArrowRightSLine v-else-if="mayOpen" size="16px" />
          <RiArrowRightSLine v-else size="16px" class="text-neutral-400" />
        </div>
      </div>

      <!-- Text -->
      <div v-if="note && note.text.trim().length > 0">{{ note.text }}</div>
      <div v-else-if="note" class="font-light italic">empty</div>
      <div v-else class="font-light italic">note not found</div>
    </div>

    <!-- Children -->
    <div v-if="open && children.length > 0" class="flex flex-col pl-2">
      <CNote
        v-for="([noteId, key], index) in children"
        :key
        :note-id
        :path="pathAppend(path, index)"
      />
    </div>

    <div v-if="creating" class="pl-2">
      <CNoteCreator
        :parent-id="props.noteId"
        @close="onCreatorClose"
        @finish="onCreatorFinish"
      />
    </div>

    <!-- Controls -->
    <div v-if="focused" class="absolute right-0 flex h-6 items-center gap-0.5">
      <CNoteButton @click="creating = true">
        <RiStickyNoteAddLine size="16px" />
      </CNoteButton>
    </div>
  </div>
</template>
