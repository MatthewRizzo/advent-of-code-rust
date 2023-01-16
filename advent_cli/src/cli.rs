//! Interface for users to interact with this application
//! Each command will query the server (via the client), and return the result
use clap::{Parser, Subcommand};

use advent_common::{command::AdventSolution, errors::AdventResult};

/// CLI option for every day/binary.
/// All should implement AdventSolution
#[derive(Subcommand, Clone, Debug)]
pub enum AdventCommands {
    /// Run the program from day1a
    Day1a(advent_day1::solution::Day1a),
    Day1b(advent_day1::solution::Day1b),
}

impl AdventSolution for AdventCommands {
    fn find_solution(&self) -> AdventResult<String> {
        use AdventCommands::*;

        match self {
            Day1a(cmd) => cmd.find_solution(),
            Day1b(cmd) => cmd.find_solution(),
        }
    }
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct AdventCLI {
    #[clap(subcommand)]
    command: AdventCommands,
}

/// Entrance to the client by parsing CLI values and running commands
pub fn run_cli() {
    let args = AdventCLI::parse();
    let cmd_res = args.command.find_solution();

    match cmd_res {
        Err(err) => {
            println!("Error Running command : <print cmd>.\n Error: {}", err);
        }
        Ok(res) => {
            println!("{}", res);
        }
    }
}