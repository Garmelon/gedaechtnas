use std::collections::{HashMap, HashSet};

use crate::{ids::NoteId, types::Note};

#[derive(Debug)]
pub struct NoteInfo {
    pub text: String,
    pub children: Vec<NoteId>,
}

/// A note store for testing.
#[derive(Default)]
pub struct Store {
    last_id: u64,
    curr_id: u64,
    notes: HashMap<NoteId, NoteInfo>,
    parents: HashMap<NoteId, HashMap<NoteId, usize>>,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn needs_update(&self) -> Option<u64> {
        if self.last_id != self.curr_id {
            Some(self.curr_id)
        } else {
            None
        }
    }

    pub fn update(&mut self) {
        self.last_id = self.curr_id;
    }

    pub fn get(&self, id: NoteId) -> Option<Note> {
        let info = self.notes.get(&id)?;

        let parents = self
            .parents
            .get(&id)
            .map(|ps| ps.keys().copied().collect::<HashSet<_>>())
            .unwrap_or_default();

        Some(Note {
            id,
            text: info.text.clone(),
            children: info.children.clone(),
            parents,
        })
    }

    fn tick(&mut self) {
        self.curr_id += 1;
    }

    fn make_consistent_and_tick(&mut self) {
        // Remove child notes that don't exist
        let children = self.notes.keys().copied().collect::<HashSet<_>>();
        for info in &mut self.notes.values_mut() {
            info.children.retain(|child| children.contains(child));
        }

        // Update parents to match new child notes
        self.parents.clear();
        for (id, info) in &self.notes {
            for child in &info.children {
                *self
                    .parents
                    .entry(*child)
                    .or_default()
                    .entry(*id)
                    .or_default() += 1;
            }
        }

        self.tick();
    }

    pub fn create(&mut self, text: String) -> NoteId {
        let id = NoteId::new();
        let info = NoteInfo {
            text,
            children: vec![],
        };

        self.notes.insert(id, info);
        self.make_consistent_and_tick();

        id
    }

    pub fn delete(&mut self, id: NoteId) -> Option<NoteInfo> {
        let info = self.notes.remove(&id)?;
        self.make_consistent_and_tick();
        Some(info)
    }

    pub fn set_text(&mut self, id: NoteId, text: String) -> Option<()> {
        let note = self.notes.get_mut(&id)?;
        if note.text == text {
            return None;
        }
        note.text = text;
        self.tick();
        Some(())
    }

    pub fn set_children(&mut self, id: NoteId, children: Vec<NoteId>) -> Option<()> {
        let note = self.notes.get_mut(&id)?;
        if note.children == children {
            return None;
        }
        note.children = children;
        self.make_consistent_and_tick();
        Some(())
    }

    /// Find the index of a child based on its id and iteration.
    ///
    /// The index returned is in the range `[0, note.children.len())`.
    ///
    /// Iteration 0 refers to the first occurrence of the id, iteration 1 to the
    /// second, and so on. Returns [`None`] if no such iteration was found.
    fn resolve_child_iteration(
        children: &[NoteId],
        child_id: NoteId,
        child_iteration: usize,
    ) -> Option<usize> {
        children
            .iter()
            .enumerate()
            .filter(|(_, it)| **it == child_id)
            .map(|(i, _)| i)
            .nth(child_iteration)
    }

    /// Find the index of a child based on its position.
    ///
    /// The index returned is in the range `[0, note.children.len()]`.
    ///
    /// # Example
    ///
    /// A small example for a note with children `[a, b, c]`:
    ///
    /// ```text
    /// Child     [ a,  b,  c]  _
    /// Position    0   1   2   3
    /// Position   -4  -3  -2  -1
    /// ```
    fn resolve_child_position(children: &[NoteId], child_position: isize) -> usize {
        if child_position > 0 {
            let child_position = child_position as usize;
            child_position.min(children.len())
        } else {
            let child_position = (-child_position - 1) as usize;
            children.len().saturating_sub(child_position)
        }
    }

    /// Add a child at the specified position.
    ///
    /// Returns `Some(())` if the operation was successful.
    pub fn add_child_at_position(
        &mut self,
        id: NoteId,
        child_id: NoteId,
        child_position: isize,
    ) -> Option<()> {
        let note = self.notes.get_mut(&id)?;
        let index = Self::resolve_child_position(&note.children, child_position);
        note.children.insert(index, child_id);

        self.make_consistent_and_tick();
        Some(())
    }

    /// Remove the specified iteration of a child.
    ///
    /// Returns `Some(())` if the operation was successful.
    pub fn remove_child_by_id(
        &mut self,
        id: NoteId,
        child_id: NoteId,
        child_iteration: usize,
    ) -> Option<()> {
        let note = self.notes.get_mut(&id)?;
        let index = Self::resolve_child_iteration(&note.children, child_id, child_iteration)?;
        note.children.remove(index);

        self.make_consistent_and_tick();
        Some(())
    }

    /// A combination of [`Self::add_child_at_position`] and
    /// [`Self::remove_child_by_id`].
    ///
    /// Returns `Some(())` if the operation was successful.
    pub fn move_child_by_id_to_position(
        &mut self,
        child_id: NoteId,
        from_id: NoteId,
        from_iteration: usize,
        to_id: NoteId,
        to_position: isize,
    ) -> Option<()> {
        let from = self.get(from_id)?;
        let to = self.get(to_id)?;

        let from_idx = Self::resolve_child_iteration(&from.children, child_id, from_iteration)?;
        let mut to_idx = Self::resolve_child_position(&to.children, to_position);

        if from_id == to_id && from_idx < to_idx {
            to_idx -= 1;
        }

        let removed_id = self
            .notes
            .get_mut(&from_id)
            .unwrap()
            .children
            .remove(from_idx);
        assert!(removed_id == child_id);

        self.notes
            .get_mut(&to_id)
            .unwrap()
            .children
            .insert(to_idx, child_id);

        self.make_consistent_and_tick();
        Some(())
    }

    pub fn clear(&mut self) {
        self.notes.clear();
        self.make_consistent_and_tick();
    }
}
