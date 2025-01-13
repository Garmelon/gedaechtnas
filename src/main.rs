use std::path::PathBuf;

use clap::Parser;
use directories::ProjectDirs;

/// Initialize the note repository.
#[derive(Debug, Parser)]
struct CmdInit {}

#[derive(Debug, Parser)]
enum Command {
    Init(CmdInit),
}

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

fn main() {
    let args = Args::parse();
    let dirs = ProjectDirs::from("de", "plugh", env!("CARGO_PKG_NAME")).unwrap();

    let config_file = args
        .config
        .unwrap_or_else(|| dirs.config_dir().join("config.toml"));

    let data_dir = dirs.data_dir();

    println!("Config file: {}", config_file.display());
    println!("Data dir:    {}", data_dir.display());
}
