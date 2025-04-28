use clap::Parser;

use crate::Environment;

/// Display current status.
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data_dir = gdn::data::open(env.data_dir.clone())?;
        println!("Data dir version: {}", gdn::data::VERSION);

        let state = gdn::data::load_state(&data_dir)?;
        match state.name {
            Some(name) => println!("Name: {name}"),
            None => println!("No name"),
        }

        Ok(())
    }
}
