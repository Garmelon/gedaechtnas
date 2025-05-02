use clap::Parser;

use crate::Environment;

/// Select a repository.
#[derive(Debug, Parser)]
pub struct Command {
    repo: String,
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data = gdn::data::open_and_migrate(env.data_dir.clone())?;
        let state = gdn::data::load_state(&data)?;

        if self.repo.is_empty() {
            println!("Deselecting repo");
            gdn::data::select_repo(&data, None)?;
            return Ok(());
        }

        let Some(id) = state.resolve_repo_identifier(&self.repo) else {
            println!("No repo found for identifier {}.", self.repo);
            return Ok(());
        };

        println!("Selecting repo {id}");
        gdn::data::select_repo(&data, Some(id))?;

        Ok(())
    }
}
