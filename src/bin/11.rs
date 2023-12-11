use std::ops::RangeInclusive;

use itertools::Itertools;

advent_of_code::solution!(11);

fn range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a < b {
        a + 1..=b
    } else {
        b + 1..=a
    }
}

fn compute_distances(galaxies: Vec<(usize, usize)>, expanded_universe: (Vec<usize>, Vec<usize>), expansion: i32) -> usize {
    let mut sum = 0;
    for (i, galaxy) in galaxies.iter().enumerate() {
        for j in i + 1..galaxies.len() {
            let galaxy_y = range(galaxy.0, galaxies[j].0);
            let galaxy_x = range(galaxy.1, galaxies[j].1);
            let line_correction = expanded_universe.0.iter()
                .filter(|e| galaxy_y.contains(e))
                .count();
            let column_correction = expanded_universe.1.iter()
                .filter(|e| galaxy_x.contains(e))
                .count();
            let dist = galaxy_y.count() + line_correction * expansion as usize
                + galaxy_x.count() + column_correction * expansion as usize;
            sum += dist;
            // println!("{:?}, {:?} -> {:?}", galaxies[i], galaxies[j], dist);
        }
    }
    sum
}

fn expand_universe(input: &str, galaxies: Vec<(usize, usize)>) -> (Vec<usize>, Vec<usize>) {
    let lines_to_expand = input
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.contains("#"))
        .map(|(i, _)| i)
        .collect_vec();
    let galaxy_columns = galaxies.iter().map(|(_, x)| x).collect_vec();
    let column_to_expand = input.lines()
        .collect_vec()[0]
        .chars()
        .enumerate()
        .map(|(i, _)| i)
        .filter(|i| {
            !galaxy_columns.contains(&i)
        })
        .collect_vec();
    (lines_to_expand, column_to_expand)
}

fn parse_univers(input: &str) -> Vec<(usize, usize)> {
    let mut output = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                output.push((y, x));
            }
        }
    }
    output
}

fn solve(input: &str, expansion: i32) -> usize {
    let galaxies = parse_univers(input);
    // println!("{:?}", galaxies);
    let expanded_universe = expand_universe(input, galaxies.clone());
    // println!("{:?}", expanded_universe);
    compute_distances(galaxies, expanded_universe, expansion)
}

pub fn part_one(input: &str) -> Option<i32> {
    let sum = solve(input, 1);
    Some(sum as i32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = solve(input, 999999);
    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_10() {
        test_part_two(10, 1030);
    }

    #[test]
    fn test_0() {
        test_part_two(0, 1030);
    }

    #[test]
    fn test_all_parts() {
        test_part_two(1, 374);
        test_part_two(10, 1030);
        test_part_two(100, 8410);
    }

    fn test_part_two(expansion: i32, expected: usize) {
        assert_eq!(solve(&advent_of_code::template::read_file("examples", DAY), expansion), expected);
    }
}
