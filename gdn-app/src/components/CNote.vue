<script setup lang="ts">
import { useNotesStore } from "@/stores/notes";
import { useUiStore } from "@/stores/ui";
import { pathAppend } from "@/util";
import {
  RiAddLine,
  RiArrowDownSLine,
  RiArrowRightSLine,
  RiCornerUpRightLine,
  RiEditLine,
  RiStickyNoteAddLine,
} from "@remixicon/vue";
import { computed, ref, watchEffect } from "vue";
import CNoteButton from "./CNoteButton.vue";
import CNoteEditor from "./CNoteEditor.vue";

const notes = useNotesStore();
const ui = useUiStore();

const props = defineProps<{
  noteId: string;
  parentId?: string;
  path: string; // From root to here
  forceOpen?: boolean;
}>();

const note = computed(() => notes.getNote(props.noteId));

const parents = computed(() => {
  let parents = notes.getParents(props.noteId);
  if (props.parentId) parents = parents.difference(new Set([props.parentId]));
  return [...parents].sort().map((id) => ({ id, text: notes.getNote(id)?.text }));
});

// Our children and their locally unique keys.
const children = computed(() => {
  if (!note.value) return [];
  const seen = new Map<string, number>();
  const children: { id: string; key: string }[] = [];
  for (const id of note.value.children) {
    const n = seen.get(id) || 0;
    seen.set(id, n + 1);
    const key = `${id}-${n}`;
    children.push({ id, key });
  }
  return children;
});

const hovering = ref(false);
const mode = ref<"editing" | "creating">();

const mayOpen = computed(() => children.value.length > 0);
const open = computed(() => mayOpen.value && ui.isOpen(props.path));

const focused = computed(() => ui.focusPath === props.path);
const hover = computed(() => hovering.value && mode.value !== "editing");

const creating = computed(() => mode.value === "creating");
const editing = computed(() => mode.value === "editing");

// Ensure we're open if we need to be.
watchEffect(() => {
  if (props.forceOpen || creating.value) ui.setOpen(props.path, true);
});

// Ensure only one editor is ever open.
watchEffect(() => {
  if (!focused.value) mode.value = undefined;
});

// Ensure we're focused when an editor is open.
watchEffect(() => {
  if (mode.value) focusOnThis();
});

function focusOnThis() {
  ui.focusPath = props.path;
}

function onClick() {
  if (!focused.value) {
    focusOnThis();
    return;
  }

  ui.toggleOpen(props.path);
}

function onEditButtonClick() {
  if (!note.value) return;
  mode.value = "editing";
}

function onEditEditorClose() {
  mode.value = undefined;
}

function onEditEditorFinish(text: string) {
  if (!note.value) return;
  note.value.text = text;
  onEditEditorClose();
}

function onCreateButtonClick() {
  if (!note.value) return;
  mode.value = "creating";
}

function onCreateEditorClose() {
  mode.value = undefined;
}

function onCreateEditorFinish(text: string) {
  if (!note.value) return;

  const newNote = notes.createNote(text);
  note.value.children.push(newNote.id);

  const lastChild = children.value.at(-1);
  if (lastChild) ui.focusPath = pathAppend(props.path, lastChild.key);

  onCreateEditorClose();
}
</script>

<template>
  <div class="flex flex-col">
    <!-- Parents -->
    <div v-if="parents.length > 0" class="pt-1">
      <div v-for="parent of parents" class="pl-6 text-xs text-neutral-400">
        <RiCornerUpRightLine size="12px" class="inline" /> {{ parent.text }}
      </div>
    </div>

    <div
      class="relative flex min-h-6 pl-1"
      :class="focused ? 'bg-neutral-200' : hover ? 'bg-neutral-100' : ''"
      @mouseenter="hovering = true"
      @mouseleave="hovering = false"
      @click="onClick"
    >
      <!-- Fold/unfold symbol -->
      <div class="flex h-6 items-center">
        <div
          class="rounded"
          :class="focused ? 'hover:bg-neutral-300' : 'hover:bg-neutral-200'"
          @click.stop="ui.toggleOpen(props.path)"
        >
          <RiArrowDownSLine v-if="open && props.forceOpen" size="16px" class="text-neutral-400" />
          <RiArrowDownSLine v-else-if="open" size="16px" />
          <RiArrowRightSLine v-else-if="mayOpen" size="16px" />
          <RiArrowRightSLine v-else size="16px" class="text-neutral-400" />
        </div>
      </div>

      <!-- Text -->
      <CNoteEditor
        v-if="editing"
        class="flex-1"
        :initialText="note?.text"
        @close="onEditEditorClose"
        @finish="onEditEditorFinish"
      />
      <div v-else-if="note && note.text.trim().length > 0" class="whitespace-pre-wrap px-1">
        {{ note.text }}
      </div>
      <div v-else-if="note" class="px-1 font-light italic">empty</div>
      <div v-else class="px-1 font-light italic">note not found</div>

      <!-- Controls -->
      <div v-if="hover" class="absolute right-0 flex h-6 items-center gap-0.5">
        <CNoteButton @click.stop="onEditButtonClick">
          <RiEditLine size="16px" />
        </CNoteButton>
        <CNoteButton @click.stop="onCreateButtonClick">
          <RiStickyNoteAddLine size="16px" />
        </CNoteButton>
      </div>
    </div>

    <!-- Children -->
    <div v-if="open && children.length > 0" class="flex flex-col pl-2">
      <CNote
        v-for="child of children"
        :key="child.key"
        :noteId="child.id"
        :parentId="props.noteId"
        :path="pathAppend(path, child.key)"
      />
    </div>

    <!-- Editor for creating new children -->
    <div v-if="creating" class="flex items-start pl-3">
      <div class="flex h-6 items-center">
        <RiAddLine size="16px" />
      </div>
      <CNoteEditor class="flex-1" @close="onCreateEditorClose" @finish="onCreateEditorFinish" />
    </div>
  </div>
</template>
