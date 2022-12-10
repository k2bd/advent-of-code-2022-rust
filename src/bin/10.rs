use std::collections::{HashMap, HashSet};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct Cpu {
    x: i32,
    cycle: u32,
    current_instruction: Option<Instruction>,
    cycles_remaining: u32,
}

impl Cpu {
    fn new() -> Cpu {
        Cpu {
            x: 1,
            cycle: 0,
            current_instruction: None,
            cycles_remaining: 0,
        }
    }

    fn do_cycle(&mut self) {
        self.cycle += 1;
        self.cycles_remaining = (self.cycles_remaining as i32 - 1).max(0) as u32;
        if self.cycles_remaining == 0 {
            if let Some(Instruction::Addx(num)) = &self.current_instruction {
                self.x += num;
            }
            self.current_instruction = None;
        };
    }

    fn set_instruction(&mut self, instruction: Instruction) {
        match &instruction {
            Instruction::Noop => self.cycles_remaining = 1,
            Instruction::Addx(_) => self.cycles_remaining = 2,
        }

        self.current_instruction = Some(instruction);
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            if let Some((_, val)) = line.split_once("addx ") {
                Instruction::Addx(val.parse().expect("Invalid integer"))
            } else {
                Instruction::Noop
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut instructions = parse_input(input);
    instructions.reverse();

    let mut cpu = Cpu::new();
    let mut signals: HashMap<u32, i32> = HashMap::new();

    while instructions.is_empty() {
        cpu.do_cycle();
        if matches!(cpu.current_instruction, None) {
            cpu.set_instruction(instructions.pop().unwrap())
        };

        if (cpu.cycle as i32 - 20) % 40 == 0 {
            signals.insert(cpu.cycle, cpu.x);
        }
    }

    Some(
        signals
            .iter()
            .map(|(&cycle, &value)| cycle as i32 * value)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut instructions = parse_input(input);
    instructions.reverse();

    let mut cpu = Cpu::new();
    let mut drawn_pixels: HashSet<u32> = HashSet::new();

    while !instructions.is_empty() {
        println!("{:?} - {:?}", cpu.cycle, cpu.x);
        if ((cpu.cycle as i32 % 40).abs_diff(cpu.x)) <= 1 {
            drawn_pixels.insert(cpu.cycle);
        }
        if matches!(cpu.current_instruction, None) {
            cpu.set_instruction(instructions.pop().unwrap())
        };
        cpu.do_cycle();
    }

    println!("{:?}", drawn_pixels);

    let monitor = (0..6).map(|row| {
        (row * 40..(row + 1) * 40)
            .map(|px| if drawn_pixels.contains(&px) { '#' } else { '.' })
            .collect::<String>()
    });

    monitor.for_each(|row| println!("{}", row));

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
