use clap::Parser;

use crate::Environment;

/// Display current status.
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data_dir = gdn::data::open(env.data_dir.clone())?;
        let state = gdn::data::load_state(&data_dir)?;

        println!("Data dir version: {}", gdn::data::VERSION);

        println!();
        if state.repos.is_empty() {
            println!("No repos");
        } else {
            println!("Repos ({}):", state.repos.len());
            for (id, name) in &state.repos {
                if state.selected_repo == Some(*id) {
                    println!("- {name} ({id}, selected)");
                } else {
                    println!("- {name} ({id})");
                }
            }
        }

        Ok(())
    }
}
