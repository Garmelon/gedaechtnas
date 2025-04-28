use clap::Parser;

use crate::Environment;

mod migrate;
mod status;

#[derive(Debug, Parser)]
pub enum Command {
    Migrate(migrate::Command),
    Status(status::Command),
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        match self {
            Self::Migrate(command) => command.run(env),
            Self::Status(command) => command.run(env),
        }
    }
}
