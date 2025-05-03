use anyhow::anyhow;
use git2::{FileMode, Repository, Tree, TreeBuilder, TreeEntry, TreeWalkMode, TreeWalkResult};
use serde::{Deserialize, Serialize};

use crate::ids::NoteId;

pub const VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub id: NoteId,
    pub text: String,
    pub children: Vec<NoteId>,
}

#[derive(Default)]
pub struct Repo {
    pub notes: Vec<Note>,
}

fn add_note_to_tree(
    repository: &Repository,
    target: &mut TreeBuilder<'_>,
    note: &Note,
) -> anyhow::Result<()> {
    let filename = format!("{}.json", note.id);
    let oid = repository.blob(&serde_json::to_vec(note)?)?;
    target.insert(filename, oid, FileMode::Blob.into())?;
    Ok(())
}

fn add_tree_to_tree(
    target: &mut TreeBuilder<'_>,
    tree: &TreeBuilder<'_>,
    filename: String,
) -> anyhow::Result<()> {
    if tree.is_empty() {
        return Ok(());
    }
    let oid = tree.write()?;
    target.insert(filename, oid, FileMode::Tree.into())?;
    Ok(())
}

fn load_note(
    repository: &Repository,
    entry: &TreeEntry<'_>,
    notes: &mut Vec<Note>,
) -> anyhow::Result<()> {
    let object = entry.to_object(repository)?;
    let content = object
        .as_blob()
        .ok_or(anyhow!("json file is not a blob!?"))?
        .content();
    let note = serde_json::from_slice(content)?;
    notes.push(note);
    Ok(())
}

impl Repo {
    pub fn load_from_tree(repository: &Repository, tree: &Tree<'_>) -> anyhow::Result<Self> {
        let mut notes = vec![];
        let mut error: Option<anyhow::Error> = None;

        tree.walk(TreeWalkMode::PreOrder, |name, entry| {
            if name.ends_with(".json") {
                if let Err(err) = load_note(repository, entry, &mut notes) {
                    error = Some(err);
                    return TreeWalkResult::Abort;
                }
            }
            TreeWalkResult::Ok
        })?;

        if let Some(err) = error {
            return Err(err);
        }

        Ok(Self { notes })
    }

    pub fn save_to_tree(mut self, repository: &Repository) -> anyhow::Result<Tree<'_>> {
        self.notes.sort_unstable_by_key(|it| it.id);

        let mut root_tree = repository.treebuilder(None)?;
        let mut year = 0;
        let mut year_tree = repository.treebuilder(None)?;
        let mut month = 0;
        let mut month_tree = repository.treebuilder(None)?;
        let mut day = 0;
        let mut day_tree = repository.treebuilder(None)?;

        for note in self.notes {
            let time = note.id.time_utc();

            if day != time.day() || month != time.month() || year != time.year() {
                add_tree_to_tree(&mut month_tree, &day_tree, format!("{day:02}"))?;
                day_tree.clear()?;
                day = time.day();
            }

            if month != time.month() || year != time.year() {
                add_tree_to_tree(&mut year_tree, &month_tree, format!("{month:02}"))?;
                month_tree.clear()?;
                month = time.month();
            }

            if year != time.year() {
                add_tree_to_tree(&mut root_tree, &year_tree, format!("{year:04}"))?;
                year_tree.clear()?;
                year = time.year();
            }

            add_note_to_tree(repository, &mut day_tree, &note)?;
        }

        add_tree_to_tree(&mut month_tree, &day_tree, format!("{day:02}"))?;
        add_tree_to_tree(&mut year_tree, &month_tree, format!("{month:02}"))?;
        add_tree_to_tree(&mut root_tree, &year_tree, format!("{year:04}"))?;

        let tree = repository.find_tree(root_tree.write()?)?;
        Ok(tree)
    }

    pub fn migrate(self) -> Self {
        self
    }
}
