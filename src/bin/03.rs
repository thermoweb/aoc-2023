use std::ops::Range;

use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Numberl {
    line: usize,
    range: Range<usize>,
    surrounding: String,
    value: i32,
}

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

impl Numberl {
    fn is_mechanical(&self) -> bool {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\d\\.]").unwrap());
        RE.is_match(&*self.surrounding)
    }
}

fn get_parts(grid: &Vec<&str>, reg: Regex) -> Vec<Numberl> {
    let mut results: Vec<_> = vec![];
    for (i, line) in grid.iter().enumerate() {
        let mut ranges: Vec<_> = reg
            .find_iter(line)
            //.inspect(|m| println!("range found : {:?} -> {:?}", m.range(), &line[m.range()]))
            .map(|m| {
                let range = m.range();
                let surr_start = if i > 1 { i - 1 } else { 0 };
                let surr_end = if i < grid.len() - 1 { i + 2 } else { grid.len() };
                let range_start = if range.start == 0 { 0 } else { range.start - 1 };
                let range_end = if range.end < line.len() - 1 { range.end + 1 } else { line.len() };
                let surrounding = &grid[surr_start..surr_end]
                    .iter()
                    //.inspect(|s| { let str = String::from(&s[range_start..range_end]); println!("i: {:?}", str)})
                    .map(|s| String::from(&s[range_start..range_end]))
                    .collect::<Vec<_>>()
                    .join("");
                Numberl { line: i, range: range.clone(), surrounding: String::from(surrounding), value: line[range].parse::<i32>().unwrap() }
            })
            .collect();
        results.append(&mut ranges);
    }
    results
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().collect::<Vec<_>>();
    let reg = Regex::new(r"(\d+)").unwrap();
    let results = get_parts(&grid, reg);
    Some(results.iter().filter(|p| p.is_mechanical()).map(|p| p.value as u32).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input.lines().collect::<Vec<_>>();
    let part_reg = Regex::new(r"[^\d\\.]").unwrap();
    let num_reg = Regex::new(r"\d+").unwrap();
    let mut results: Vec<Part> = vec![];
    for (i, line) in grid.iter().enumerate() {
        let mut result = part_reg
            .find_iter(line)
            .map(|m| {
                let surr_start = if i > 1 { i - 1 } else { 0 };
                let surr_end = if i < grid.len() - 1 { i + 2 } else { grid.len() };
                let mut part = Part { values: vec![], symbol: String::from(&line[m.range()]), surrounding: m.range() };
                for l in &grid[surr_start..surr_end] {
                    let mut values = num_reg.find_iter(l)
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
    Some(results
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
