<script setup lang="ts">
import { RiAddLine } from "@remixicon/vue";
import CNoteEditor from "./CNoteEditor.vue";

const emit = defineEmits<{
  (e: "close"): void;
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
      @close="() => emit('close')"
      @finish="(text) => emit('finish', text)"
    />
  </div>
</template>
