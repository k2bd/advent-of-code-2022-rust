use std::collections::HashMap;

/// A yard of crates in stacks, with each stack given a unique number
#[derive(Debug, PartialEq)]
struct CrateYard {
    crates: HashMap<u32, Vec<char>>,
}

/// An instruction to move crates from one stack to another
#[derive(Debug, PartialEq)]
struct CrateInstruction {
    quantity: u32,
    from: u32,
    to: u32,
}

impl CrateYard {
    /// Apply a single crate instruction for the CrateMover 9000
    fn apply_instruction_crane9000(&mut self, instruction: CrateInstruction) {
        (0..instruction.quantity).for_each(|_| {
            let taken_crate = self
                .crates
                .get_mut(&instruction.from)
                .expect("Cannot move from stack.")
                .pop()
                .expect("Cannot take from an empty stack.");
            self.crates
                .get_mut(&instruction.to)
                .expect("Cannot move to stack.")
                .push(taken_crate);
        })
    }

    /// Apply a single crate instruction for the CrateMover 9001
    fn apply_instruction_crane9001(&mut self, instruction: CrateInstruction) {
        let from_stack = self
            .crates
            .get_mut(&instruction.from)
            .expect("Cannot move from stack");
        let stack_len = from_stack.len();
        let taken_crates = from_stack.split_off(stack_len - instruction.quantity as usize);
        self.crates
            .get_mut(&instruction.to)
            .expect("Cannot move to stack")
            .extend(taken_crates);
    }

    /// Get the readout of the top crates from each stack
    fn top_readout(&self) -> String {
        let mut entries = self.crates.iter().collect::<Vec<(&u32, &Vec<char>)>>();
        entries.sort_by(|(index_a, _), (index_b, _)| index_a.cmp(index_b));

        entries
            .iter()
            .filter_map(|(_, stack)| stack.iter().last())
            .collect::<String>()
    }
}

impl From<Vec<&str>> for CrateYard {
    /// Create from multiline text definition
    fn from(lines: Vec<&str>) -> Self {
        let mut reader = lines.iter().rev();
        let stack_labels = reader
            .next()
            .expect("No indices row")
            .split_whitespace()
            .map(|i| i.parse::<u32>().expect("Index is not a valid integer"))
            .collect::<Vec<u32>>();

        let mut stack_map = HashMap::<u32, Vec<char>>::new();

        stack_labels.iter().for_each(|label| {
            stack_map.insert(*label, Vec::new());
        });

        fn index_location(index: usize) -> usize {
            (index * 4) + 1
        }

        reader.for_each(|line| {
            stack_labels.iter().enumerate().for_each(|(index, label)| {
                if let Some(c) = line.chars().nth(index_location(index)) {
                    if c != ' ' {
                        stack_map.get_mut(label).unwrap().push(c)
                    }
                }
            })
        });

        CrateYard { crates: stack_map }
    }
}

impl From<&str> for CrateInstruction {
    /// Create from a line definition
    fn from(line: &str) -> Self {
        let mut reader = line.split_whitespace();
        reader.next();
        let quantity = reader
            .next()
            .expect("No quantity")
            .parse::<u32>()
            .expect("quantity is not an integer");
        reader.next();
        let from = reader
            .next()
            .expect("No from")
            .parse::<u32>()
            .expect("from is not an integer");
        reader.next();
        let to = reader
            .next()
            .expect("No to")
            .parse::<u32>()
            .expect("to is not an integer");

        CrateInstruction { quantity, from, to }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let lines_vec = input.lines().collect::<Vec<&str>>();
    let mut parts = lines_vec.split(|element| element.is_empty());

    let yard_def = parts.next().expect("No yard definition");
    let move_defs = parts.next().expect("No move definitions");

    let mut yard = CrateYard::from(Into::<Vec<&str>>::into(yard_def));

    move_defs
        .iter()
        .for_each(|move_def| yard.apply_instruction_crane9000(CrateInstruction::from(*move_def)));

    Some(yard.top_readout())
}

pub fn part_two(input: &str) -> Option<String> {
    let lines_vec = input.lines().collect::<Vec<&str>>();
    let mut parts = lines_vec.split(|element| element.is_empty());

    let yard_def = parts.next().expect("No yard definition");
    let move_defs = parts.next().expect("No move definitions");

    let mut yard = CrateYard::from(Into::<Vec<&str>>::into(yard_def));

    move_defs
        .iter()
        .for_each(|move_def| yard.apply_instruction_crane9001(CrateInstruction::from(*move_def)));

    Some(yard.top_readout())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(CrateYard{ crates: HashMap::from([(1, vec!['Z', 'N']), (2, vec!['M', 'C', 'D']), (3, vec!['P'])]) }, CrateInstruction {quantity: 1, from: 2, to: 1},CrateYard{ crates: HashMap::from([(1, vec!['Z', 'N', 'D']), (2, vec!['M', 'C']), (3, vec!['P'])]) } )]
    #[case(CrateYard{ crates: HashMap::from([(1, vec!['Z', 'N', 'D']), (2, vec!['M', 'C']), (3, vec!['P'])]) }, CrateInstruction {quantity: 3, from: 1, to: 3},CrateYard{ crates: HashMap::from([(1, vec![]), (2, vec!['M', 'C']), (3, vec!['P', 'D', 'N', 'Z'])]) } )]
    fn test_apply_crate_instruction_9000(
        #[case] mut state: CrateYard,
        #[case] instruction: CrateInstruction,
        #[case] expected_state: CrateYard,
    ) {
        state.apply_instruction_crane9000(instruction);
        assert_eq!(state, expected_state);
    }

    #[rstest]
    #[case(CrateYard{ crates: HashMap::from([(1, vec!['Z', 'N']), (2, vec!['M', 'C', 'D']), (3, vec!['P'])]) }, CrateInstruction {quantity: 1, from: 2, to: 1},CrateYard{ crates: HashMap::from([(1, vec!['Z', 'N', 'D']), (2, vec!['M', 'C']), (3, vec!['P'])]) } )]
    #[case(CrateYard{ crates: HashMap::from([(1, vec!['Z', 'N', 'D']), (2, vec!['M', 'C']), (3, vec!['P'])]) }, CrateInstruction {quantity: 3, from: 1, to: 3},CrateYard{ crates: HashMap::from([(1, vec![]), (2, vec!['M', 'C']), (3, vec!['P', 'Z', 'N', 'D'])]) } )]
    fn test_apply_crate_instruction_9001(
        #[case] mut state: CrateYard,
        #[case] instruction: CrateInstruction,
        #[case] expected_state: CrateYard,
    ) {
        state.apply_instruction_crane9001(instruction);
        assert_eq!(state, expected_state);
    }

    #[rstest]
    #[case(CrateYard{ crates: HashMap::from([(1, vec!['C']), (2, vec!['M']), (3, vec!['P', 'D', 'N', 'Z'])]) }, "CMZ")]
    fn test_top_readout(#[case] state: CrateYard, #[case] expected_readout: &str) {
        assert_eq!(state.top_readout(), expected_readout);
    }

    #[test]
    fn test_parse_crate_yard() {
        assert_eq!(
            CrateYard::from(vec![
                "    [D]    ",
                "[N] [C]    ",
                "[Z] [M] [P]",
                " 1   2   3 "
            ]),
            CrateYard {
                crates: HashMap::from([
                    (1, vec!['Z', 'N']),
                    (2, vec!['M', 'C', 'D']),
                    (3, vec!['P'])
                ])
            }
        );
    }

    #[test]
    fn test_parse_crate_instruction() {
        assert_eq!(
            CrateInstruction::from("move 3 from 1 to 2"),
            CrateInstruction {
                quantity: 3,
                from: 1,
                to: 2
            }
        );
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
