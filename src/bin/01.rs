fn carried_totals(input: &str) -> Vec<u32> {
    let lines: Vec<&str> = input.lines().collect();

    lines
        .iter()
        .fold(vec![0], |mut acc, value| match value.parse::<u32>() {
            Ok(i) => {
                let val = acc.pop().unwrap();
                acc.push(val + i);
                acc
            }
            _ => {
                acc.push(0);
                acc
            }
        })
}

pub fn part_one(input: &str) -> Option<u32> {
    carried_totals(input).iter().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut totals = carried_totals(input);

    totals.sort();

    let top_carried = totals.iter().rev().take(3);
    Some(top_carried.sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
