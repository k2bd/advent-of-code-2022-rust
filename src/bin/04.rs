/// A zone the elves have to clean
#[derive(Debug, Clone, Copy, PartialEq)]
struct CleaningZone {
    min: u32,
    max: u32,
}

/// Parse a zone out of a string of the format "12-34"
fn parse_zone(input: &str) -> CleaningZone {
    let parsed_nums = input
        .split('-')
        .map(|val| val.parse::<u32>().expect("Section is not an integer"))
        .collect::<Vec<u32>>();

    CleaningZone {
        min: *parsed_nums.first().unwrap(),
        max: *parsed_nums.last().unwrap(),
    }
}

fn parse_puzzle_line(input: &str) -> (CleaningZone, CleaningZone) {
    let mut parts = input.split(',');
    let left = parse_zone(parts.next().unwrap());
    let right = parse_zone(parts.next().unwrap());

    (left, right)
}

/// Return whether one zone fully contains the other
fn fully_contained(left: CleaningZone, right: CleaningZone) -> bool {
    (left.min <= right.min && left.max >= right.max)
        || (right.min <= left.min && right.max >= left.max)
}

fn any_overlap(left: CleaningZone, right: CleaningZone) -> bool {
    (left.min..left.max + 1).any(|num| (right.min..right.max + 1).contains(&num))
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_puzzle_line)
            .filter(|(left, right)| fully_contained(*left, *right))
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(parse_puzzle_line)
            .filter(|(left, right)| any_overlap(*left, *right))
            .count() as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("2-3", CleaningZone {min: 2, max: 3})]
    fn test_parse_zone(#[case] input: &str, #[case] zone: CleaningZone) {
        assert_eq!(parse_zone(input), zone);
    }

    #[rstest]
    #[case(CleaningZone {min: 2, max: 8}, CleaningZone { min: 3, max: 7 }, true)]
    fn test_fully_contained(
        #[case] left_zone: CleaningZone,
        #[case] right_zone: CleaningZone,
        #[case] expected: bool,
    ) {
        assert_eq!(fully_contained(left_zone, right_zone), expected);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
