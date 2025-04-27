use clap::Parser;

use crate::Environment;

mod migrate;

#[derive(Debug, Parser)]
pub enum Command {
    Migrate(migrate::Command),
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        match self {
            Self::Migrate(command) => command.run(env),
        }
    }
}
