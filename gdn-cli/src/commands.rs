use clap::Parser;

use crate::Environment;

mod repo;
mod status;
mod tidy;

#[derive(Debug, Parser)]
pub enum Command {
    #[command(visible_alias = "s")]
    Status(status::Command),

    #[command(visible_alias = "t")]
    Tidy(tidy::Command),

    #[command(subcommand)]
    #[command(visible_alias = "r")]
    Repo(repo::Command),
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        match self {
            Self::Status(command) => command.run(env),
            Self::Tidy(command) => command.run(env),
            Self::Repo(command) => command.run(env),
        }
    }
}
