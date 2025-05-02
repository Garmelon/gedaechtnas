mod add;
mod info;
mod list;
mod remove;

use clap::Parser;

use crate::Environment;

/// Perform repo operations.
#[derive(Debug, Parser)]
pub enum Command {
    #[command(visible_alias = "l")]
    List(list::Command),

    #[command(visible_alias = "i")]
    Info(info::Command),

    #[command(visible_alias = "a")]
    Add(add::Command),

    #[command(visible_alias = "r")]
    Remove(remove::Command),
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        match self {
            Self::List(command) => command.run(env),
            Self::Info(command) => command.run(env),
            Self::Add(command) => command.run(env),
            Self::Remove(command) => command.run(env),
        }
    }
}
