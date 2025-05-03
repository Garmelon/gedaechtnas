mod list;

use clap::Parser;

use crate::Environment;

/// Perform note operations.
#[derive(Debug, Parser)]
pub enum Command {
    #[command(visible_alias = "l")]
    List(list::Command),
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        match self {
            Self::List(command) => command.run(env),
        }
    }
}
