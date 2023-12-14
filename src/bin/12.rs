use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;

use crate::Spring::{Damaged, Operational, Unknown};

advent_of_code::solution!(12);

#[derive(Eq, Hash, PartialEq, Clone)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

impl Spring {
    fn to_char(&self) -> char {
        match self {
            Damaged => '#',
            Operational => '.',
            Unknown => '?',
        }
    }

    fn to_string(&self) -> String {
        self.to_char().to_string()
    }
}

type Cache = HashMap<Record, usize>;

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Record {
    springs: String,
    damaged_groups: Vec<usize>,
}

impl Record {
    fn from(input: &str) -> Record {
        let (raw_springs, raw_damaged) = input.split_once(' ').unwrap();
        let springs = raw_springs.to_string();
        let damaged_groups = raw_damaged.split(',').map(|c| c.parse().unwrap()).collect_vec();
        Record { springs, damaged_groups }
    }
    fn from_copies(input: &str, copies: usize) -> Record {
        let (raw_springs, raw_damaged) = input.split_once(' ').unwrap();
        let mut springs = raw_springs.to_string();
        let mut groups = raw_damaged.to_string();
        if copies > 1 {
            springs = vec![springs; copies].join(&Unknown.to_string());
            groups = vec![groups; copies].join(",");
        }
        let damaged_groups = groups.split(',').map(|c| c.parse().unwrap()).collect_vec();
        Record { springs, damaged_groups }
    }

    fn create(springs: String, damaged_groups: Vec<usize>) -> Option<Record> {
        let record = Record { springs, damaged_groups };
        record.reduce()
    }

    fn reduce(mut self) -> Option<Record> {
        // mostly inspired by https://github.com/BorisBoutillier/advent-of-code-2023/blob/bfd91d3883eaca0a425e3361d9cffeff7d48fae8/day-12/src/lib.rs#L33
        let mut start_groups = vec![];
        let mut current_group = 0;
        let mut last_operational = None;
        let mut finished = true;
        for (i, spring) in self.springs.chars().enumerate() {
            match spring {
                '?' => {
                    finished = false;
                    break;
                }
                '.' => {
                    if current_group > 0 {
                        start_groups.push(current_group);
                        current_group = 0;
                    }
                    last_operational = Some(i);
                }
                '#' => {
                    current_group += 1;
                }
                _ => unreachable!(),
            }
        }
        if finished {
            if current_group > 0 {
                start_groups.push(current_group);
            }
            if self.damaged_groups != start_groups {
                return None;
            } else {
                return Some(Record { springs: "".to_string(), damaged_groups: vec![] });
            }
        }
        if start_groups.len() > self.damaged_groups.len()
            || !start_groups.iter().zip(self.damaged_groups.iter()).all(|(a, b)| a == b)
            || (current_group > 0
            && (self.damaged_groups.len() == start_groups.len()
            || self.damaged_groups[start_groups.len()] < current_group)) {
            None
        } else {
            self.damaged_groups = self.damaged_groups[start_groups.len()..].to_vec();
            if let Some(id) = last_operational {
                self.springs = self.springs[id..].to_string();
            }
            Some(self)
        }
    }

    fn get_permutations(&self, cache: &mut Cache) -> usize {
        if let Some(permutations) = cache.get(self) {
            *permutations
        } else {
            let permuts = self.compute_permutations(cache);
            cache.insert(self.clone(), permuts);
            permuts
        }
    }

    fn compute_permutations(&self, cache: &mut Cache) -> usize {
        if self.damaged_groups.is_empty() {
            return if self.springs.contains('#') { 0 } else { 1 };
        }
        let first_unknown = self.springs.chars().position(|s| s == Unknown.to_char()).unwrap();
        [Damaged, Operational]
            .iter()
            .map(|condition| {
                let new_springs = [&self.springs[..first_unknown], &self.springs[first_unknown + 1..]].join(&condition.to_string());
                let record = Record::create(new_springs, self.damaged_groups.clone());
                record.map(|r| r.get_permutations(cache)).unwrap_or(0)
            })
            .sum()
    }
}

fn count_possible_records(record: Record) -> usize {
    let mut cache = Cache::new();
    record.get_permutations(&mut cache)
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines()
        .collect_vec()
        .par_iter()
        .map(|l| Record::from(l))
        .map(|r| count_possible_records(r))
        .sum::<usize>();
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input.lines()
        .collect_vec()
        .par_iter()
        .map(|l| Record::from_copies(l, 5))
        .map(|r| count_possible_records(r))
        .sum::<usize>();
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
