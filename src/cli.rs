use clap::{Parser, Subcommand};

use crate::Playbook;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show information about playbook
    About,
    /// Show source code of playbook
    Source,
}

pub fn run_with_cli(playbook: Playbook, source: &'static str) {
    let args = Args::parse();
    if let Some(cmd) = args.command {
        match cmd {
            Commands::About => {
                println!("# Playbook: {}", playbook.name);
                println!();
                println!("{}", playbook.description);
            }
            Commands::Source => println!("{source}"),
        }
    } else {
        playbook.apply();
    }
}
