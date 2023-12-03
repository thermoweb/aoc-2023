use std::ops::Range;

use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(3);

#[derive(Debug)]
struct Part {
    line: usize,
    range: Range<usize>,
    surrounding: String,
    value: i32,
}

impl Part {
    fn is_mechanical(&self) -> bool {
        static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\d\\.]").unwrap());
        RE.is_match(&*self.surrounding)
    }
}


pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().collect::<Vec<_>>();
    let reg = Regex::new(r"(\d+)").unwrap();
    let results = get_parts(&grid, reg);
    Some(results.iter().filter(|p| p.is_mechanical()).map(|p| p.value as u32).sum())
}

fn get_parts(grid: &Vec<&str>, reg: Regex) -> Vec<Part> {
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
                Part { line: i, range: range.clone(), surrounding: String::from(surrounding), value: line[range].parse::<i32>().unwrap() }
            })
            .collect();
        results.append(&mut ranges);
    }
    results
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
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
