use std::{fs, path::PathBuf};

use advent_common::{errors::AdventResult, utils};

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

fn main() -> AdventResult<()> {
    let project_root_dir: PathBuf = utils::get_project_root()?;
    let input_file_path: PathBuf = project_root_dir
        .as_path()
        .join("day1")
        .join("day1a")
        .join("input.txt");

    let input: String = fs::read_to_string(input_file_path)?;
    let res: u64 = find_highest_cal(input)?;
    println!("Elf with highest amount of calories has: {}", res);
    Ok(())
}
