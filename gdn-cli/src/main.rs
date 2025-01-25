mod commands;

use std::path::PathBuf;

use clap::Parser;
use gdn::Paths;

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
    paths: Paths,
}

fn main() {
    let args = Args::parse();

    let paths = if cfg!(unix) {
        Paths::on_linux().unwrap()
    } else if cfg!(windows) {
        Paths::on_windows().unwrap()
    } else {
        panic!("running on unsupported platform, only Linux and Windows are supported")
    };

    println!("State file: {}", paths.state_file().display());
    println!("Repos dir:  {}", paths.repos_dir().display());

    let env = Environment { paths };

    if let Err(err) = args.cmd.run(&env) {
        println!();
        eprintln!("{err:?}");
        std::process::exit(1);
    }
}
