use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::{LockedDataDir, UnlockedDataDir};

pub const VERSION: u32 = 1;

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    pub name: Option<String>,
}

pub fn state_file(dir: &UnlockedDataDir) -> PathBuf {
    dir.path().join("state.json")
}

pub fn load_state(dir: &UnlockedDataDir) -> anyhow::Result<State> {
    dir.read_json(&state_file(dir))
}

pub fn save_state(dir: &LockedDataDir, state: &State) -> anyhow::Result<()> {
    dir.write_json(&state_file(dir), state)
}
