mod add;
mod show;

use clap::Parser;

use crate::Environment;

/// Perform repo operations.
#[derive(Debug, Parser)]
pub enum Command {
    Show(show::Command),
    Add(add::Command),
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        match self {
            Self::Show(command) => command.run(env),
            Self::Add(command) => command.run(env),
        }
    }
}
