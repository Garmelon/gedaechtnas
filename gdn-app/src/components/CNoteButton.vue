<script setup lang="ts">
const { visible, inverted = false } = defineProps<{
  visible: boolean;
  inverted?: boolean;
}>();
</script>

<template>
  <div
    class="flex select-none items-center rounded-sm border p-0.5 transition hover:bg-neutral-200 active:scale-95"
    :class="{
      'bg-white text-black': !inverted,
      'border-black bg-black text-white hover:bg-neutral-600': inverted,
      hidden: !visible,
    }"
  >
    <slot></slot>
  </div>
</template>
