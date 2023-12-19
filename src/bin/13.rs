use std::cmp;
use colored::Colorize;
use itertools::{Itertools, min};
advent_of_code::solution!(13);

fn get_possible_vertical_reflection(input: &str, possiblities: Vec<usize>) -> Vec<usize> {
    // println!("{} - {:?}", input, possiblities);
    let mut possible_reflections: Vec<usize> = vec![];
    for i in possiblities {
        let length = cmp::min(i, input.len() - i);
        let (left, right) = input.split_at(i);
        let left_mirrored = left.chars().rev().collect::<String>();
        if &left_mirrored[0..length] == &right[0..length] {
            possible_reflections.push(i);
        }
        // println!("[{}]{}|{} - {}|{} -> {}", i, &input[0..i], &input[i..input.len()], &left_mirrored[0..length], &right[0..length], &left_mirrored[0..length] == &right[0..length]);
    }
    possible_reflections
}

fn get_vertical_reflection(input: &str) -> Option<usize> {
    let lines = input.lines().collect_vec();
    let mut current_range = (1..lines[0].len()).collect_vec();
    for line in lines {
        let possible_reflections = get_possible_vertical_reflection(line, current_range);
        if possible_reflections.len() == 0 {
            return None;
        }
        current_range = possible_reflections;
    }
    Some(current_range[0])
}

fn get_horizontal_reflection(input: &str) -> Option<usize> {
    let binding = transpose2(input.lines().map(|l| l.chars().collect_vec()).collect_vec());
    let second_pattern = binding.iter().map(|chars| chars.iter().collect::<String>()).collect_vec().join("\n");
    get_vertical_reflection(&second_pattern)
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input.split("\n\n")
        .map(|line| {
            let vertical_reflection = get_vertical_reflection(line);
            return if vertical_reflection.is_some() {
                vertical_reflection.unwrap()
            } else {
                let horizontal_reflection = get_horizontal_reflection(line);
                horizontal_reflection.unwrap() * 100
            }
        })
        .sum::<usize>();
    println!("result = {:?}", result);
    Some(result as u64)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
