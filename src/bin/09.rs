use itertools::Itertools;

advent_of_code::solution!(9);

fn parse_history(str: &str) -> Vec<i64> {
    str.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect_vec()
}

fn process_history(history: Vec<i64>) -> i64 {
    if !history.iter().any(|n| *n != 0) {
        return 0;
    }

    return process_history(history.windows(2).map(|t| t[1] - t[0]).collect_vec())
        + history.last().unwrap();
}

fn process_history_part2(history: Vec<i64>) -> i64 {
    if !history.iter().any(|n| *n != 0) {
        return 0;
    }
    return history.last().unwrap()
        - process_history_part2(history.windows(2).map(|t| t[0] - t[1]).collect_vec());
}

pub fn part_one(input: &str) -> Option<i64> {
    input.lines()
        .map(parse_history)
        .map(process_history)
        .sum::<i64>()
        .into()
}

pub fn part_two(input: &str) -> Option<i64> {
    input.lines()
        .map(parse_history)
        .map(|history| history.iter().rev().copied().collect_vec())
        .map(process_history_part2)
        .sum::<i64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
