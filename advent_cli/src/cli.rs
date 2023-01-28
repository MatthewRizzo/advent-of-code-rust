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
    Day2a(advent_day2::solution::Day2a),
    Day2b(advent_day2::solution::Day2b),
    Day3a(advent_day3::solution::Day3a),
    Day3b(advent_day3::solution::Day3b),
    Day4a(advent_day4::solution::Day4a),
    Day4b(advent_day4::solution::Day4b),
}

impl AdventSolution for AdventCommands {
    fn find_solution(&self) -> AdventResult<String> {
        use AdventCommands::*;

        match self {
            Day1a(cmd) => cmd.find_solution(),
            Day1b(cmd) => cmd.find_solution(),
            Day2a(cmd) => cmd.find_solution(),
            Day2b(cmd) => cmd.find_solution(),
            Day3a(cmd) => cmd.find_solution(),
            Day3b(cmd) => cmd.find_solution(),
            Day4a(cmd) => cmd.find_solution(),
            Day4b(cmd) => cmd.find_solution(),
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
            println!("Error Running command : <print cmd>.\n Error: {:?}", err);
        }
        Ok(res) => {
            println!("{}", res);
        }
    }
}
