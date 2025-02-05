<script setup lang="ts">
import { useNotesStore } from "@/stores/notes";
import { RiArrowDownSLine, RiArrowRightSLine } from "@remixicon/vue";
import { computed, ref } from "vue";

const { notes } = useNotesStore();

const props = defineProps<{ id: string | undefined }>();

const note = computed(() => (props.id ? notes.get(props.id) : undefined));
const open = ref(false);
</script>

<template>
  <div class="flex flex-col">
    <div class="flex flex-row gap-1" @click="open = !open">
      <div
        v-if="note && note.children.length > 0 && !open"
        class="flex items-center"
      >
        <RiArrowRightSLine size="16px" />
      </div>
      <div
        v-else-if="note && note.children.length > 0"
        class="flex items-center"
      >
        <RiArrowDownSLine size="16px" />
      </div>
      <div v-else class="flex items-center">
        <RiArrowRightSLine size="16px" class="text-neutral-400" />
      </div>
      <!-- <div v-else class="text-neutral-500">v</div> -->
      <div v-if="note">{{ note.text }}</div>
      <div v-else class="font-light italic">note not found</div>
    </div>
    <div
      v-if="note && open"
      class="flex flex-col border-l border-neutral-300 pl-3"
    >
      <CNote v-for="child in note.children" :id="child"></CNote>
    </div>
  </div>
</template>
