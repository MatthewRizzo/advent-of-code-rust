//! Implements the solution to day3

use crate::encoding::ItemPriority;
use advent_common::{
    command::AdventSolution,
    errors::{AdventErrors, AdventResult},
    utils,
};

use clap::Args;
use std::{fs, path::PathBuf, str::FromStr};

struct Rucksack {
    compartment_one: String,
    compartment_two: String,
}

impl Rucksack {
    fn find_duplicate(&self) -> AdventResult<String> {
        for item in self.compartment_one.chars() {
            if self.compartment_two.contains(item) {
                return Ok(item.to_string());
            }
        }
        Ok("".to_string())
    }
}

#[derive(Args, Clone, Debug)]
pub struct Day3a {
    /// Path to strategy guide file relative to this file's directory
    #[arg(short, long, default_value = "input.txt")]
    file_name: PathBuf,
}

impl Day3a {
    pub(crate) fn solve_problem_3a(&self) -> AdventResult<String> {
        let project_root_dir: PathBuf = utils::get_project_root()?;
        let input_file_path: PathBuf = project_root_dir
            .as_path()
            .join("day3")
            .join(&self.file_name);

        let input: String = fs::read_to_string(input_file_path)?;

        let mut total_priority: u64 = 0;
        let rucksacks = Self::generate_rucksacks(input)?;

        for rucksack in rucksacks {
            total_priority += Self::get_duplicate_priority(rucksack)? as u64;
        }

        Ok(format!("Total Priority: {}", total_priority))
    }

    fn generate_rucksacks(input: String) -> AdventResult<Vec<Rucksack>> {
        let mut rucksacks: Vec<Rucksack> = Vec::new();
        for line in input.lines() {
            let rucksack = Self::parse_for_rustsack(line)?;
            rucksacks.push(rucksack)
        }

        Ok(rucksacks)
    }

    fn parse_for_rustsack(line: &str) -> AdventResult<Rucksack> {
        let num_items = line.len();
        if num_items % 2 == 1 {
            let err_msg = format!("Line has odd number of items {}, line: {}", num_items, line);
            println!("{}", err_msg);
            return Err(AdventErrors::AdventError(err_msg));
        }

        let middle_idx = num_items >> 1;
        let (compartment_one, compartment_two) = line.split_at(middle_idx);
        let rucksack = Rucksack {
            compartment_one: compartment_one.to_string(),
            compartment_two: compartment_two.to_string(),
        };

        Ok(rucksack)
    }

    fn get_duplicate_priority(rucksack: Rucksack) -> AdventResult<u8> {
        let duplicate_item = rucksack.find_duplicate()?;
        let item_priority = ItemPriority::from_str(&duplicate_item)? as u8;
        Ok(item_priority)
    }
}

impl AdventSolution for Day3a {
    fn find_solution(&self) -> AdventResult<String> {
        self.solve_problem_3a()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_get_duplicate_priority() {
        let rucksack = Rucksack {
            compartment_one: "vJrwpWtwJgWr".to_string(),
            compartment_two: "hcsFMMfFFhFp".to_string(),
        };
        let item_priority = Day3a::get_duplicate_priority(rucksack).expect("Should not error");
        assert!(item_priority == ItemPriority::LOWER_P as u8);

        let rucksack = Rucksack {
            compartment_one: "jqHRNqRjqzjGDLGL".to_string(),
            compartment_two: "rsFMfFZSrLrFZsSL".to_string(),
        };
        let item_priority = Day3a::get_duplicate_priority(rucksack).expect("Should not error");
        assert!(item_priority == ItemPriority::L as u8);

        let rucksack = Rucksack {
            compartment_one: "PmmdzqPrV".to_string(),
            compartment_two: "vPwwTWBwg".to_string(),
        };
        let item_priority = Day3a::get_duplicate_priority(rucksack).expect("Should not error");
        assert!(item_priority == ItemPriority::P as u8);
    }

    #[test]
    fn test_parse_for_rustsack() {
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rustsack = Day3a::parse_for_rustsack(line).expect("Rustsack should be parsable");
        assert!(
            rustsack.compartment_one == "vJrwpWtwJgWr",
            "Compartment one: {}. Expected: vJrwpWtwJgWr",
            rustsack.compartment_one
        );
        assert!(
            rustsack.compartment_two == "hcsFMMfFFhFp",
            "Compartment two: {}. Expected: hcsFMMfFFhFp",
            rustsack.compartment_two
        );
    }
}
