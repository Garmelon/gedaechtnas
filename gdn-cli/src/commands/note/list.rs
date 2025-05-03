use clap::Parser;

use crate::Environment;

/// List all notes in the current repository.
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data = gdn::data::open(env.data_dir.clone())?;
        let state = gdn::data::load_state(&data)?;
        let Some(selected) = state.selected_repo else {
            println!("No repo selected");
            return Ok(());
        };
        let repo = gdn::data::load_repo(&data, selected)?;
        let mut notes = repo.notes.into_iter().collect::<Vec<_>>();
        notes.sort_unstable_by_key(|(id, _)| *id);

        if notes.is_empty() {
            println!("No notes");
            return Ok(());
        }

        for (id, note) in notes {
            if note.children.is_empty() {
                println!("{id}: {}", note.text);
            } else {
                let children = note
                    .children
                    .iter()
                    .map(|it| it.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                println!("{id}: {} [{children}]", note.text);
            }
        }

        Ok(())
    }
}
