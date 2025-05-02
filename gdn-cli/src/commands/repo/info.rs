use clap::Parser;
use gdn::data::REPO_VERSION;

use crate::Environment;

/// Show info about a repository.
#[derive(Debug, Parser)]
pub struct Command {
    repo: String,
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data = gdn::data::open(env.data_dir.clone())?;
        let state = gdn::data::load_state(&data)?;

        let Some(id) = state.resolve_repo_identifier(&self.repo) else {
            println!("No repo found for identifier {}.", self.repo);
            return Ok(());
        };

        let version = gdn::data::load_repo_version(&data, id)?;
        let repo = gdn::data::load_repo(&data, id)?;

        println!("Repo version: {version} (latest: {REPO_VERSION})",);
        println!("Number of notes: {}", repo.notes.len());

        Ok(())
    }
}
