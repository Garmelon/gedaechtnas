use std::collections::HashSet;

use gdn::{ids::NoteId, store::RichNote};
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: NoteId,
    pub text: String,
    pub children: Vec<NoteId>,
    pub parents: HashSet<NoteId>,
}

impl From<RichNote> for Note {
    fn from(value: RichNote) -> Self {
        Self {
            id: value.id,
            text: value.text,
            children: value.children,
            parents: value.parents,
        }
    }
}

////////////
// Events //
////////////

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventNotesStoreUpdate {
    pub store_id: u64,
}
