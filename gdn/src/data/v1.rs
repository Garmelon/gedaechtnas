use std::{collections::HashMap, fs, path::PathBuf};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

use crate::{
    ids::RepoId,
    repo::{self, Repo},
};

use super::{LockedDataDir, UnlockedDataDir};

pub const VERSION: u32 = 1;

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    pub repos: HashMap<RepoId, String>,
    pub selected_repo: Option<RepoId>,
}

impl State {
    pub fn normalize(&mut self) {
        if let Some(selected) = self.selected_repo {
            if !self.repos.contains_key(&selected) {
                self.selected_repo = None;
            }
        }
    }

    pub fn resolve_repo_identifier(&self, identifier: &str) -> Option<RepoId> {
        // If the identifier is a valid repo id, always interpret it as such.
        // There must always be an unambiguous way to refer to repos.
        if let Ok(id) = identifier.parse::<RepoId>() {
            if self.repos.contains_key(&id) {
                return Some(id);
            }
        }

        // Otherwise, interpret the identifier as a repo name and find the
        // corresponding id.
        self.repos
            .iter()
            .find(|(_, name)| *name == identifier)
            .map(|(id, _)| *id)
    }
}

pub fn state_file(dir: &UnlockedDataDir) -> PathBuf {
    dir.path().join("state.json")
}

pub fn repos_dir(dir: &UnlockedDataDir) -> PathBuf {
    dir.path().join("repos")
}

pub fn repo_dir(dir: &UnlockedDataDir, id: RepoId) -> PathBuf {
    repos_dir(dir).join(id.to_string())
}

pub fn load_state(dir: &UnlockedDataDir) -> anyhow::Result<State> {
    dir.read_json(&state_file(dir))
}

pub fn save_state(dir: &LockedDataDir, mut state: State) -> anyhow::Result<()> {
    state.normalize();
    dir.write_json(&state_file(dir), &state)
}

pub fn load_repo_version(dir: &UnlockedDataDir, id: RepoId) -> anyhow::Result<u32> {
    repo::load_version(&repo_dir(dir, id))
}

pub fn load_repo(dir: &UnlockedDataDir, id: RepoId) -> anyhow::Result<Repo> {
    repo::load(&repo_dir(dir, id))
}

pub fn add_repo(dir: &LockedDataDir, name: String) -> anyhow::Result<RepoId> {
    let id = RepoId::new();

    let mut state = load_state(dir)?;
    state.repos.insert(id, name);
    save_state(dir, state)?;

    repo::init(&repo_dir(dir, id))?;

    Ok(id)
}

pub fn remove_repo(dir: &LockedDataDir, id: RepoId) -> anyhow::Result<()> {
    let mut state = load_state(dir)?;
    state.repos.remove(&id).is_none();
    save_state(dir, state)?;

    // TODO Check if this works with read-only files
    fs::remove_dir_all(repo_dir(dir, id))?;

    Ok(())
}

pub fn set_repo_name(dir: &LockedDataDir, id: RepoId, name: String) -> anyhow::Result<()> {
    let mut state = load_state(dir)?;
    *state
        .repos
        .get_mut(&id)
        .ok_or_else(|| anyhow!("no repo with id {id}"))? = name;
    save_state(dir, state)?;
    Ok(())
}

pub fn select_repo(dir: &LockedDataDir, id: Option<RepoId>) -> anyhow::Result<()> {
    let mut state = load_state(dir)?;
    state.selected_repo = id;
    save_state(dir, state)?;
    Ok(())
}

pub fn tidy(dir: &LockedDataDir) -> anyhow::Result<()> {
    let state = load_state(dir)?;

    // TODO Detect repo dirs that should not exist, and let the user now.
    //
    // The repo dir contains very important user data. To avoid data loss, we
    // must not delete files or directories that should not exist. Instead, we
    // should let the user know so they can check if they want to keep them.

    // TODO Create repos that don't exist.

    Ok(())
}
