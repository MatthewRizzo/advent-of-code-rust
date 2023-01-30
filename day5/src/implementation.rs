//! Implements structs used for the solution

use std::{fmt, fs, path::PathBuf, str, vec};

use advent_common::{
    errors::{AdventErrors, AdventResult},
    utils,
};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Default)]
pub(crate) struct ItemCrate {
    item: Option<char>,
}

#[derive(Clone, Debug)]
pub(crate) struct CrateStack {
    crates: Vec<ItemCrate>,
}

#[derive(Clone, Debug)]
pub(crate) struct CargoHold {
    stacks: Vec<CrateStack>,
}

/// An unprocessed cargo line from the inout. spans multiple stacks.
type InputCargoLine = Vec<Option<ItemCrate>>;

const SPACE_WIDTH: usize = 1;
const CRATE_WIDTH: usize = 3;
const TOTAL_ITEM_WIDTH: usize = SPACE_WIDTH + CRATE_WIDTH;
const MOVE_STR: &str = "move";
const FROM_STR: &str = "from";
const TO_STR: &str = "to";
const MOVE_STR_LEN: usize = MOVE_STR.len();
const FROM_STR_LEN: usize = FROM_STR.len();
const TO_STR_LEN: usize = TO_STR.len();

impl ItemCrate {
    pub fn new(item: char) -> Self {
        Self { item: Some(item) }
    }

    /// Creates a crate from the input form [<item>]
    ///
    /// # Return
    ///
    /// * `Err` - The ItemCrate cannot be constructed
    /// * `Some` - The ItemCrate parsed
    /// * `None` - There is no item to put in a crate.
    ///     i.e. it was empty for the current stack in the row.
    fn from_input_str(raw_crate: &str) -> AdventResult<Option<Self>> {
        let num_spaces = raw_crate.matches(' ').count();
        let num_open_braces = raw_crate.matches('[').count();
        let num_close_braces = raw_crate.matches(']').count();

        if num_spaces > 2 || num_open_braces < 1 || num_close_braces < 1 {
            return Ok(None);
        }

        let err_msg_fn = |bracket: char| {
            AdventErrors::AdventError(format!("Error finding {} in raw crate", bracket))
        };
        let start_idx = raw_crate.find('[').ok_or_else(|| err_msg_fn('['))? + 1;
        let end_idx = raw_crate.find(']').ok_or_else(|| err_msg_fn(']'))?;

        let result = &raw_crate[start_idx..end_idx]
            .chars()
            .next()
            .ok_or_else(|| {
                AdventErrors::AdventError("Could not get crate character".to_string())
            })?;
        Ok(Some(ItemCrate::new(result.to_owned())))
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.item.is_none()
    }
}

impl fmt::Display for ItemCrate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(item) = self.item {
            write!(f, "{}", item)
        } else {
            write!(f, "None")
        }
    }
}

impl Default for CrateStack {
    fn default() -> Self {
        let stack: Vec<ItemCrate> = vec![ItemCrate::default()];
        Self { crates: stack }
    }
}

impl fmt::Display for CrateStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let map_fn = |item_crate: &ItemCrate| {
            // if !item_crate.is_empty() {
            //     item_crate.to_string()
            // } else {
            //     "".to_string()
            // }
            if !item_crate.is_empty() {
                item_crate.to_string()
            } else {
                "-".to_string()
            }
        };

        let crate_str: String = self.crates.iter().map(map_fn).collect();
        write!(f, "[{}]", crate_str)
    }
}

impl Default for CargoHold {
    fn default() -> Self {
        let stacks: Vec<CrateStack> = vec![];
        Self { stacks }
    }
}

impl fmt::Display for CargoHold {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let map_fn = |stack: &CrateStack| stack.to_string() + " ";
        let stack_str: String = self.stacks.iter().map(map_fn).collect();
        write!(f, "[ {}]", stack_str)
    }
}

pub(crate) struct ProcessedInput {}

impl CrateStack {
    fn push(&mut self, item: ItemCrate) {
        // remove the default empty crate if necessary
        if !self.crates.is_empty() && self.crates[0].is_empty() {
            self.crates.pop();
        }
        self.crates.push(item);
    }

    fn pop(&mut self) -> AdventResult<Option<ItemCrate>> {
        let item = self.crates.pop();
        Ok(item)
    }

    /// # Brief
    /// Retrieves the ItemCrate at the given idx
    #[cfg(test)]
    fn get_crate(&self, crate_idx: usize) -> Option<&ItemCrate> {
        self.crates.get(crate_idx)
    }

    /// Simulates moving the crates between two stack. Returns the resulting stacks
    /// # Return
    /// (Source CrateStack, Destination CrateStack)
    fn move_crates_between_stacks(
        &self,
        other: &CrateStack,
        amount_to_move: u32,
    ) -> AdventResult<(CrateStack, CrateStack)> {
        let mut simulated_source_crate = self.clone();
        let mut simulated_dest_crate = other.clone();

        for _i in 1..=amount_to_move {
            let item_to_move = simulated_source_crate.pop()?;

            if let Some(item) = item_to_move {
                simulated_dest_crate.push(item);
            }
        }

        Ok((simulated_source_crate, simulated_dest_crate))
    }

    pub fn peak_stack(&self) -> AdventResult<Option<ItemCrate>> {
        let num_crates = self.crates.len();
        if num_crates == 0 {
            Ok(None)
        } else {
            let last_crate_idx = num_crates - 1;
            let top_crate = self.crates[last_crate_idx];
            Ok(Some(top_crate))
        }
    }
}

impl CargoHold {
    pub(crate) fn from_file(file_name: &PathBuf) -> AdventResult<Self> {
        ProcessedInput::read_in_file(file_name)
    }

    /// Adds an entire row of input crates to the corresponding stack
    pub(crate) fn push_crate_row(&mut self, crate_row: InputCargoLine) {
        for (stack_idx, opt_item) in crate_row.iter().enumerate() {
            if let Some(item) = opt_item {
                self.stacks[stack_idx].push(item.to_owned());
            }
        }
    }

    /// Modifiys the inital hold to be the right size
    fn initialize(&mut self, total_num_stacks: usize) {
        let stacks: Vec<CrateStack> = vec![CrateStack::default(); total_num_stacks];
        self.stacks = stacks;
    }

    pub fn get_top_of_each_stack(&self) -> AdventResult<Vec<Option<ItemCrate>>> {
        let mut res: Vec<Option<ItemCrate>> = vec![];

        for stack in self.stacks.iter() {
            let top_crate = stack.peak_stack()?;
            res.push(top_crate);
        }

        Ok(res)
    }

    /// # Brief
    /// Retrieves the stack associated to input file (1 based)
    pub(crate) fn get_stack(&self, stack_num: usize) -> AdventResult<&CrateStack> {
        let get_err_fn =
            || AdventErrors::AdventError(format!("Retrieving stack {} failed", stack_num));
        let stack_idx: usize = stack_num - 1;
        let stack = self.stacks.get(stack_idx).ok_or_else(get_err_fn)?;
        Ok(stack)
    }

    ///
    /// # Params:
    /// * `source_stack_num` - The stack number according to the input (i.e. not 0 indexed)
    /// * `dest_stack_num`- - The stack number according to the input (i.e. not 0 indexed)
    fn move_crates_between_stacks(
        &mut self,
        source_stack_num: usize,
        dest_stack_num: usize,
        amount_to_move: u32,
    ) -> AdventResult<()> {
        let source_stack = self.get_stack(source_stack_num as usize)?;
        let dest_stack = self.get_stack(dest_stack_num as usize)?;

        let (new_source_stack, new_dest_stack) =
            source_stack.move_crates_between_stacks(dest_stack, amount_to_move)?;

        let source_stack_idx = source_stack_num - 1;
        let dest_stack_idx = dest_stack_num - 1;

        self.stacks[source_stack_idx] = new_source_stack;
        self.stacks[dest_stack_idx] = new_dest_stack;

        Ok(())
    }
}

impl ProcessedInput {
    /// Processes the line and returns a vector containing the crate
    ///
    /// # Return
    ///
    /// * If an element of the vector is None, that stack doesn't have an item.
    fn parse_stack_line(line: &str) -> AdventResult<Option<InputCargoLine>> {
        let mut row_crates: InputCargoLine = vec![];
        if line.is_empty() {
            return Ok(None);
        }

        let chunked_up_crates = line
            .as_bytes()
            .chunks(TOTAL_ITEM_WIDTH)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()?;

        for raw_crate in chunked_up_crates {
            let item_crate: Option<ItemCrate> = ItemCrate::from_input_str(raw_crate)?;
            row_crates.push(item_crate);
        }
        Ok(Some(row_crates))
    }

    ///
    /// # Brief
    ///  Generates ta vector representing the inputs for a given line
    ///
    /// # Return
    /// * `Some` - The cargo line processed. This spans across ALL stacks
    /// * `None` - If the line signals the stop to the stack inputs
    fn process_file_stack_line(file_line: &str) -> AdventResult<Option<(InputCargoLine, usize)>> {
        let line_items = Self::parse_stack_line(file_line)?;

        match line_items {
            None => Ok(None),
            Some(line_items) => {
                let total_num_stacks: usize = line_items.len();

                let mut cargo_line: InputCargoLine = vec![None; total_num_stacks];

                for (stack_num, opt_item_crate) in line_items.iter().enumerate() {
                    if let Some(item_crate) = opt_item_crate {
                        cargo_line[stack_num] = Some(item_crate.to_owned());
                    }
                }
                Ok(Some((cargo_line, total_num_stacks)))
            }
        }
    }

    /// Processes line of form 'move # from X to Y' where X and Y are stack numbers
    /// Moves the proper amount of crates around as needed by the current line
    fn process_file_move_cmd_line(file_line: &str, cargo_hold: &mut CargoHold) -> AdventResult<()> {
        if file_line.is_empty() {
            return Ok(());
        }

        let err_msg_fn = |word: &str| {
            AdventErrors::AdventError(format!("Error finding {} in line - {}", word, file_line))
        };

        let move_idx = file_line
            .find(MOVE_STR)
            .ok_or_else(|| err_msg_fn(MOVE_STR))?;
        let from_idx = file_line
            .find(FROM_STR)
            .ok_or_else(|| err_msg_fn(FROM_STR))?;
        let to_idx = file_line.find(TO_STR).ok_or_else(|| err_msg_fn(TO_STR))?;

        let move_amount_str = file_line[(move_idx + MOVE_STR_LEN)..from_idx].trim();
        let move_source_str = file_line[(from_idx + FROM_STR_LEN)..to_idx].trim();
        let move_dest_str = file_line[(to_idx + TO_STR_LEN)..].trim();

        // println!(
        //     "move_amount_str = '{}'. move_source_str = '{}'. move_dest_str = '{}'",
        //     move_amount_str, move_source_str, move_dest_str
        // );

        let move_amount = move_amount_str.parse::<u32>()?;

        // These are from input and not index'd
        let move_source_crate = move_source_str.parse::<usize>()?;
        let move_dest_crate = move_dest_str.parse::<usize>()?;

        cargo_hold.move_crates_between_stacks(move_source_crate, move_dest_crate, move_amount)?;

        Ok(())
    }

    ///
    /// # Return
    /// * `CargoHold` - The Initial cargo hold before performing any of the cmds
    /// * `move_cmd_line_start` - The line number of the file where the move
    /// commands start
    fn process_file_input(lines: &str) -> AdventResult<(CargoHold, usize)> {
        // Process the starting stacks
        let mut cargo_hold: CargoHold = CargoHold::default();

        let mut move_cmd_line_start: usize = 0;

        // input of lines goes top-> bottom. We need to create a stack from bottom->top
        let mut reversed_stacks: Vec<InputCargoLine> = vec![];
        let mut total_num_stacks: usize = 0;

        for (idx, line) in lines.lines().enumerate() {
            match Self::process_file_stack_line(line)? {
                None => {
                    move_cmd_line_start = idx;
                    break;
                }
                Some((processed_cargo_line, num_stacks)) => {
                    total_num_stacks = num_stacks;
                    reversed_stacks.push(processed_cargo_line);
                }
            }
        }

        if cargo_hold.stacks.is_empty() {
            cargo_hold.initialize(total_num_stacks);
        }

        // Apply the reversed inputs back onto the actual cargo hold
        for cargo_line in reversed_stacks.iter().rev() {
            cargo_hold.push_crate_row(cargo_line.to_vec());
        }

        Ok((cargo_hold, move_cmd_line_start))
    }

    fn run_move_commands(
        lines: String,
        move_cmd_line_start: usize,
        cargo_hold: &CargoHold,
    ) -> AdventResult<CargoHold> {
        let mut hold_post_cmd = cargo_hold.clone();
        for line in lines.lines().skip(move_cmd_line_start) {
            Self::process_file_move_cmd_line(line, &mut hold_post_cmd)?;
        }

        Ok(hold_post_cmd)
    }

    pub(self) fn read_in_file(file_name: &PathBuf) -> AdventResult<CargoHold> {
        let project_root_dir: PathBuf = utils::get_project_root()?;
        let input_file_path: PathBuf = project_root_dir.as_path().join("day5").join(&file_name);

        let input: String = fs::read_to_string(input_file_path)?;

        let (input_cargo_hold, move_cmd_start_idx) = Self::process_file_input(&input)?;
        Self::run_move_commands(input, move_cmd_start_idx, &input_cargo_hold)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_from_input_str() {
        let err_msg = "Error creating crate from input str".to_string();
        let crate1 = ItemCrate::from_input_str("[A]").expect(&err_msg).unwrap();
        assert!(
            crate1.item == Some('A'),
            "Expected A, got {}",
            crate1.item.unwrap()
        );
    }

    /// Unwraps the input and asserts to
    fn test_parse_line_is_some<T>(val_to_test: &Option<T>, expected: T)
    where
        T: std::cmp::PartialEq + Copy,
    {
        assert!(val_to_test.is_some());
        let val = val_to_test.expect("Could get value out of val_to_test");
        assert!(val == expected);
    }

    #[test]
    fn test_parse_stack_line() {
        let input1 = "    [D]    ";
        let input2 = "[N] [C]    ";
        let input3 = "[Z] [M] [P]";
        let input4 = " 1   2   3 ";

        let process_input1_res = ProcessedInput::parse_stack_line(input1)
            .expect("Processing line should not error")
            .unwrap();
        let in1_stack1 = process_input1_res.get(0).unwrap();
        let in1_stack2 = process_input1_res.get(1).unwrap();
        let in1_stack3 = process_input1_res.get(2).unwrap();
        assert!(in1_stack1.to_owned() == None);
        test_parse_line_is_some(in1_stack2, ItemCrate::new('D'));
        assert!(in1_stack3.to_owned() == None);

        let process_input2_res = ProcessedInput::parse_stack_line(input2)
            .expect("Processing line should not error")
            .unwrap();
        let in2_stack1 = process_input2_res.get(0).unwrap();
        let in2_stack2 = process_input2_res.get(1).unwrap();
        let in2_stack3 = process_input2_res.get(2).unwrap();
        test_parse_line_is_some(in2_stack1, ItemCrate::new('N'));
        test_parse_line_is_some(in2_stack2, ItemCrate::new('C'));
        assert!(in2_stack3.to_owned() == None);

        let process_input3_res = ProcessedInput::parse_stack_line(input3)
            .expect("Processing line should not error")
            .unwrap();
        let in3_stack1 = process_input3_res.get(0).unwrap();
        let in3_stack2 = process_input3_res.get(1).unwrap();
        let in3_stack3 = process_input3_res.get(2).unwrap();
        test_parse_line_is_some(in3_stack1, ItemCrate::new('Z'));
        test_parse_line_is_some(in3_stack2, ItemCrate::new('M'));
        test_parse_line_is_some(in3_stack3, ItemCrate::new('P'));

        let process_input4_res = ProcessedInput::parse_stack_line(input4)
            .expect("Processing line should not error")
            .unwrap();
        let in4_stack1 = process_input4_res.get(0).unwrap();
        let in4_stack2 = process_input4_res.get(1).unwrap();
        let in4_stack3 = process_input4_res.get(2).unwrap();
        assert!(in4_stack1.to_owned() == None);
        assert!(in4_stack2.to_owned() == None);
        assert!(in4_stack3.to_owned() == None);
    }

    #[test]
    fn test_process_file_input() {
        let input1 = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3";

        let (cargo_hold, _move_cmd_start_line) =
            ProcessedInput::process_file_input(&input1.to_string()).expect("Shouldn't fail");

        let create_stack1 = cargo_hold.get_stack(1).expect("Getting stack 0 failed");
        assert!(
            create_stack1.get_crate(0) == Some(&ItemCrate::new('Z')),
            "Got {:?}. Expected: {:?}",
            create_stack1.get_crate(0),
            Some(&ItemCrate::new('Z'))
        );
        assert!(
            create_stack1.get_crate(1) == Some(&ItemCrate::new('N')),
            "Got {:?}. Expected: {:?}",
            create_stack1.get_crate(1),
            Some(&ItemCrate::new('N'))
        );
        assert!(
            create_stack1.get_crate(2) == None,
            "Got {:?}. Expected: None",
            create_stack1.get_crate(2)
        );

        let create_stack2 = cargo_hold.get_stack(2).expect("Getting stack 1 failed");
        assert!(create_stack2.get_crate(0) == Some(&ItemCrate::new('M')));
        assert!(create_stack2.get_crate(1) == Some(&ItemCrate::new('C')));
        assert!(create_stack2.get_crate(2) == Some(&ItemCrate::new('D')));
        assert!(create_stack2.get_crate(3) == None);

        let create_stack3 = cargo_hold.get_stack(3).expect("Getting stack 1 failed");
        assert!(create_stack3.get_crate(0) == Some(&ItemCrate::new('P')));
        assert!(create_stack3.get_crate(1) == None);
    }

    #[test]
    fn test_push_crate_row() {
        let input1 = "    [D]    ";
        let input2 = "[N] [C]    ";
        let input3 = "[Z] [M] [P]";
        let in1 = ProcessedInput::parse_stack_line(input1)
            .expect("fail")
            .unwrap();
        let in2 = ProcessedInput::parse_stack_line(input2)
            .expect("fail")
            .unwrap();
        let in3 = ProcessedInput::parse_stack_line(input3)
            .expect("fail")
            .unwrap();

        let mut cargo_hold: CargoHold = CargoHold::default();
        CargoHold::initialize(&mut cargo_hold, 3);

        cargo_hold.push_crate_row(in3);
        cargo_hold.push_crate_row(in2);
        cargo_hold.push_crate_row(in1);

        let crate_stack0 = &cargo_hold.stacks[0];
        assert!(crate_stack0.get_crate(0) == Some(&ItemCrate::new('Z')));
        assert!(crate_stack0.get_crate(1) == Some(&ItemCrate::new('N')));
        assert!(crate_stack0.get_crate(2) == None);

        let crate_stack1 = &cargo_hold.stacks[1];
        assert!(crate_stack1.get_crate(0) == Some(&ItemCrate::new('M')));
        assert!(crate_stack1.get_crate(1) == Some(&ItemCrate::new('C')));
        assert!(crate_stack1.get_crate(2) == Some(&ItemCrate::new('D')));

        let crate_stack2 = &cargo_hold.stacks[2];
        assert!(crate_stack2.get_crate(0) == Some(&ItemCrate::new('P')));
        assert!(crate_stack2.get_crate(1) == None);
        assert!(crate_stack2.get_crate(2) == None);
    }

    #[test]
    fn test_move_crates_between_stacks() {
        let input1 = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3";

        let (mut cargo_hold, _move_cmd_start_line) =
            ProcessedInput::process_file_input(&input1.to_string()).expect("Shouldn't fail");

        cargo_hold
            .move_crates_between_stacks(1, 2, 3)
            .expect("Shouldn't fail");

        assert!(cargo_hold.stacks[0].get_crate(0) == None);
        assert!(cargo_hold.stacks[1].get_crate(0) == Some(&ItemCrate::new('M')));
        assert!(cargo_hold.stacks[1].get_crate(1) == Some(&ItemCrate::new('C')));
        assert!(cargo_hold.stacks[1].get_crate(2) == Some(&ItemCrate::new('D')));
        assert!(cargo_hold.stacks[1].get_crate(3) == Some(&ItemCrate::new('N')));
        assert!(cargo_hold.stacks[1].get_crate(4) == Some(&ItemCrate::new('Z')));
    }

    #[test]
    fn test_get_top_of_each_stack() {
        let input1 = "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3";

        let (mut cargo_hold, _move_cmd_start_line) =
            ProcessedInput::process_file_input(&input1.to_string()).expect("Shouldn't fail");

        println!("Starting cargo hold: {}", cargo_hold.to_string());

        cargo_hold
            .move_crates_between_stacks(2, 1, 1)
            .expect("Shouldn't fail");
        println!("cargo hold: {}", cargo_hold.to_string());
        let top_line = cargo_hold.get_top_of_each_stack().expect("Shouldn't fail");
        assert!(top_line[0].unwrap().item == Some('D'));
        assert!(top_line[1].unwrap().item == Some('C'));
        assert!(top_line[2].unwrap().item == Some('P'));

        cargo_hold
            .move_crates_between_stacks(1, 3, 3)
            .expect("Shouldn't fail");
        let top_line = cargo_hold.get_top_of_each_stack().expect("Shouldn't fail");
        assert!(top_line[0] == None);
        assert!(top_line[1].unwrap().item == Some('C'));
        assert!(top_line[2].unwrap().item == Some('Z'));

        cargo_hold
            .move_crates_between_stacks(2, 1, 2)
            .expect("Shouldn't fail");
        let top_line = cargo_hold.get_top_of_each_stack().expect("Shouldn't fail");
        assert!(top_line[0].unwrap().item == Some('M'));
        assert!(top_line[1] == None);
        assert!(top_line[2].unwrap().item == Some('Z'));

        cargo_hold
            .move_crates_between_stacks(1, 2, 1)
            .expect("Shouldn't fail");
        let top_line = cargo_hold.get_top_of_each_stack().expect("Shouldn't fail");
        assert!(top_line[0].unwrap().item == Some('C'));
        assert!(top_line[1].unwrap().item == Some('M'));
        assert!(top_line[2].unwrap().item == Some('Z'));
    }
}
