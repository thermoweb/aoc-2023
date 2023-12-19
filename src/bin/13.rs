use colored::Colorize;
use itertools::Itertools;

advent_of_code::solution!(13);

fn reflects_at(input: &Vec<Vec<char>>, expected_diff: usize) -> Option<usize> {
    (1..input.len()).find(|&offset| {
        let top_half = input.iter().take(offset).rev();
        let bottom_half = input.iter().skip(offset);
        let binding = top_half.zip(bottom_half);
        let found_diff: usize = binding
            .map(|(row1, row2)| row1.iter().zip(row2.iter()).filter(|(a, b)| a != b).count())
            .sum();
        found_diff == expected_diff
    })
}

fn process(input: &str, expected_diff: usize) -> usize {
    input.split("\n\n")
        .map(|pattern| {
            let pattern_vec = pattern.lines().map(|l| l.chars().collect_vec()).collect_vec();
            if let Some(i) = reflects_at(&pattern_vec, expected_diff) {
                return i * 100;
            }
            let cols = (0..pattern_vec[0].len())
                .map(|i| pattern_vec.iter().map(|row| row[i]).collect_vec())
                .collect_vec();
            if let Some(i) = reflects_at(&cols, expected_diff) {
                return i;
            }
            0
        })
        .sum::<usize>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = process(input, 0);
    println!("result = {:?}", result);
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = process(input, 1);
    println!("result = {:?}", result);
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
