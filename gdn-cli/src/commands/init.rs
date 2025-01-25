use std::fs;

use anyhow::Context;
use clap::Parser;

use crate::Environment;

/// Initialize a note repository.
#[derive(Debug, Parser)]
pub struct Command {
    repo_name: String,
}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let dir = env.paths.repo_dir(&self.repo_name);

        fs::create_dir_all(&dir)
            .with_context(|| format!("Failed to create directory {}", dir.display()))
            .context("Failed to initialize notes repo")?;

        let repo = gix::init_bare(&dir)
            .with_context(|| format!("Failed to initialize bare git repo at {}", dir.display()))
            .context("Failed to initialize notes repo")?;

        dbg!(repo);
        Ok(())
    }
}
