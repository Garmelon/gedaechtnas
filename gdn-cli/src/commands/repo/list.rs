use clap::Parser;
use gdn::data::State;

use crate::Environment;

/// List all repositories.
#[derive(Debug, Parser)]
pub struct Command {}

impl Command {
    pub fn run(self, env: &Environment) -> anyhow::Result<()> {
        let data = gdn::data::open(env.data_dir.clone())?;
        let state = gdn::data::load_state(&data)?;
        print_repo_list(&state);
        Ok(())
    }
}

pub fn print_repo_list(state: &State) {
    let mut repos = state
        .repos
        .iter()
        .map(|(id, name)| (name, *id))
        .collect::<Vec<_>>();
    repos.sort_unstable();

    if repos.is_empty() {
        println!("No repos");
    } else {
        println!("Repos: {}", repos.len());
        for (name, id) in repos {
            if state.selected_repo == Some(id) {
                println!("- {name} ({id}, selected)");
            } else {
                println!("- {name} ({id})");
            }
        }
    }
}
