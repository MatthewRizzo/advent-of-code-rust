//! Implements the solution to day4

use advent_common::{
    command::AdventSolution,
    errors::{AdventErrors, AdventResult},
    utils,
};

use clap::Args;
use std::{
    cmp::{PartialEq, PartialOrd},
    fs,
    path::PathBuf,
};

#[derive(Default, Debug, Clone, PartialEq, PartialOrd, Eq)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    pub(crate) fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }

    /// Creates the struct from 'X-X'
    fn from_range(range: String) -> AdventResult<Self> {
        let ranges: Vec<&str> = range.split('-').collect();
        let get_val_fn = |idx: usize| {
            ranges
                .get(idx)
                .ok_or_else(|| AdventErrors::AdventError("Can't get range".to_string()))
                .map(|val| val.to_owned())
        };

        let start = get_val_fn(0)?.parse::<u32>()?;
        let end = get_val_fn(1)?.parse::<u32>()?;
        Ok(Self::new(start, end))
    }

    fn is_within_other(&self, other: &Range) -> AdventResult<bool> {
        if (self.start >= other.start && self.end <= other.end)
            || (self.start <= other.start && self.end >= other.end)
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

#[derive(Default, Debug, Clone)]
struct Pair {
    elf_one: Range,
    elf_two: Range,
}

impl Pair {
    /// Instantiates a pair from X-X,Y-Y
    fn from_line(line: String) -> AdventResult<Self> {
        let pair_strings: Vec<&str> = line.split(',').collect();

        let get_pair_strings_fn = |idx: usize| {
            pair_strings
                .get(idx)
                .ok_or_else(|| AdventErrors::AdventError("Can't get pair".to_string()))
                .map(|val| val.to_owned())
        };

        let range_one: String = get_pair_strings_fn(0)?.to_string();
        let range_two: String = get_pair_strings_fn(1)?.to_string();

        let elf_one: Range = Range::from_range(range_one)?;
        let elf_two: Range = Range::from_range(range_two)?;

        Ok(Self { elf_one, elf_two })
    }

    /// Determines if one of the elves range's fully encompass the other's
    fn is_fully_contained(&self) -> AdventResult<bool> {
        self.elf_one.is_within_other(&self.elf_two)
    }
}

#[derive(Args, Clone, Debug)]
pub struct Day4a {
    /// Path to strategy guide file relative to this file's directory
    #[arg(short, long, default_value = "input.txt")]
    file_name: PathBuf,
}

impl Day4a {
    pub(crate) fn solve_problem_4a(&self) -> AdventResult<String> {
        let project_root_dir: PathBuf = utils::get_project_root()?;
        let input_file_path: PathBuf = project_root_dir
            .as_path()
            .join("day4")
            .join(&self.file_name);

        let input: String = fs::read_to_string(input_file_path)?;

        let mut num_fully_contained = 0;

        for line in input.lines() {
            let elf_pair: Pair = Pair::from_line(line.to_string())?;
            if elf_pair.is_fully_contained()? {
                num_fully_contained += 1;
            }
        }

        Ok(format!(
            "Total pairs fully contained: {}",
            num_fully_contained
        ))
    }
}

impl AdventSolution for Day4a {
    fn find_solution(&self) -> AdventResult<String> {
        self.solve_problem_4a()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_is_within_other() {
        let larger_range: Range = Range::new(0, 5);
        let inside_range: Range = Range::new(0, 3);
        let res1 = larger_range
            .is_within_other(&inside_range)
            .expect("Should not error");
        assert!(
            res1 == true,
            "{:?} not within {:?}",
            inside_range,
            larger_range
        );

        let larger_range2: Range = Range::new(0, 2);
        let inside_range2: Range = Range::new(0, 7);
        assert!(larger_range2
            .is_within_other(&inside_range2)
            .expect("Should not error"));

        let overlap_range1: Range = Range::new(0, 5);
        let overlap_range2: Range = Range::new(3, 7);
        assert!(
            overlap_range1
                .is_within_other(&overlap_range2)
                .expect("Should not error")
                == false
        );
    }
}
