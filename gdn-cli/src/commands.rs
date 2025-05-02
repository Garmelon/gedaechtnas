use clap::Parser;

use crate::Environment;

mod repo;
mod status;
mod tidy;

#[derive(Debug, Parser)]
pub enum Command {
    Status(status::Command),
    Tidy(tidy::Command),
    #[command(subcommand)]
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
