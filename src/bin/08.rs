use std::collections::HashMap;

struct Forest {
    trees: HashMap<(u32, u32), u32>,
}

#[derive(Debug)]
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
            Direction::North => (0..y).rev().map(|y_c| (x, y_c)).collect(),
            Direction::East => (x + 1..self.width()).map(|x_c| (x_c, y)).collect(),
            Direction::South => (y + 1..self.height()).map(|y_c| (x, y_c)).collect(),
            Direction::West => (0..x).rev().map(|x_c| (x_c, y)).collect(),
        };

        range
            .iter()
            .find(|loc| self.trees.get(loc).unwrap() >= tree)
            .copied()
    }

    fn visible(&self, loc: (u32, u32)) -> bool {
        matches!(self._find_blocker(loc, Direction::North), None)
            || matches!(self._find_blocker(loc, Direction::East), None)
            || matches!(self._find_blocker(loc, Direction::South), None)
            || matches!(self._find_blocker(loc, Direction::West), None)
    }

    fn score(&self, (x, y): (u32, u32)) -> u32 {
        let north = self
            ._find_blocker((x, y), Direction::North)
            .unwrap_or((x, 0));
        let east = self
            ._find_blocker((x, y), Direction::East)
            .unwrap_or((self.width() - 1, y));
        let west = self
            ._find_blocker((x, y), Direction::West)
            .unwrap_or((0, y));
        let south = self
            ._find_blocker((x, y), Direction::South)
            .unwrap_or((x, self.height() - 1));

        [north, east, south, west]
            .map(|(x_c, y_c)| x.abs_diff(x_c) + y.abs_diff(y_c))
            .iter()
            .product::<u32>()
    }
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
    let forest = Forest::from(input);

    let mut scores = forest
        .trees
        .keys()
        .map(|&loc| forest.score(loc) as u32)
        .collect::<Vec<_>>();
    scores.sort();

    scores.last().copied()
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
        assert_eq!(part_two(&input), Some(8));
    }
}
