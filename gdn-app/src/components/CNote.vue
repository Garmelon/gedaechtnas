<script setup lang="ts">
import { useNotesStore } from "@/stores/notes";
import { RiArrowDownSLine, RiArrowRightSLine } from "@remixicon/vue";
import { computed, ref } from "vue";

const { notes } = useNotesStore();

const props = defineProps<{ id: string | undefined }>();

const note = computed(() => (props.id ? notes.get(props.id) : undefined));
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
</script>

<template>
  <div class="flex flex-col">
    <div class="flex flex-row gap-1" @click="open = !open">
      <!-- Fold/unfold symbol -->
      <div v-if="children.length > 0 && !open" class="flex items-center">
        <RiArrowRightSLine size="16px" />
      </div>
      <div v-else-if="children.length > 0" class="flex items-center">
        <RiArrowDownSLine size="16px" />
      </div>
      <div v-else class="flex items-center">
        <RiArrowRightSLine size="16px" class="text-neutral-400" />
      </div>

      <!-- Text -->
      <div v-if="note">{{ note.text }}</div>
      <div v-else class="font-light italic">note not found</div>
    </div>

    <!-- Children -->
    <div
      v-if="note && open"
      class="flex flex-col border-l border-neutral-300 pl-3"
    >
      <CNote v-for="[id, key] in children" :id :key></CNote>
    </div>
  </div>
</template>
