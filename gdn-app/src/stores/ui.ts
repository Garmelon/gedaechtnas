import { Segment, Path as UiPath } from "@/lib/path";
import { defineStore } from "pinia";
import { ref, watchEffect } from "vue";

interface HistoryEntry {
  anchorId: string;
  focusPath: UiPath;
  openPaths: Set<string>;
}

type Mode =
  | { type: "focus" }
  | { type: "edit"; path: UiPath }
  | { type: "insert"; path: UiPath; index: number };

export const useUiStore = defineStore("ui", () => {
  const history = ref<HistoryEntry[]>([]);

  // Managed by history
  const anchorId = ref<string>();
  const focusPath = ref<UiPath>(new UiPath());
  const openPaths = ref<Set<string>>(new Set());

  const mode = ref<Mode>({ type: "focus" });
  const pinned = ref<{ segment: Segment; parentId?: string }>();

  // Ensure the currently focused note is visible.
  watchEffect(() => {
    if (mode.value.type === "insert") {
      for (const ancestor of mode.value.path.ancestors()) {
        setOpen(ancestor, true);
      }
    } else {
      // The node pointed to by the path itself doesn't need to be unfolded.
      for (const ancestor of focusPath.value.ancestors().slice(1)) {
        setOpen(ancestor, true);
      }
    }
  });

  ///////////////////////////////////
  // History and anchor management //
  ///////////////////////////////////

  function pushAnchorId(id: string): void {
    if (anchorId.value) {
      history.value.push({
        anchorId: anchorId.value,
        focusPath: focusPath.value,
        openPaths: openPaths.value,
      });
    }

    anchorId.value = id;
    focusPath.value = new UiPath();
    openPaths.value = new Set();

    mode.value = { type: "focus" };
  }

  function popAnchorId(): void {
    // Temporary solution until I implement some UI for anchorId===undefined
    if (history.value.length === 0) return;

    const entry = history.value.pop();
    if (entry) {
      anchorId.value = entry.anchorId;
      focusPath.value = entry.focusPath;
      openPaths.value = entry.openPaths;
    } else {
      anchorId.value = undefined;
      focusPath.value = new UiPath();
      openPaths.value = new Set();
    }

    mode.value = { type: "focus" };
  }

  ///////////////////////////////
  // Mode and focus management //
  ///////////////////////////////

  function isFocused(path: UiPath): boolean {
    return mode.value.type === "focus" && focusPath.value.eq(path);
  }

  function isEditing(path: UiPath): boolean {
    return mode.value.type === "edit" && mode.value.path.eq(path);
  }

  function getInsertIndex(path: UiPath): number | undefined {
    if (mode.value.type !== "insert") return;
    if (!mode.value.path.eq(path)) return;
    if (mode.value.index < 0) return;
    return mode.value.index;
  }

  function focus(): void {
    mode.value = { type: "focus" };
  }

  function focusOn(path: UiPath): void {
    focus();
    focusPath.value = path;
  }

  function edit(path: UiPath): void {
    mode.value = { type: "edit", path };
  }

  function insertAt(path: UiPath, index: number): void {
    mode.value = { type: "insert", path, index };
  }

  //////////////////
  // Node folding //
  //////////////////

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

  /////////////
  // Pinning //
  /////////////

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
    history,
    anchorId,
    focusPath,
    openPaths,
    mode,
    pinned,

    // History and anchor management
    pushAnchorId,
    popAnchorId,

    // Mode and focus management
    isFocused,
    isEditing,
    getInsertIndex,
    focus,
    focusOn,
    edit,
    insertAt,

    // Node folding
    isOpen,
    setOpen,
    toggleOpen,

    // Pinning
    isPinned,
    setPinned,
    unsetPinned,
  };
});
