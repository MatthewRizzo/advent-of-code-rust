//! Implements the solution to day4

use advent_common::{command::AdventSolution, errors::AdventResult};

use clap::Args;
use std::path::PathBuf;

use crate::implementation::{CargoHold, ItemCrate};

/// Run the program for Day 4a
#[derive(Args, Clone, Debug)]
pub struct Day5a {
    /// Path to strategy guide file relative to this file's directory
    #[arg(short, long, default_value = "input.txt")]
    file_name: PathBuf,
}

impl Day5a {
    pub(crate) fn solve_problem_5a(&self) -> AdventResult<String> {
        let cargo_hold = CargoHold::from_file(&self.file_name)?;
        let top_crates: Vec<Option<ItemCrate>> = cargo_hold.get_top_of_each_stack()?;

        let map_fn = |opt_item: &Option<ItemCrate>| match opt_item {
            Some(item) => item.to_string(),
            None => "-".to_string(),
        };

        let crates_str: String = top_crates.iter().map(map_fn).collect();

        let res_msg = format!("Crate on top of each stack: {}", crates_str);

        Ok(res_msg)
    }
}

impl AdventSolution for Day5a {
    fn find_solution(&self) -> AdventResult<String> {
        self.solve_problem_5a()
    }
}
