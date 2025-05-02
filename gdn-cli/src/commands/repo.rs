mod add;
mod list;
mod remove;
mod show;

use clap::Parser;

use crate::Environment;

/// Perform repo operations.
#[derive(Debug, Parser)]
pub enum Command {
    List(list::Command),
    Show(show::Command),
    Add(add::Command),
    Remove(remove::Command),
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        match self {
            Self::List(command) => command.run(env),
            Self::Show(command) => command.run(env),
            Self::Add(command) => command.run(env),
            Self::Remove(command) => command.run(env),
        }
    }
}
