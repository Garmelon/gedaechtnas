use clap::Parser;

use crate::Environment;

/// Perform data dir maintenance.
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data = gdn::data::open_and_migrate(env.data_dir.clone())?;
        gdn::data::tidy(&data)?;
        Ok(())
    }
}
