<script setup lang="ts">
import { RiCheckLine, RiCloseLine, RiFileCopyLine, RiFileTransferLine } from "@remixicon/vue";
import { onMounted, ref, useTemplateRef } from "vue";
import CNoteButton from "./CNoteButton.vue";

const { initialText = "", moveAndCopy = false } = defineProps<{
  initialText?: string;
  moveAndCopy?: boolean;
}>();

const emit = defineEmits<{
  (e: "close" | "move" | "copy"): void;
  (e: "finish", text: string): void;
}>();

const textarea = useTemplateRef<HTMLTextAreaElement>("textarea");
const text = ref(initialText);
// TODO Store text globally somewhere so it doesn't get lost when editor moves

onMounted(() => {
  textarea.value?.focus();
  updateTextareaHeight();
});

function updateTextareaHeight(): void {
  if (!textarea.value) return;
  textarea.value.style.height = "0px";
  textarea.value.style.height = `${textarea.value.scrollHeight.toFixed()}px`;
}

function onInput(): void {
  updateTextareaHeight();
}

function onKeyPress(ev: KeyboardEvent): void {
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

    <div class="flex h-6 items-center gap-0.5">
      <CNoteButton v-if="moveAndCopy">
        <RiFileTransferLine size="16px" @click.stop="emit('move')" />
      </CNoteButton>
      <CNoteButton v-if="moveAndCopy">
        <RiFileCopyLine size="16px" @click.stop="emit('copy')" />
      </CNoteButton>
      <div v-if="moveAndCopy" class="w-0.5"></div>
      <CNoteButton @click.stop="emit('finish', text)">
        <RiCheckLine size="16px" />
      </CNoteButton>
      <CNoteButton @click.stop="emit('close')">
        <RiCloseLine size="16px" />
      </CNoteButton>
    </div>
  </div>
</template>
