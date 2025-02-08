import { pathAncestors, pathLiesOn } from "@/util";
import { defineStore } from "pinia";
import { ref, watchEffect } from "vue";

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

  function isOpen(path: string): boolean {
    return openPaths.value.has(path);
  }

  function setOpen(path: string, open: boolean) {
    // Move the focusPath if necessary
    if (!open && isOpen(path) && pathLiesOn(path, focusPath.value)) {
      focusPath.value = path;
    }

    // Don't update openPaths unnecessarily.
    // Just in case vue itself doesn't debounce Set operations.
    if (open && !isOpen(path)) openPaths.value.add(path);
    else if (!open && isOpen(path)) openPaths.value.delete(path);
  }

  function toggleOpen(path: string) {
    setOpen(path, !isOpen(path));
  }

  return {
    anchorId,
    focusPath,
    isOpen,
    setOpen,
    toggleOpen,
  };
});
