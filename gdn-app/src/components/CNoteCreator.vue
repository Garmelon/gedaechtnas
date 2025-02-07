<script setup lang="ts">
import { RiAddLine, RiCheckLine, RiCloseLine } from "@remixicon/vue";
import { onMounted, ref, useTemplateRef } from "vue";
import CNoteButton from "./CNoteButton.vue";

const emit = defineEmits<{
  (e: "close"): void;
  (e: "finish", text: string): void;
}>();

const input = useTemplateRef<HTMLInputElement>("input");
const text = ref("");

onMounted(() => input.value?.focus());
</script>

<template>
  <div class="flex flex-row items-center gap-1 pl-1 pr-0.5">
    <!-- Fold/unfold symbol -->
    <div class="flex items-center">
      <div class="rounded">
        <RiAddLine size="16px" />
      </div>
    </div>

    <!-- Text -->
    <input
      ref="input"
      v-model="text"
      type="text"
      class="z-1 flex-1 bg-neutral-100"
      autofocus
    />

    <CNoteButton @click="emit('finish', text)">
      <RiCheckLine size="16px" />
    </CNoteButton>

    <CNoteButton @click="emit('close')">
      <RiCloseLine size="16px" />
    </CNoteButton>
  </div>
</template>
