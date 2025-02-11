<script setup lang="ts">
const props = defineProps<{
  inverted?: boolean;
}>();
</script>

<template>
  <div
    class="flex select-none items-center rounded-sm border p-0.5 transition hover:bg-neutral-200 active:scale-95"
    :class="
      props.inverted
        ? 'border-black bg-black text-white hover:bg-neutral-600'
        : 'bg-white text-black'
    "
  >
    <slot></slot>
  </div>
</template>
