<script setup lang="ts">
import { Note as Note } from "@/types";

const props = defineProps<{ note: Note }>();
</script>

<template>
  <div class="flex flex-row gap-1">
    <div>•</div>
    <div class="flex flex-col">
      <div>{{ props.note.text }}</div>
      <CNote v-for="child in props.note.children" :note="child"></CNote>
    </div>
  </div>
</template>
