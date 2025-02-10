import { Segment, Path as UiPath } from "@/lib/path";
import { defineStore } from "pinia";
import { ref, watchEffect } from "vue";

export const useUiStore = defineStore("ui", () => {
  const anchorId = ref<string>();
  const focusPath = ref<UiPath>(new UiPath());
  const openPaths = ref<Set<string>>(new Set());
  const pinned = ref<{ segment: Segment; parentId?: string }>();

  // Ensure all nodes on the focusPath are unfolded.
  watchEffect(() => {
    // The node pointed to by the path itself doesn't need to be unfolded.
    for (const ancestor of focusPath.value.ancestors().slice(1)) {
      setOpen(ancestor, true);
    }
  });

  function isOpen(path: UiPath): boolean {
    return openPaths.value.has(path.fmt());
  }

  function setOpen(path: UiPath, value: boolean): void {
    // Don't update openPaths unnecessarily.
    // Just in case vue itself doesn't debounce Set operations.
    if (value && !isOpen(path)) {
      openPaths.value.add(path.fmt());
    } else if (!value && isOpen(path)) {
      // Move the focusPath if necessary
      if (path.isPrefixOf(focusPath.value)) focusPath.value = path;

      openPaths.value.delete(path.fmt());
    }
  }

  function toggleOpen(path: UiPath): void {
    setOpen(path, !isOpen(path));
  }

  function isPinned(segment: Segment, parentId?: string): boolean {
    if (!pinned.value) return false;
    return pinned.value.segment.eq(segment) && pinned.value.parentId === parentId;
  }

  function setPinned(segment: Segment, parentId?: string): void {
    pinned.value = { segment, parentId };
  }

  function unsetPinned(): void {
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
