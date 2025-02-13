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
  RiLinkUnlinkM,
  RiPushpinFill,
  RiPushpinLine,
} from "@remixicon/vue";
import { computed, ref, watchEffect } from "vue";
import CNoteButton from "./CNoteButton.vue";
import CNoteChildEditor from "./CNoteChildEditor.vue";
import CNoteEditor from "./CNoteEditor.vue";
import { computedAsync } from "@vueuse/core";

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
const note = computedAsync(async () => await notes.getNote(id.value));

const parents = computedAsync(async () => {
  const result = [];
  for (const parent of note.value?.parents ?? new Set()) {
    if (parentId !== undefined && parent === parentId) continue;
    result.push({
      id: parent,
      text: (await notes.getNote(parent))?.text,
    });
  }
  result.sort((a, b) => (a.id < b.id ? -1 : a.id > b.id ? 1 : 0));
  return result;
});

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

async function onDeleteButtonClick(): Promise<void> {
  await notes.deleteNote(segment.id);
}

async function onUnlinkButtonClick(): Promise<void> {
  if (parentId === undefined) return;
  await notes.removeChild(parentId, segment);
}

function onEditButtonClick(): void {
  if (!note.value) return;
  ui.edit(path);
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

function onPinButtonClick(): void {
  if (pinned.value) ui.unsetPinned();
  else ui.setPinned(segment, parentId);
}

function onMoveButtonClick(): void {
  ui.pushAnchorId(segment.id);
}

function onEditEditorClose(): void {
  ui.focus();
}

async function onEditEditorFinish(text: string): Promise<void> {
  if (!note.value) return;
  await notes.setText(segment.id, text);
  onEditEditorClose();
}

function onInsertEditorClose(): void {
  ui.focus();
}

async function onInsertEditorFinish(text: string): Promise<void> {
  if (!note.value) return;

  if (insertIndex.value !== undefined) {
    const child = await notes.createNote(text);
    await notes.addChild(segment.id, child.id, insertIndex.value);
  }

  onInsertEditorClose();
}

async function onInsertEditorMove(): Promise<void> {
  if (!ui.pinned) return;
  if (insertIndex.value === undefined) return;

  if (ui.pinned.parentId) {
    await notes.moveChild(ui.pinned.parentId, ui.pinned.segment, segment.id, insertIndex.value);
  } else {
    await notes.addChild(segment.id, ui.pinned.segment.id, insertIndex.value);
  }

  onInsertEditorClose();
  ui.unsetPinned();
}

async function onInsertEditorCopy(): Promise<void> {
  if (!ui.pinned) return;
  if (insertIndex.value === undefined) return;

  await notes.addChild(segment.id, ui.pinned.segment.id, insertIndex.value);

  onInsertEditorClose();
}
</script>

<template>
  <div class="flex flex-col">
    <!-- Parents -->
    <div v-if="(parents?.length ?? 0) > 0" class="pt-1">
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
        <CNoteButton :visible="hovering" @click.stop="onDeleteButtonClick">
          <RiDeleteBinLine size="16px" />
        </CNoteButton>
        <CNoteButton :visible="hovering" @click.stop="onUnlinkButtonClick">
          <RiLinkUnlinkM size="16px" />
        </CNoteButton>
        <CNoteButton :visible="hovering" @click.stop="onEditButtonClick">
          <RiEditLine size="16px" />
        </CNoteButton>
        <div class="w-0.5"></div>
        <CNoteButton :visible="hovering" @click.stop="onInsertSiblingBeforeButtonClick">
          <RiArrowUpWideLine size="16px" />
        </CNoteButton>
        <CNoteButton :visible="hovering" @click.stop="onInsertSiblingAfterButtonClick">
          <RiArrowDownWideLine size="16px" />
        </CNoteButton>
        <CNoteButton :visible="hovering" @click.stop="onInsertChildButtonClick">
          <RiCornerDownRightLine size="16px" />
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
