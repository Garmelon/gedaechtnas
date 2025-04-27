use std::path::PathBuf;

use clap::Parser;

use crate::commands::Command;

mod commands;

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
    data_dir: PathBuf,
}

fn run() -> anyhow::Result<()> {
    let args = Args::parse();

    let env = Environment {
        data_dir: gdn::data::path()?,
    };

    println!("Data dir: {}", env.data_dir.display());
    args.cmd.run(&env)?;
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        println!();
        eprintln!("{err:?}");
        std::process::exit(1);
    }
}
