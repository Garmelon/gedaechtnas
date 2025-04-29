use clap::Parser;

use crate::Environment;

/// Add a new repository.
#[derive(Debug, Parser)]
pub struct Command {
    name: String,
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data = gdn::data::open_and_migrate(env.data_dir.clone())?;
        let id = gdn::data::add_repo(&data, self.name.clone())?;
        println!("Added repo {} ({id}).", self.name);
        Ok(())
    }
}
