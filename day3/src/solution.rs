//! Implements the solution to day3

use crate::encoding::ItemPriority;
use advent_common::{
    command::AdventSolution,
    errors::{AdventErrors, AdventResult},
    utils,
};

use clap::Args;
use std::{fs, path::PathBuf, str::FromStr};

#[derive(Clone, Debug, Default)]
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
    pub(crate) fn merge_compartments(&self) -> AdventResult<String> {
        let self_combined_sack = format!("{}{}", self.compartment_one, self.compartment_two);
        Ok(self_combined_sack)
    }

    pub(crate) fn find_shared_items(&self, sack2: &Rucksack) -> AdventResult<Vec<char>> {
        let self_combined_sack = self.merge_compartments()?;
        let other_combined_sack = sack2.merge_compartments()?;

        let mut common_items = vec![];

        for item in self_combined_sack.chars() {
            if other_combined_sack.contains(item) {
                common_items.push(item);
            }
        }
        if common_items.is_empty() {
            Err(AdventErrors::AdventError(
                "Could not find shared item".to_string(),
            ))
        } else {
            Ok(common_items)
        }
    }
}

struct Day3Common {}

impl Day3Common {
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
}

#[derive(Args, Clone, Debug)]
pub struct Day3a {
    /// Path to strategy guide file relative to this file's directory
    #[arg(short, long, default_value = "input.txt")]
    file_name: PathBuf,
}
#[derive(Debug)]

struct ElfGroup {
    group: [Rucksack; 3],
}

#[derive(Args, Clone, Debug)]
pub struct Day3b {
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
        let rucksacks = Day3Common::generate_rucksacks(input)?;

        for rucksack in rucksacks {
            total_priority += Self::get_duplicate_priority(rucksack)? as u64;
        }

        Ok(format!("Total Priority: {}", total_priority))
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

impl ElfGroup {
    pub fn from_vec(array: &[Rucksack]) -> AdventResult<Self> {
        let process_idx_fn = |idx: usize| {
            array
                .get(idx)
                .ok_or_else(Rucksack::default)
                .map_or_else(|err_res| err_res, |rucksack| rucksack.to_owned())
        };

        let group = [process_idx_fn(0), process_idx_fn(1), process_idx_fn(2)];
        Ok(Self { group })
    }

    /// Finds the item common to all 3 elves in the group
    pub(crate) fn find_common_item(&self) -> AdventResult<char> {
        let common_a_b = self.group[0].find_shared_items(&self.group[1])?;
        let common_b_c = self.group[1].find_shared_items(&self.group[2])?;

        for common_item in common_a_b.iter() {
            if common_b_c.contains(common_item) {
                return Ok(common_item.to_owned());
            }
        }

        Err(AdventErrors::AdventError(
            "Could not find item common between all 3 elves".to_string(),
        ))
    }
}

impl Day3b {
    fn solve_problem_3b(&self) -> AdventResult<String> {
        let project_root_dir: PathBuf = utils::get_project_root()?;
        let input_file_path: PathBuf = project_root_dir
            .as_path()
            .join("day3")
            .join(&self.file_name);

        let input: String = fs::read_to_string(input_file_path)?;

        let mut total_priority: u64 = 0;
        let rucksacks = Day3Common::generate_rucksacks(input)?;

        let groups = Self::create_rucksack_groups(rucksacks)?;
        for group in groups.iter() {
            let common_item = group.find_common_item()?;
            let item_priority: ItemPriority = ItemPriority::from_str(&common_item.to_string())?;
            total_priority += item_priority as u64;
        }

        Ok(format!("Total Priority: {}", total_priority))
    }

    /// Divide all elf rucksacks into groups
    fn create_rucksack_groups(rucksacks: Vec<Rucksack>) -> AdventResult<Vec<ElfGroup>> {
        let mut groups: Vec<ElfGroup> = vec![];
        for sack_array in rucksacks.rchunks(3) {
            let grouping = ElfGroup::from_vec(sack_array)?;
            groups.push(grouping);
        }

        Ok(groups)
    }
}

impl AdventSolution for Day3b {
    fn find_solution(&self) -> AdventResult<String> {
        self.solve_problem_3b()
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
        let rustsack = Day3Common::parse_for_rustsack(line).expect("Rustsack should be parsable");
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

    #[test]
    fn test_merge_compartments() {
        let line = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rustsack = Day3Common::parse_for_rustsack(line).expect("Rustsack should be parsable");

        assert!(
            rustsack
                .merge_compartments()
                .expect("Merging shouldn't fail")
                == line
        );
    }

    #[test]
    fn test_find_shared_items() {
        let line1 = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let rustsack1 = Day3Common::parse_for_rustsack(line1).expect("Rustsack should be parsable");

        let line2 = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let rustsack2 = Day3Common::parse_for_rustsack(line2).expect("Rustsack should be parsable");

        let line3 = "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL";
        let rustsack3 = Day3Common::parse_for_rustsack(line3).expect("Rustsack should be parsable");

        let group = ElfGroup::from_vec(&[rustsack1, rustsack2, rustsack3])
            .expect("Creating group should not fail");
        let common_item = group
            .find_common_item()
            .expect("Finding common item shouldn't fail");
        assert!(common_item == 'r');

        let line4 = "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn";
        let rustsack4 = Day3Common::parse_for_rustsack(line4).expect("Rustsack should be parsable");

        let line5 = "ttgJtRGJQctTZtZT";
        let rustsack5 = Day3Common::parse_for_rustsack(line5).expect("Rustsack should be parsable");

        let line6 = "CrZsJsPPZsGzwwsLwLmpwMDw";
        let rustsack6 = Day3Common::parse_for_rustsack(line6).expect("Rustsack should be parsable");

        let group = ElfGroup::from_vec(&[rustsack4, rustsack5, rustsack6])
            .expect("Creating group should not fail");
        let common_item = group
            .find_common_item()
            .expect("Finding common item shouldn't fail");
        assert!(common_item == 'Z');
    }
}
