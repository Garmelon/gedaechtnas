use clap::Parser;

use crate::Environment;

/// Remove an existing repository.
#[derive(Debug, Parser)]
pub struct Command {
    repo: String,
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data = gdn::data::open_and_migrate(env.data_dir.clone())?;
        let state = gdn::data::load_state(&data)?;
        let Some(id) = state.resolve_repo_identifier(&self.repo) else {
            println!("No repo found for identifier {}.", self.repo);
            return Ok(());
        };
        gdn::data::remove_repo(&data, id)?;
        println!("Removed repo {} ({id}).", self.repo);
        Ok(())
    }
}
