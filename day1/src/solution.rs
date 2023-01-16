use advent_common::{command::AdventSolution, errors::AdventResult, utils};
use clap::Args;
use std::{cmp::Reverse, fs, path::PathBuf};

/// Finds the elf with the highest calories and returns the amount
///
/// # Return
/// The calories of the elf that has the most
///
/// # Param
/// The read in string from the input file
fn find_highest_cal(input: String) -> AdventResult<u64> {
    let mut highest_cal: u64 = 0;
    let mut cur_elf_cal: u64 = 0;
    for line in input.lines() {
        if line.is_empty() {
            if cur_elf_cal > highest_cal {
                highest_cal = cur_elf_cal;
            }
            cur_elf_cal = 0;
        } else {
            cur_elf_cal += line.parse::<u64>()?;
        }
    }

    Ok(highest_cal)
}

///
/// # Return
/// A sorted vector
fn generate_cal_list(input: String) -> AdventResult<Vec<u64>> {
    let mut res: Vec<u64> = Vec::new();

    let mut cur_elf_cal = 0;

    for line in input.lines() {
        if line.is_empty() {
            res.push(cur_elf_cal);
            cur_elf_cal = 0;
        } else {
            cur_elf_cal += line.parse::<u64>()?;
        }
    }

    res.sort_by_key(|w| Reverse(*w));
    Ok(res)
}

// Obtains the solution to the problem
pub(crate) fn solve_problem_1a() -> AdventResult<String> {
    let project_root_dir: PathBuf = utils::get_project_root()?;
    let input_file_path: PathBuf = project_root_dir.as_path().join("day1").join("input.txt");

    let input: String = fs::read_to_string(input_file_path)?;
    let res: u64 = find_highest_cal(input)?;
    let res_msg = format!("Elf with highest amount of calories has: {}", res);
    Ok(res_msg)
}

pub(crate) fn solve_problem_1b() -> AdventResult<String> {
    let project_root_dir: PathBuf = utils::get_project_root()?;
    let input_file_path: PathBuf = project_root_dir.as_path().join("day1").join("input.txt");

    let input: String = fs::read_to_string(input_file_path)?;

    let mut cal_list: Vec<u64> = generate_cal_list(input)?;

    while cal_list.len() < 3 {
        cal_list.push(0);
    }

    let combined_calories = cal_list[0] + cal_list[1] + cal_list[2];

    let res_msg = format!("Total calories for top 3 elves: {}", combined_calories);
    Ok(res_msg)
}

#[derive(Args, Clone, Debug)]
pub struct Day1a {}

impl AdventSolution for Day1a {
    fn find_solution(&self) -> AdventResult<String> {
        solve_problem_1a()
    }
}

#[derive(Args, Clone, Debug)]
pub struct Day1b {}

impl AdventSolution for Day1b {
    fn find_solution(&self) -> AdventResult<String> {
        solve_problem_1b()
    }
}
