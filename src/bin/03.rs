const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn compartment_contents(rucksack_contents: &str) -> (&str, &str) {
    rucksack_contents.split_at(rucksack_contents.len() / 2)
}

/// Get a single character shared by all strings
fn shared_content(strings: Vec<&str>) -> Option<char> {
    ALPHABET
        .chars()
        .find(|letter| strings.iter().all(|entry| entry.contains(*letter)))
}

fn shared_compartment_content(left: &str, right: &str) -> Option<char> {
    shared_content(vec![left, right])
}

fn priority(value: char) -> Option<u32> {
    ALPHABET
        .chars()
        .position(|letter| letter == value)
        .map(|v| v as u32 + 1)
}

pub fn part_one(input: &str) -> Option<u32> {
    let priorities = input.lines().map(|line| {
        let (left, right) = compartment_contents(line);
        priority(shared_compartment_content(left, right).unwrap()).unwrap()
    });

    Some(priorities.sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_vec = input.lines().collect::<Vec<&str>>();
    let priorities = input_vec
        .chunks(3)
        .map(|chunk| priority(shared_content(chunk.to_vec()).unwrap()).unwrap());

    Some(priorities.sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("vJrwpWtwJgWrhcsFMMfFFhFp", ("vJrwpWtwJgWr", "hcsFMMfFFhFp"))]
    fn test_compartment_contents(#[case] rucksack_contents: &str, #[case] result: (&str, &str)) {
        assert_eq!(compartment_contents(rucksack_contents), result);
    }

    #[rstest]
    #[case("vJrwpWtwJgWr", "hcsFMMfFFhFp", "p")]
    fn test_shared_content(#[case] left: &str, #[case] right: &str, #[case] shared: char) {
        assert_eq!(shared_compartment_content(left, right), Some(shared));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
