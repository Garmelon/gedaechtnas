import { pathAncestors, pathLiesOn } from "@/util";
import { defineStore } from "pinia";
import { ref, watch, watchEffect } from "vue";

export const useUiStore = defineStore("ui", () => {
  const anchorId = ref<string>();
  const focusPath = ref<string>("");
  const openPaths = ref<Set<string>>(new Set());

  // Ensure all nodes on the focusPath are unfolded.
  watchEffect(() => {
    // The node pointed to by the path itself doesn't need to be unfolded.
    for (const ancestor of pathAncestors(focusPath.value).slice(1)) {
      openPaths.value.add(ancestor);
    }
  });

  // Ensure the focusPath is updated when a node that lies on it is folded.
  watch(openPaths, (now, old) => {
    for (const folded of old.difference(now)) {
      if (pathLiesOn(folded, focusPath.value)) {
        focusPath.value = folded;
      }
    }
  });

  return {
    anchorId,
    focusPath,
    openPaths,
  };
});
