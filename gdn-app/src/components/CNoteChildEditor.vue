<script setup lang="ts">
import { RiAddLine } from "@remixicon/vue";
import CNoteEditor from "./CNoteEditor.vue";

const { moveAndCopy = false } = defineProps<{
  moveAndCopy?: boolean;
}>();

const emit = defineEmits<{
  (e: "close" | "move" | "copy"): void;
  (e: "finish", text: string): void;
}>();
</script>

<template>
  <div class="flex items-start pl-1">
    <div class="flex h-6 items-center">
      <RiAddLine size="16px" />
    </div>

    <CNoteEditor
      class="flex-1"
      :move-and-copy
      @close="() => emit('close')"
      @finish="(text) => emit('finish', text)"
      @move="() => emit('move')"
      @copy="() => emit('copy')"
    />
  </div>
</template>
