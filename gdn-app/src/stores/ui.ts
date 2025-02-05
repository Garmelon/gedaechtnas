import { defineStore } from "pinia";
import { ref } from "vue";

export const useUiStore = defineStore("ui", () => {
  const anchor = ref<string>();
  const focusPath = ref<number[]>([1]);

  return {
    anchor,
    focusPath,
  };
});
