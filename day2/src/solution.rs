use advent_common::{
    command::AdventSolution,
    errors::{AdventErrors, AdventResult},
    utils,
};
use clap::Args;
use std::{fs, path::PathBuf, str::FromStr};

#[derive(Copy, Clone, Debug, PartialEq)]
enum GameOptions {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl GameOptions {
    /// Resolves the outcome of a match between 2 players.
    fn resolve_match(player: &GameOptions, opponent: &GameOptions) -> AdventResult<MatchResult> {
        if player == opponent {
            return Ok(MatchResult::Tie);
        }

        match player {
            &GameOptions::Rock => {
                if opponent == &GameOptions::Paper {
                    Ok(MatchResult::Opponent)
                } else {
                    Ok(MatchResult::Player)
                }
            }
            &GameOptions::Paper => {
                if opponent == &GameOptions::Scissors {
                    Ok(MatchResult::Opponent)
                } else {
                    Ok(MatchResult::Player)
                }
            }
            &GameOptions::Scissors => {
                if opponent == &GameOptions::Rock {
                    Ok(MatchResult::Opponent)
                } else {
                    Ok(MatchResult::Player)
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum OpponentStrategyMap {
    A,
    B,
    C,
}

impl OpponentStrategyMap {
    /// Converts the strategy mapping to its point value
    fn de_encrypt_input(&self) -> GameOptions {
        match &self {
            OpponentStrategyMap::A => GameOptions::Rock,
            OpponentStrategyMap::B => GameOptions::Paper,
            OpponentStrategyMap::C => GameOptions::Scissors,
        }
    }
}

impl FromStr for OpponentStrategyMap {
    type Err = AdventErrors;
    fn from_str(input: &str) -> AdventResult<OpponentStrategyMap> {
        match input {
            "A" => Ok(OpponentStrategyMap::A),
            "B" => Ok(OpponentStrategyMap::B),
            "C" => Ok(OpponentStrategyMap::C),
            _ => Err(AdventErrors::AdventError(
                "Could not map opponent input -> OpponentStrategyMap".to_string(),
            )),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum PlayerStrategyMap {
    X,
    Y,
    Z,
}

impl PlayerStrategyMap {
    /// Converts the strategy mapping to its point value
    fn de_encrypt_input(&self) -> GameOptions {
        match &self {
            PlayerStrategyMap::X => GameOptions::Rock,
            PlayerStrategyMap::Y => GameOptions::Paper,
            PlayerStrategyMap::Z => GameOptions::Scissors,
        }
    }
}

impl FromStr for PlayerStrategyMap {
    type Err = AdventErrors;
    fn from_str(input: &str) -> AdventResult<PlayerStrategyMap> {
        match input {
            "X" => Ok(PlayerStrategyMap::X),
            "Y" => Ok(PlayerStrategyMap::Y),
            "Z" => Ok(PlayerStrategyMap::Z),
            _ => Err(AdventErrors::AdventError(format!(
                "Could not map player input ({}) -> PlayerStrategyMap",
                input
            ))),
        }
    }
}

#[derive(Debug, PartialEq)]
enum MatchResult {
    Opponent = 0,
    Tie = 3,
    Player = 6,
}

#[derive(Args, Clone, Debug)]
pub struct Day2a {
    /// Path to strategy guide file relative to this file's directory
    #[arg(short, long, default_value = "strategy_guide.txt")]
    file_name: PathBuf,
}

impl AdventSolution for Day2a {
    fn find_solution(&self) -> AdventResult<String> {
        self.solve_problem_2a()
    }
}

impl Day2a {
    pub(crate) fn solve_problem_2a(&self) -> AdventResult<String> {
        let project_root_dir: PathBuf = utils::get_project_root()?;
        let input_file_path: PathBuf = project_root_dir
            .as_path()
            .join("day2")
            .join(&self.file_name);

        let input: String = fs::read_to_string(input_file_path)?;

        let mut total_score: u64 = 0;
        for line in input.lines() {
            let split_line = line.split(" ").collect::<Vec<&str>>();
            self.validate_line_len(line, &split_line)?;
            let opponent_input = OpponentStrategyMap::from_str(&split_line[0])?.de_encrypt_input();
            let player_input = PlayerStrategyMap::from_str(&split_line[1])?.de_encrypt_input();
            let match_winner = GameOptions::resolve_match(&player_input, &opponent_input)?;

            let match_res_score = match_winner as u8;
            let shape_selection_score = player_input as u8;
            let current_match_res = match_res_score + shape_selection_score;
            println!("\n---------------------------");
            println!("Current match info:");
            println!(
                "Player: {:?} vs Opponent: {:?}",
                player_input, opponent_input
            );
            println!(
                "match_winner = {:?} - {} points",
                match_winner, match_res_score
            );
            println!("score from shape = {:?}", shape_selection_score);
            println!("Overall points from match: {:?}", current_match_res);
            println!("---------------------------\n");

            total_score += current_match_res as u64;
        }

        let res_str = format!("The total score is {}", total_score);
        Ok(res_str)
    }

    fn validate_line_len(&self, raw_line: &str, split_line: &Vec<&str>) -> AdventResult<()> {
        if split_line.len() > 2 {
            Err(AdventErrors::AdventError(format!(
                "Line {} had more than 2 entries",
                raw_line
            )))
        } else if split_line.len() < 2 {
            Err(AdventErrors::AdventError(format!(
                "Line {} had less than 2 entries",
                raw_line
            )))
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_resolve_match() {
        // resolve_match
        // test player with paper
        let player = GameOptions::Paper;
        let opponent = GameOptions::Rock;
        let winner = GameOptions::resolve_match(&player, &opponent).expect("cant resolve match 1");
        assert!(winner == MatchResult::Player);

        let player = GameOptions::Paper;
        let opponent = GameOptions::Scissors;
        let winner = GameOptions::resolve_match(&player, &opponent).expect("cant resolve match 1");
        assert!(winner == MatchResult::Opponent);

        let player = GameOptions::Paper;
        let opponent = GameOptions::Paper;
        let winner = GameOptions::resolve_match(&player, &opponent).expect("cant resolve match 1");
        assert!(winner == MatchResult::Tie);

        // test player with rock
        let player = GameOptions::Rock;
        let opponent = GameOptions::Rock;
        let winner = GameOptions::resolve_match(&player, &opponent).expect("cant resolve match 1");
        assert!(winner == MatchResult::Tie);

        let player = GameOptions::Rock;
        let opponent = GameOptions::Scissors;
        let winner = GameOptions::resolve_match(&player, &opponent).expect("cant resolve match 1");
        assert!(winner == MatchResult::Player);

        let player = GameOptions::Rock;
        let opponent = GameOptions::Paper;
        let winner = GameOptions::resolve_match(&player, &opponent).expect("cant resolve match 1");
        assert!(winner == MatchResult::Opponent);

        // test player with scissors
        let player = GameOptions::Scissors;
        let opponent = GameOptions::Rock;
        let winner = GameOptions::resolve_match(&player, &opponent).expect("cant resolve match 1");
        assert!(winner == MatchResult::Opponent);

        let player = GameOptions::Scissors;
        let opponent = GameOptions::Scissors;
        let winner = GameOptions::resolve_match(&player, &opponent).expect("cant resolve match 1");
        assert!(winner == MatchResult::Tie);

        let player = GameOptions::Scissors;
        let opponent = GameOptions::Paper;
        let winner = GameOptions::resolve_match(&player, &opponent).expect("cant resolve match 1");
        assert!(winner == MatchResult::Player);
    }
}
