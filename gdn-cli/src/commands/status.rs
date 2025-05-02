use clap::Parser;

use crate::Environment;

/// Display current status.
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        println!("Data dir: {}", env.data_dir.display());

        let version = gdn::data::read_version(env.data_dir.clone())?;
        if version == gdn::data::VERSION {
            println!("Data dir version: {version} (current)");
        } else {
            println!(
                "Data dir version: {version} (outdated, current: {})",
                gdn::data::VERSION
            );
        }

        let data = gdn::data::open(env.data_dir.clone())?;
        let state = gdn::data::load_state(&data)?;

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
