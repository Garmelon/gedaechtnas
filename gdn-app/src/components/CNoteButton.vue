<script setup lang="ts">
const {
  visible = true,
  inverted = false,
  disabled = false,
} = defineProps<{
  visible?: boolean;
  inverted?: boolean;
  disabled?: boolean;
}>();
</script>

<template>
  <button
    class="flex h-5 w-5 items-center justify-center rounded-sm border border-black transition"
    :class="{
      'bg-white text-black': !inverted,
      'bg-black text-white': inverted,
      'hover:bg-neutral-200': !disabled && !inverted,
      'hover:bg-neutral-600': !disabled && inverted,
      'active:scale-95': !disabled,
      'opacity-40': disabled,
      invisible: !visible,
    }"
    :disabled
  >
    <slot></slot>
  </button>
</template>
