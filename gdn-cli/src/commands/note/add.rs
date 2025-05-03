use clap::Parser;
use gdn::ids::NoteId;
use gdn::repo::Note;

use crate::Environment;

/// Add a note to the selected repository.
#[derive(Debug, Parser)]
pub struct Command {
    text: String,
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data = gdn::data::open_and_migrate(env.data_dir.clone())?;
        let state = gdn::data::load_state(&data)?;
        let Some(selected) = state.selected_repo else {
            println!("No repo selected");
            return Ok(());
        };
        let mut repo = gdn::data::load_repo(&data, selected)?;

        repo.notes.push(Note {
            id: NoteId::new(),
            text: self.text,
            children: vec![],
        });

        let oid = gdn::data::save_repo(&data, selected, repo)?;
        println!("Note added ({oid}).");

        Ok(())
    }
}
