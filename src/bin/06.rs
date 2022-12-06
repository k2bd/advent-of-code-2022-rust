use std::collections::HashSet;

fn find_marker(input: &str, message_size: usize) -> Option<usize> {
    (0..input.len())
        .find(|&index| {
            input[index..index + message_size]
                .chars()
                .collect::<HashSet<_>>()
                .len()
                == message_size
        })
        .map(|ind| (ind + message_size))
}

pub fn part_one(input: &str) -> Option<usize> {
    find_marker(input, 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    find_marker(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 5)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 6)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11)]
    fn test_part_one(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part_one(input), Some(expected));
    }

    #[rstest]
    #[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
    #[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
    #[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
    #[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
    #[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
    fn test_part_two(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(part_two(input), Some(expected));
    }
}
