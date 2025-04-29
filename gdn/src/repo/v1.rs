use std::collections::HashMap;

use git2::Repository;
use serde::{Deserialize, Serialize};

use crate::ids::NoteId;

pub const VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
pub struct Note {
    pub text: String,
    pub children: Vec<NoteId>,
}

#[derive(Default)]
pub struct Repo {
    pub notes: HashMap<NoteId, Note>,
}

impl Repo {
    pub fn load(repository: &Repository) -> anyhow::Result<Self> {
        todo!()
    }

    pub fn migrate(self) -> super::Repo {
        self
    }
}
