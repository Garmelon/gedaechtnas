import { Segment, Path as UiPath } from "@/lib/path";
import { defineStore } from "pinia";
import { computed, ref, watchEffect } from "vue";

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
  const _anchorId = ref<string>();
  const _focusPath = ref<UiPath>(new UiPath());
  const openPaths = ref<Set<string>>(new Set());

  const _mode = ref<Mode>({ type: "focus" });
  const pinned = ref<{ segment: Segment; parentId?: string }>();

  // Ensure the currently focused note is visible.
  watchEffect(() => {
    if (_mode.value.type === "insert") {
      for (const ancestor of _mode.value.path.ancestors()) {
        setOpen(ancestor, true);
      }
    } else {
      // The node pointed to by the path itself doesn't need to be unfolded.
      for (const ancestor of _focusPath.value.ancestors().slice(1)) {
        setOpen(ancestor, true);
      }
    }
  });

  ///////////////////////////////////
  // History and anchor management //
  ///////////////////////////////////

  const anchorId = computed(() => _anchorId.value); // Getter

  function pushAnchorId(id: string): void {
    if (_anchorId.value) {
      history.value.push({
        anchorId: _anchorId.value,
        focusPath: _focusPath.value,
        openPaths: openPaths.value,
      });
    }

    _anchorId.value = id;
    _focusPath.value = new UiPath();
    openPaths.value = new Set();

    _mode.value = { type: "focus" };
  }

  function popAnchorId(): void {
    // Temporary solution until I implement some UI for anchorId===undefined
    if (history.value.length === 0) return;

    const entry = history.value.pop();
    if (entry) {
      _anchorId.value = entry.anchorId;
      _focusPath.value = entry.focusPath;
      openPaths.value = entry.openPaths;
    } else {
      _anchorId.value = undefined;
      _focusPath.value = new UiPath();
      openPaths.value = new Set();
    }

    _mode.value = { type: "focus" };
  }

  ///////////////////////////////
  // Mode and focus management //
  ///////////////////////////////

  const mode = computed(() => _mode.value.type);
  const focusPath = computed(() => _focusPath.value);

  function isFocused(path: UiPath): boolean {
    return _mode.value.type === "focus" && _focusPath.value.eq(path);
  }

  function isEditing(path: UiPath): boolean {
    return _mode.value.type === "edit" && _mode.value.path.eq(path);
  }

  function getInsertIndex(path: UiPath): number | undefined {
    if (_mode.value.type !== "insert") return;
    if (!_mode.value.path.eq(path)) return;
    if (_mode.value.index < 0) return;
    return _mode.value.index;
  }

  function focus(): void {
    _mode.value = { type: "focus" };
  }

  function focusOn(path: UiPath): void {
    focus();
    _focusPath.value = path;
  }

  function edit(path: UiPath): void {
    _mode.value = { type: "edit", path };
  }

  function insertAt(path: UiPath, index: number): void {
    _mode.value = { type: "insert", path, index };
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
      if (path.isPrefixOf(_focusPath.value)) _focusPath.value = path;

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
    anchorId,
    pushAnchorId,
    popAnchorId,
    isFocused,
    isEditing,
    getInsertIndex,
    mode,
    focusPath,
    focus,
    focusOn,
    edit,
    insertAt,
    isOpen,
    setOpen,
    toggleOpen,
    isPinned,
    setPinned,
    unsetPinned,
  };
});
