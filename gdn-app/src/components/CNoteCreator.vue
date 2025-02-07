<script setup lang="ts">
import { RiAddLine, RiCheckLine, RiCloseLine } from "@remixicon/vue";
import { onMounted, ref, useTemplateRef } from "vue";
import CNoteButton from "./CNoteButton.vue";

const emit = defineEmits<{
  (e: "close"): void;
  (e: "finish", text: string): void;
}>();

const input = useTemplateRef<HTMLTextAreaElement>("input");
const text = ref("");

onMounted(() => {
  input.value?.focus();
  updateTextareaHeight();
});

function updateTextareaHeight() {
  if (!input.value) return;
  input.value.style.height = "0px";
  input.value.style.height = `${input.value.scrollHeight}px`;
}
</script>

<template>
  <div class="flex flex-row items-start pl-1">
    <!-- Fold/unfold symbol -->
    <div class="flex h-6 items-center">
      <div class="rounded">
        <RiAddLine size="16px" />
      </div>
    </div>

    <!-- Text -->
    <textarea
      ref="input"
      v-model="text"
      class="z-1 flex-1 resize-none bg-neutral-100 px-1 outline-none"
      autofocus
      @input="updateTextareaHeight()"
    ></textarea>

    <div class="ml-0.5 flex h-6 items-center">
      <CNoteButton @click="emit('finish', text)">
        <RiCheckLine size="16px" />
      </CNoteButton>
    </div>

    <div class="ml-0.5 flex h-6 items-center">
      <CNoteButton @click="emit('close')">
        <RiCloseLine size="16px" />
      </CNoteButton>
    </div>
  </div>
</template>
