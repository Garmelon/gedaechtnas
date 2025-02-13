use std::collections::HashSet;

use serde::Serialize;

use crate::ids::NoteId;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub id: NoteId,
    pub text: String,
    pub children: Vec<NoteId>,
    pub parents: HashSet<NoteId>,
}

////////////
// Events //
////////////

#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventNotesStoreUpdate {
    pub store_id: u64,
}
