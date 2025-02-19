use clap::Parser;

use crate::Environment;

/// Create or migrate the data dir, if necessary.
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        gdn::data::open_and_migrate(env.data_dir.clone())?;
        Ok(())
    }
}
