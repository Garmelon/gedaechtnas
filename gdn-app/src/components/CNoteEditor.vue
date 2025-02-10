<script setup lang="ts">
import { RiCheckLine, RiCloseLine } from "@remixicon/vue";
import { onMounted, ref, useTemplateRef } from "vue";
import CNoteButton from "./CNoteButton.vue";

const props = defineProps<{
  initialText?: string;
}>();

const emit = defineEmits<{
  (e: "close"): void;
  (e: "finish", text: string): void;
}>();

const textarea = useTemplateRef<HTMLTextAreaElement>("textarea");
const text = ref(props.initialText ?? "");

onMounted(() => {
  textarea.value?.focus();
  updateTextareaHeight();
});

function updateTextareaHeight() {
  if (!textarea.value) return;
  textarea.value.style.height = "0px";
  textarea.value.style.height = `${textarea.value.scrollHeight.toFixed()}px`;
}

function onInput() {
  updateTextareaHeight();
}

function onKeyPress(ev: KeyboardEvent) {
  if (ev.key === "Escape") {
    emit("close");
  } else if (ev.key === "Enter" && !ev.shiftKey) {
    emit("finish", text.value);
  } else return;

  ev.preventDefault();
  ev.stopPropagation();
}
</script>

<template>
  <div class="flex items-start gap-0.5">
    <!-- Text -->
    <textarea
      ref="textarea"
      v-model="text"
      class="z-1 flex-1 resize-none bg-neutral-100 px-1 outline-none"
      autofocus
      @input="onInput"
      @keypress="onKeyPress"
      @click.stop
    ></textarea>

    <div class="flex h-6 items-center">
      <CNoteButton @click.stop="emit('finish', text)">
        <RiCheckLine size="16px" />
      </CNoteButton>
    </div>

    <div class="flex h-6 items-center">
      <CNoteButton @click.stop="emit('close')">
        <RiCloseLine size="16px" />
      </CNoteButton>
    </div>
  </div>
</template>
