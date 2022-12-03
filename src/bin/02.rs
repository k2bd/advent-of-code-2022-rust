/// Possible moves, and the value of making each move
#[derive(Copy, Clone, Debug, PartialEq)]
enum RPSMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

/// Possible outcomes and their respective scores
#[derive(Copy, Clone, Debug)]
enum RPSResult {
    Win = 6,
    Draw = 3,
    Lose = 0,
}

fn score_rps_game(my_move: RPSMove, their_move: RPSMove) -> u32 {
    let match_points = match (their_move as i32 - my_move as i32).rem_euclid(3) {
        0 => RPSResult::Draw,
        1 => RPSResult::Lose,
        2 => RPSResult::Win,
        _ => {
            panic!("Bad result")
        }
    };

    match_points as u32 + my_move as u32
}

/// Get a move from a character in part 1
fn get_part_1_move(input: char) -> RPSMove {
    match input {
        'A' => RPSMove::Rock,
        'X' => RPSMove::Rock,
        'B' => RPSMove::Paper,
        'Y' => RPSMove::Paper,
        'C' => RPSMove::Scissors,
        'Z' => RPSMove::Scissors,
        _ => panic!("Bad move"),
    }
}

/// Get a desired result from a character in part 2
fn get_part_2_result(input: char) -> RPSResult {
    match input {
        'X' => RPSResult::Lose,
        'Y' => RPSResult::Draw,
        'Z' => RPSResult::Win,
        _ => panic!("Bad result"),
    }
}

/// Get our move from an opponent's move and desired result in part 2
fn get_part_2_move(their_move: RPSMove, desired_result: RPSResult) -> RPSMove {
    let move_index = (their_move as i32 - 1).rem_euclid(3);
    let move_value = match desired_result {
        RPSResult::Lose => (move_index - 1).rem_euclid(3) + 1,
        RPSResult::Draw => (move_index).rem_euclid(3) + 1,
        RPSResult::Win => (move_index + 1).rem_euclid(3) + 1,
    };

    match move_value {
        1 => RPSMove::Rock,
        2 => RPSMove::Paper,
        3 => RPSMove::Scissors,
        _ => panic!("Resulting move is invalid"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let scores = input.lines().map(|line| {
        score_rps_game(
            get_part_1_move(line.chars().last().unwrap()),
            get_part_1_move(line.chars().next().unwrap()),
        )
    });

    Some(scores.sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let scores = input.lines().map(|line| {
        let their_move = get_part_1_move(line.chars().next().unwrap());
        let desired_result = get_part_2_result(line.chars().last().unwrap());

        let our_move = get_part_2_move(their_move, desired_result);
        our_move as u32 + desired_result as u32
    });

    Some(scores.sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(RPSMove::Rock, RPSMove::Rock, 4)]
    #[case(RPSMove::Rock, RPSMove::Paper, 1)]
    #[case(RPSMove::Rock, RPSMove::Scissors, 7)]
    #[case(RPSMove::Paper, RPSMove::Rock, 8)]
    #[case(RPSMove::Paper, RPSMove::Paper, 5)]
    #[case(RPSMove::Paper, RPSMove::Scissors, 2)]
    #[case(RPSMove::Scissors, RPSMove::Rock, 3)]
    #[case(RPSMove::Scissors, RPSMove::Paper, 9)]
    #[case(RPSMove::Scissors, RPSMove::Scissors, 6)]
    fn test_score_rps_games(
        #[case] my_move: RPSMove,
        #[case] their_move: RPSMove,
        #[case] result: u32,
    ) {
        assert_eq!(score_rps_game(my_move, their_move), result);
    }

    #[rstest]
    #[case(RPSMove::Rock, RPSResult::Draw, RPSMove::Rock)]
    #[case(RPSMove::Paper, RPSResult::Lose, RPSMove::Rock)]
    #[case(RPSMove::Scissors, RPSResult::Win, RPSMove::Rock)]
    fn test_get_resulting_move(
        #[case] their_move: RPSMove,
        #[case] desired_result: RPSResult,
        #[case] my_move: RPSMove,
    ) {
        assert_eq!(get_part_2_move(their_move, desired_result), my_move);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
