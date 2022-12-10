use std::collections::HashMap;

struct Forest {
    trees: HashMap<(u32, u32), u32>,
}

enum Direction {
    North,
    East,
    South,
    West,
}

impl Forest {
    fn width(&self) -> u32 {
        self.trees
            .keys()
            .map(|(x, _)| x)
            .max()
            .expect("No max")
            .to_owned()
            + 1
    }

    fn height(&self) -> u32 {
        self.trees
            .keys()
            .map(|(_, y)| y)
            .max()
            .expect("No max")
            .to_owned()
            + 1
    }

    /// Get the location of a blocker in a cardinal direction
    fn _find_blocker(&self, (x, y): (u32, u32), dir: Direction) -> Option<(u32, u32)> {
        let tree = self.trees.get(&(x, y)).unwrap();
        let range: Vec<(u32, u32)> = match dir {
            Direction::North => (0..y).map(|y_c| (x, y_c)).collect(),
            Direction::East => (x + 1..self.width()).map(|x_c| (x_c, y)).collect(),
            Direction::South => (y + 1..self.height()).map(|y_c| (x, y_c)).collect(),
            Direction::West => (0..x).map(|x_c| (x_c, y)).collect(),
        };

        range
            .iter()
            .find(|loc| self.trees.get(&loc).unwrap() >= tree)
            .copied()
    }

    fn visible(&self, (x, y): (u32, u32)) -> bool {
        let tree = self.trees.get(&(x, y)).unwrap();
        (0..x).all(|x_cmp| self.trees.get(&(x_cmp, y)).unwrap() < tree)
            || (x + 1..self.width()).all(|x_cmp| self.trees.get(&(x_cmp, y)).unwrap() < tree)
            || (0..y).all(|y_cmp| self.trees.get(&(x, y_cmp)).unwrap() < tree)
            || (y + 1..self.height()).all(|y_cmp| self.trees.get(&(x, y_cmp)).unwrap() < tree)
    }

    fn score(&self, (x, y): (u32, u32)) -> u32 {}
}

impl From<&str> for Forest {
    fn from(input: &str) -> Self {
        Forest {
            trees: input
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(move |(x, char)| ((x as u32, y as u32), char.to_digit(10).unwrap()))
                })
                .collect(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let forest = Forest::from(input);

    Some(
        forest
            .trees
            .keys()
            .map(|&loc| forest.visible(loc) as u32)
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
