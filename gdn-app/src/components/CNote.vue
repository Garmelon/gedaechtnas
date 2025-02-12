<script setup lang="ts">
import { Path, Segment } from "@/lib/path";
import { useNotesStore } from "@/stores/notes";
import { useUiStore } from "@/stores/ui";
import {
  RiArrowDownSLine,
  RiArrowDownWideLine,
  RiArrowRightDoubleLine,
  RiArrowRightSLine,
  RiArrowUpWideLine,
  RiCornerDownRightLine,
  RiCornerUpRightLine,
  RiDeleteBinLine,
  RiEditLine,
  RiPushpinFill,
  RiPushpinLine,
} from "@remixicon/vue";
import { computed, ref, watchEffect } from "vue";
import CNoteButton from "./CNoteButton.vue";
import CNoteChildEditor from "./CNoteChildEditor.vue";
import CNoteEditor from "./CNoteEditor.vue";

const notes = useNotesStore();
const ui = useUiStore();

const {
  path,
  segment,
  parentId,
  parentIndex = 0,
  forceOpen = false,
} = defineProps<{
  path: Path; // From root to here
  segment: Segment;
  parentId?: string;
  parentIndex?: number;
  forceOpen?: boolean;
}>();

const id = computed(() => segment.id);
const note = computed(() => notes.getNote(id.value));

const parents = computed(() => {
  let parents = notes.getParents(id.value);
  if (parentId) parents = parents.difference(new Set([parentId]));
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

const mayOpen = computed(() => children.value.length > 0);
const open = computed(() => mayOpen.value && ui.isOpen(path));

const pinned = computed(() => ui.isPinned(segment, parentId));
const focused = computed(() => ui.isFocused(path));
const editing = computed(() => ui.isEditing(path));
const insertIndex = computed(() => ui.getInsertIndex(path));

// Ensure we're open if we need to be.
watchEffect(() => {
  if (forceOpen || editing.value) ui.setOpen(path, true);
});

function onClick(): void {
  if (!focused.value) {
    ui.focusOn(path);
    return;
  }

  ui.toggleOpen(path);
}

function onInsertSiblingBeforeButtonClick(): void {
  const parent = path.parent();
  if (!parent) return;
  ui.insertAt(parent, parentIndex);
}

function onInsertSiblingAfterButtonClick(): void {
  const parent = path.parent();
  if (!parent) return;
  ui.insertAt(parent, parentIndex + 1);
}

function onInsertChildButtonClick(): void {
  ui.insertAt(path, children.value.length);
}

function onEditButtonClick(): void {
  if (!note.value) return;
  ui.edit(path);
}

function onEditEditorClose(): void {
  ui.focus();
}

function onEditEditorFinish(text: string): void {
  if (!note.value) return;
  note.value.text = text;
  onEditEditorClose();
}

function onPinButtonClick(): void {
  if (pinned.value) ui.unsetPinned();
  else ui.setPinned(segment, parentId);
}

function onMoveButtonClick(): void {
  ui.pushAnchorId(segment.id);
}

function onInsertEditorClose(): void {
  ui.focus();
}

function onInsertEditorFinish(text: string): void {
  if (!note.value) return;

  if (insertIndex.value !== undefined) {
    const childNote = notes.createNote(text);
    note.value.children.splice(insertIndex.value, 0, childNote.id);
  }

  onInsertEditorClose();
}

function onInsertEditorMove(): void {
  if (!ui.pinned) return;
  if (insertIndex.value === undefined) return;

  if (ui.pinned.parentId) {
    notes.removeChild(ui.pinned.parentId, ui.pinned.segment);
  }

  notes.addChild(segment.id, ui.pinned.segment.id, insertIndex.value);

  onInsertEditorClose();
  ui.unsetPinned();
}

function onInsertEditorCopy(): void {
  if (!ui.pinned) return;
  if (insertIndex.value === undefined) return;

  notes.addChild(segment.id, ui.pinned.segment.id, insertIndex.value);

  onInsertEditorClose();
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
      :class="focused ? 'bg-neutral-200' : hovering ? 'bg-neutral-100' : ''"
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
      <div v-show="!editing" class="absolute right-0 flex h-6 items-center gap-0.5">
        <!-- Maybe this should be two separate fullsize buttons so they're easier to click. -->
        <!-- Maybe I should reorder/group the buttons, especially once I add a delete button. -->
        <div class="flex h-5 w-5 flex-col">
          <button
            class="flex h-0 grow select-none items-center justify-center rounded-t-sm border border-b-0 border-black bg-white p-0.5 text-black transition hover:bg-neutral-200 active:scale-95"
            :class="{ invisible: !hovering }"
            @click.stop="onInsertSiblingBeforeButtonClick"
          >
            <RiArrowUpWideLine size="16px" />
          </button>
          <button
            class="flex h-0 grow select-none items-center justify-center rounded-b-sm border border-t-0 border-black bg-white p-0.5 text-black transition hover:bg-neutral-200 active:scale-95"
            :class="{ invisible: !hovering }"
            @click.stop="onInsertSiblingAfterButtonClick"
          >
            <RiArrowDownWideLine size="16px" />
          </button>
        </div>
        <CNoteButton :visible="hovering" @click.stop="onInsertChildButtonClick">
          <RiCornerDownRightLine size="16px" />
        </CNoteButton>
        <div class="w-0.5"></div>
        <CNoteButton :visible="hovering">
          <RiDeleteBinLine size="16px" />
        </CNoteButton>
        <CNoteButton :visible="hovering" @click.stop="onEditButtonClick">
          <RiEditLine size="16px" />
        </CNoteButton>
        <div class="w-0.5"></div>
        <CNoteButton
          :visible="hovering || pinned"
          :inverted="pinned"
          @click.stop="onPinButtonClick"
        >
          <RiPushpinFill v-if="pinned" size="16px" />
          <RiPushpinLine v-else size="16px" />
        </CNoteButton>
        <CNoteButton
          :visible="hovering"
          :disabled="ui.anchorId === segment.id"
          @click.stop="onMoveButtonClick"
        >
          <RiArrowRightDoubleLine size="16px" />
        </CNoteButton>
      </div>
    </div>

    <!-- Children -->
    <div v-if="open || insertIndex !== undefined" class="flex flex-col pl-2">
      <CNoteChildEditor
        v-if="insertIndex === 0"
        :move-and-copy="ui.pinned !== undefined"
        @close="onInsertEditorClose"
        @finish="onInsertEditorFinish"
        @move="onInsertEditorMove"
        @copy="onInsertEditorCopy"
      />

      <template v-for="(child, index) of children" :key="child.fmt()">
        <CNote :path="path.concat(child)" :segment="child" :parent-id="id" :parent-index="index" />

        <CNoteChildEditor
          v-if="insertIndex === index + 1"
          :move-and-copy="ui.pinned !== undefined"
          @close="onInsertEditorClose"
          @finish="onInsertEditorFinish"
          @move="onInsertEditorMove"
          @copy="onInsertEditorCopy"
        />
      </template>
    </div>
  </div>
</template>
