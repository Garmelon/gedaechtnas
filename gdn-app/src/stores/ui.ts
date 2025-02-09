import { pathAncestors, pathLiesOn, pathSlice } from "@/util";
import { defineStore } from "pinia";
import { ref, watchEffect } from "vue";

export const useUiStore = defineStore("ui", () => {
  const anchorId = ref<string>();
  const focusPath = ref<string>("");
  const openPaths = ref<Set<string>>(new Set());
  const pinned = ref<string>(); // The last two segments of the path

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

  function setOpen(path: string, value: boolean) {
    // Move the focusPath if necessary
    if (!value && isOpen(path) && pathLiesOn(path, focusPath.value)) {
      focusPath.value = path;
    }

    // Don't update openPaths unnecessarily.
    // Just in case vue itself doesn't debounce Set operations.
    if (value && !isOpen(path)) openPaths.value.add(path);
    else if (!value && isOpen(path)) openPaths.value.delete(path);
  }

  function toggleOpen(path: string) {
    setOpen(path, !isOpen(path));
  }

  function isPinned(path: string): boolean {
    return pathSlice(path, -2) === pinned.value;
  }

  function setPinned(path: string) {
    pinned.value = pathSlice(path, -2);
  }

  function unsetPinned() {
    pinned.value = undefined;
  }

  return {
    anchorId,
    focusPath,
    isOpen,
    setOpen,
    toggleOpen,
    isPinned,
    setPinned,
    unsetPinned,
  };
});
