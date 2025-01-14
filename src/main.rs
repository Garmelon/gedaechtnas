mod commands;

use std::path::PathBuf;

use clap::Parser;
use directories::ProjectDirs;

use crate::commands::Command;

/// GedächtNAS - external storage for your brain.
#[derive(Debug, Parser)]
#[command(version)]
struct Args {
    /// Path to the config file.
    #[arg(long, short)]
    config: Option<PathBuf>,
    #[command(subcommand)]
    cmd: Command,
}

struct Environment {
    config_file: PathBuf,
    data_dir: PathBuf,
}

impl Environment {
    fn repo_dir(&self) -> PathBuf {
        self.data_dir.join("notes.git")
    }
}

fn main() {
    let args = Args::parse();
    let dirs = ProjectDirs::from("de", "plugh", env!("CARGO_PKG_NAME")).unwrap();

    let config_file = args
        .config
        .unwrap_or_else(|| dirs.config_dir().join("config.toml"));

    let data_dir = dirs.data_dir().to_path_buf();

    println!("Config file: {}", config_file.display());
    println!("Data dir:    {}", data_dir.display());

    let env = Environment {
        config_file,
        data_dir,
    };

    if let Err(err) = args.cmd.run(&env) {
        println!();
        eprintln!("{err:?}");
        std::process::exit(1);
    }
}
