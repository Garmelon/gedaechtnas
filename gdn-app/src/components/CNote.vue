<script setup lang="ts">
import { useNotesStore } from "@/stores/notes";
import { useUiStore } from "@/stores/ui";
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
  path: number[]; // From root to here
  focusPath?: number[]; // From here to focus
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

const open = ref(false);

const focused = computed(() => props.focusPath?.length === 0);

const creating = ref(false);

// We want to set open to true when we're on the focus path, but then it should
// stay true. Hence a computed() combining open and forceOpen would not suffice.
watchEffect(() => {
  open.value; // Ensure we stay open if `open.value = false` is attempted
  if (props.focusPath && props.focusPath.length > 0) open.value = true;
});

// Abort creating a child node whenever we stop being focused.
watchEffect(() => {
  if (!focused.value) creating.value = false;
});

// Ensure the creator component is visible.
watchEffect(() => {
  if (creating.value) open.value = true;
});

function focusPathFor(index: number): number[] | undefined {
  if (!props.focusPath) return undefined;
  if (index !== props.focusPath[0]) return undefined;
  return props.focusPath.slice(1);
}

function focusOnThis() {
  ui.focusPath = props.path.slice();
}

function toggleOpen() {
  if (props.focusPath) {
    // We're on the focus path, so the situation is one of these cases:
    //
    // 1. We're closed and focused, in which case this does nothing.
    // 2. We're open and focused, in which case this does nothing.
    // 3. We're open and on the focus path, in which case closing will hide the
    //    actual focus and we should be focused instead.
    //
    // In any case, we can just...
    focusOnThis();
  }

  open.value = !open.value;
}

function onClick() {
  if (!focused.value) {
    focusOnThis();
    return;
  }

  toggleOpen();
}
</script>

<template>
  <div class="relative flex flex-col">
    <div
      class="flex flex-row gap-1 pl-1"
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
          <RiArrowRightSLine v-if="children.length > 0 && !open" size="16px" />
          <RiArrowDownSLine v-else-if="children.length > 0" size="16px" />
          <RiArrowRightSLine v-else size="16px" class="text-neutral-400" />
        </div>
      </div>

      <!-- Text -->
      <div v-if="note">{{ note.text }}</div>
      <div v-else class="font-light italic">note not found</div>
    </div>

    <!-- Children -->
    <div v-if="open && children.length > 0" class="flex flex-col pl-2">
      <CNote
        v-for="([noteId, key], index) in children"
        :key
        :note-id
        :path="path.concat(index)"
        :focusPath="focusPathFor(index)"
      />
    </div>

    <div v-if="creating" class="pl-2">
      <CNoteCreator
        :parent-id="props.noteId"
        @close="creating = false"
        @finish="creating = false"
      />
    </div>

    <!-- Controls -->
    <div
      v-if="focused"
      class="absolute right-0.5 flex h-6 items-center gap-0.5"
    >
      <CNoteButton @click="creating = true">
        <RiStickyNoteAddLine size="16px" />
      </CNoteButton>
    </div>
  </div>
</template>
