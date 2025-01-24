mod init;

use clap::Parser;

use crate::Environment;

#[derive(Debug, Parser)]
pub enum Command {
    Init(init::Command),
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        match self {
            Self::Init(command) => command.run(env),
        }
    }
}
