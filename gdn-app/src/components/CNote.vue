<script setup lang="ts">
import { Path, Segment } from "@/lib/path";
import { useNotesStore } from "@/stores/notes";
import { useUiStore } from "@/stores/ui";
import {
  RiAddLine,
  RiArrowDownSLine,
  RiArrowRightSLine,
  RiCornerUpRightLine,
  RiEditLine,
  RiPushpinFill,
  RiPushpinLine,
  RiStickyNoteAddLine,
} from "@remixicon/vue";
import { computed, ref, watchEffect } from "vue";
import CNoteButton from "./CNoteButton.vue";
import CNoteEditor from "./CNoteEditor.vue";

const notes = useNotesStore();
const ui = useUiStore();

const props = defineProps<{
  path: Path; // From root to here
  segment: Segment;
  parentId?: string;
  forceOpen?: boolean;
}>();

const id = computed(() => props.segment.id);
const note = computed(() => notes.getNote(id.value));

const parents = computed(() => {
  let parents = notes.getParents(id.value);
  if (props.parentId) parents = parents.difference(new Set([props.parentId]));
  return [...parents].sort().map((id) => ({ id, text: notes.getNote(id)?.text }));
});

// Our children and the
const children = computed(() => {
  if (!note.value) return [];
  const seen = new Map<string, number>();
  const children = [];
  for (const id of note.value.children) {
    const iteration = seen.get(id) ?? 0;
    seen.set(id, iteration + 1);
    children.push(new Segment(id, iteration));
  }
  return children;
});

const hovering = ref(false);
const mode = ref<"editing" | "creating">();

const mayOpen = computed(() => children.value.length > 0);
const open = computed(() => mayOpen.value && ui.isOpen(props.path));

const focused = computed(() => ui.focusPath.eq(props.path));
const pinned = computed(() => ui.isPinned(props.segment, props.parentId));
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

function focusOnThis(): void {
  ui.focusPath = props.path;
}

function onClick(): void {
  if (!focused.value) {
    focusOnThis();
    return;
  }

  ui.toggleOpen(props.path);
}

function onPinButtonClick(): void {
  if (pinned.value) ui.unsetPinned();
  else ui.setPinned(props.segment, props.parentId);
}

function onEditButtonClick(): void {
  if (!note.value) return;
  mode.value = "editing";
}

function onEditEditorClose(): void {
  mode.value = undefined;
}

function onEditEditorFinish(text: string): void {
  if (!note.value) return;
  note.value.text = text;
  onEditEditorClose();
}

function onCreateButtonClick(): void {
  if (!note.value) return;
  mode.value = "creating";
}

function onCreateEditorClose(): void {
  mode.value = undefined;
}

function onCreateEditorFinish(text: string): void {
  if (!note.value) return;

  const newNote = notes.createNote(text);
  note.value.children.push(newNote.id);

  const lastChild = children.value.at(-1);
  if (lastChild) ui.focusPath = props.path.concat(lastChild);

  onCreateEditorClose();
}
</script>

<template>
  <div class="flex flex-col">
    <!-- Parents -->
    <div v-if="parents.length > 0" class="pt-1">
      <div v-for="parent of parents" :key="parent.id" class="pl-6 text-xs text-neutral-400">
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
          @click.stop="ui.toggleOpen(path)"
        >
          <RiArrowDownSLine v-if="open && forceOpen" size="16px" class="text-neutral-400" />
          <RiArrowDownSLine v-else-if="open" size="16px" />
          <RiArrowRightSLine v-else-if="mayOpen" size="16px" />
          <RiArrowRightSLine v-else size="16px" class="text-neutral-400" />
        </div>
      </div>

      <!-- Text -->
      <CNoteEditor
        v-if="editing"
        class="flex-1"
        :initial-text="note?.text"
        @close="onEditEditorClose"
        @finish="onEditEditorFinish"
      />
      <div v-else-if="note && note.text.trim().length > 0" class="whitespace-pre-wrap px-1">
        {{ note.text }}
      </div>
      <div v-else-if="note" class="px-1 font-light italic">empty</div>
      <div v-else class="px-1 font-light italic">note not found</div>

      <!-- Controls -->
      <div v-if="hover || pinned" class="absolute right-0 flex h-6 items-center gap-0.5">
        <CNoteButton v-if="hover" @click.stop="onEditButtonClick">
          <RiEditLine size="16px" />
        </CNoteButton>
        <CNoteButton v-if="hover" @click.stop="onCreateButtonClick">
          <RiStickyNoteAddLine size="16px" />
        </CNoteButton>
        <CNoteButton :inverted="pinned" @click.stop="onPinButtonClick">
          <RiPushpinFill v-if="pinned" size="16px" />
          <RiPushpinLine v-else size="16px" />
        </CNoteButton>
      </div>
    </div>

    <!-- Children -->
    <div v-if="open && children.length > 0" class="flex flex-col pl-2">
      <CNote
        v-for="child of children"
        :key="child.fmt()"
        :path="path.concat(child)"
        :segment="child"
        :parent-id="id"
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
