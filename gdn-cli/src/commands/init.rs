use std::fs;

use anyhow::Context;
use clap::Parser;

use crate::Environment;

/// Initialize the note repository.
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let directory = env.repo_dir();

        fs::create_dir_all(&directory)
            .with_context(|| format!("Failed to create directory {}", directory.display()))
            .context("Failed to initialize notes repo")?;

        let repo = gix::init_bare(&directory)
            .with_context(|| format!("Failed to initialize git repo at {}", directory.display()))
            .context("Failed to initialize notes repo")?;

        dbg!(repo);
        Ok(())
    }
}
