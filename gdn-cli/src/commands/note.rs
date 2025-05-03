mod add;
mod list;

use clap::Parser;

use crate::Environment;

/// Perform note operations.
#[derive(Debug, Parser)]
pub enum Command {
    #[command(visible_alias = "l")]
    List(list::Command),
    #[command(visible_alias = "a")]
    Add(add::Command),
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        match self {
            Self::List(command) => command.run(env),
            Self::Add(command) => command.run(env),
        }
    }
}
