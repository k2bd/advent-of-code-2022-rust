use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl Add<Position> for Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl Sub<Position> for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl Add<Step> for Position {
    type Output = Position;

    fn add(self, rhs: Step) -> Self::Output {
        self + rhs.rel_position()
    }
}
impl Sub<Step> for Position {
    type Output = Position;

    fn sub(self, rhs: Step) -> Self::Output {
        self - rhs.rel_position()
    }
}

impl Position {
    /// Distance taken to move between two points, allowing diagonals
    fn move_dist(&self, other: Position) -> u32 {
        let dx = (self.0 - other.0).abs();
        let dy = (self.1 - other.1).abs();

        let diagonal_moves = dx.min(dy);
        let straight_moves = dx.max(dy) - diagonal_moves;

        (diagonal_moves + straight_moves) as u32
    }
}

#[derive(Clone, Copy)]
enum Step {
    Up,
    Right,
    Down,
    Left,
}

impl Step {
    fn rel_position(&self) -> Position {
        match &self {
            Step::Up => Position(0, -1),
            Step::Right => Position(1, 0),
            Step::Down => Position(0, 1),
            Step::Left => Position(-1, 0),
        }
    }
}

struct Rope {
    head: Position,
    tail: Option<Box<Rope>>,
}

impl Rope {
    fn new(len: u32) -> Rope {
        Rope {
            head: Position(0, 0),
            tail: if len > 0 {
                Some(Box::new(Rope::new(len - 1)))
            } else {
                None
            },
        }
    }

    fn move_head(&mut self, dir: Position) {
        let new_head_pos = self.head + dir;
        self.head = new_head_pos;

        if let Some(tail) = &mut self.tail {
            let tail_move = if tail.head.move_dist(new_head_pos) > 1 {
                let diff = new_head_pos - tail.head;
                Position(diff.0.signum(), diff.1.signum())
            } else {
                Position(0, 0)
            };

            tail.move_head(tail_move);
        }
    }

    /// Get the ultimate tail position of the rope
    fn tail_pos(&self) -> Position {
        if let Some(tail) = &self.tail {
            tail.tail_pos()
        } else {
            self.head
        }
    }
}

fn get_steps(input: &str) -> Vec<Step> {
    input
        .lines()
        .flat_map(|line| {
            let (dir_str, num) = line.split_once(' ').expect("Invalid line");

            let dir = match dir_str {
                "U" => Step::Up,
                "R" => Step::Right,
                "D" => Step::Down,
                "L" => Step::Left,
                _ => panic!("Invalid dir"),
            };

            (0..num.parse::<u32>().expect("Invalid num")).map(move |_: u32| dir)
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rope = Rope::new(1);
    let mut visited: HashSet<Position> = HashSet::from([Position(0, 0)]);

    get_steps(input).iter().for_each(|step| {
        rope.move_head(step.rel_position());
        visited.insert(rope.tail_pos());
    });

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope::new(9);
    let mut visited: HashSet<Position> = HashSet::from([Position(0, 0)]);

    get_steps(input).iter().for_each(|step| {
        rope.move_head(step.rel_position());
        visited.insert(rope.tail_pos());
    });

    Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }

    #[test]
    fn test_part_two_extra() {
        let input = advent_of_code::read_file("examples_extra", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
