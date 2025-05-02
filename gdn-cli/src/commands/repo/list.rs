use clap::Parser;

use crate::Environment;

/// List all repositories.
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data = gdn::data::open(env.data_dir.clone())?;
        let state = gdn::data::load_state(&data)?;

        let mut repos = state
            .repos
            .into_iter()
            .map(|(id, name)| (name, id))
            .collect::<Vec<_>>();
        repos.sort_unstable();

        if repos.is_empty() {
            println!("No repos");
        } else {
            println!("Repos: {}", repos.len());
            for (name, id) in repos {
                println!("- {name} ({id })")
            }
        }

        Ok(())
    }
}
