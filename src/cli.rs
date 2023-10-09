use clap::{Parser, Subcommand};

use crate::{interfaces::ActionResult, Playbook};

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

#[derive(Parser)]
struct ArgsWithInput {
    #[command(subcommand)]
    command: CommandsWithInput,
}

#[derive(Subcommand)]
enum CommandsWithInput {
    /// Show information about playbook
    About {
        /// Input data for playbook
        input: Option<String>,
    },
    /// Show source code of playbook
    Source,
    /// Apply playbook
    Apply {
        /// Input data for playbook
        input: String,
    },
}

fn print_about(playbook: &Playbook) {
    println!("# Playbook: {}", playbook.name);
    println!();
    println!("{}", playbook.description);
}

pub fn run_cli(playbook: Playbook, source: &'static str) {
    let args = Args::parse();
    if let Some(cmd) = args.command {
        match cmd {
            Commands::About => print_about(&playbook),
            Commands::Source => println!("{source}"),
        }
    } else if playbook.apply() == ActionResult::Fail {
        std::process::exit(1);
    }
}

impl From<Playbook> for Result<Playbook, String> {
    fn from(value: Playbook) -> Self {
        Ok(value)
    }
}

pub fn run_cli_with_input<GetPlaybook, IntoPb>(
    get_playbook: GetPlaybook,
    input_help: &'static str,
    source: &'static str,
) where
    IntoPb: Into<Result<Playbook, String>>,
    GetPlaybook: FnOnce(&[u8]) -> IntoPb,
{
    let args = ArgsWithInput::parse();
    match args.command {
        CommandsWithInput::About { input: Some(input) } => {
            match get_playbook(input.as_bytes()).into() {
                Ok(pb) => {
                    print_about(&pb);
                    println!();
                    println!("{input_help}");
                }
                Err(e) => {
                    println!("{e}");
                    std::process::exit(1);
                }
            }
        }
        CommandsWithInput::About { input: None } => {
            println!("{input_help}");
        }
        CommandsWithInput::Source => println!("{source}"),
        CommandsWithInput::Apply { input } => match get_playbook(input.as_bytes()).into() {
            Ok(pb) => {
                if pb.apply() == ActionResult::Fail {
                    std::process::exit(1);
                }
            }
            Err(e) => {
                println!("{e}");
                std::process::exit(1);
            }
        },
    };
}
