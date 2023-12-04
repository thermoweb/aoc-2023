use std::ops::Range;

use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Part {
    values: Vec<u32>,
    symbol: String,
    surrounding: Range<usize>,
}

#[derive(Debug)]
struct Number {
    value: u32,
    positions: Range<usize>,
}

impl Number {
    fn intersect(&self, range: Range<usize>) -> bool {
        self.positions.start <= range.end && range.start <= self.positions.end
    }
}

fn get_parts(grid: &Vec<&str>) -> Vec<Part> {
    static PART_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\d\\.]").unwrap());
    static NUM_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
    let mut results: Vec<Part> = vec![];
    for (i, line) in grid.iter().enumerate() {
        let mut result = PART_REG
            .find_iter(line)
            .map(|m| {
                let surr_start = if i > 1 { i - 1 } else { 0 };
                let surr_end = if i < grid.len() - 1 { i + 2 } else { grid.len() };
                let mut part = Part { values: vec![], symbol: String::from(&line[m.range()]), surrounding: m.range() };
                for l in &grid[surr_start..surr_end] {
                    let mut values = NUM_REG.find_iter(l)
                        .map(|m2| Number { value: (&l[m2.range()]).parse().unwrap(), positions: m2.range() })
                        .filter(|n| n.intersect(part.surrounding.clone()))
                        .map(|n| n.value)
                        .collect::<Vec<_>>();
                    part.values.append(&mut values);
                }
                part
            })
            .collect::<Vec<_>>();
        results.append(&mut result);
    }
    results
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().collect::<Vec<_>>();
    let results = get_parts(&grid);
    Some(results
        .iter()
        .map(|p| p.values.iter().sum::<u32>())
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().collect::<Vec<_>>();
    Some(get_parts(&grid)
        .iter()
        .filter(|p| p.values.len() > 1 && p.symbol == "*")
        .map(|p| p.values[0] * p.values[1])
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
