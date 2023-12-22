use std::collections::HashMap;

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::Rule::{Default, Inferior, Superior};

advent_of_code::solution!(19);

type Part = HashMap<String, u32>;

fn part_from(input: &str) -> Part {
    let mut map = Part::new();
    static PART_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"x=(\d+),m=(\d+),a=(\d+),s=(\d+)").unwrap());
    let captures = PART_REG.captures(input).unwrap();
    let x = input[captures.get(1).unwrap().range()].parse::<u32>().unwrap();
    let m = input[captures.get(2).unwrap().range()].parse::<u32>().unwrap();
    let a = input[captures.get(3).unwrap().range()].parse::<u32>().unwrap();
    let s = input[captures.get(4).unwrap().range()].parse::<u32>().unwrap();
    map.insert(String::from("x"), x);
    map.insert(String::from("m"), m);
    map.insert(String::from("a"), a);
    map.insert(String::from("s"), s);
    map
}

#[derive(Debug)]
enum Rule {
    Default(String),
    Inferior(String, u32, String),
    Superior(String, u32, String),
}

impl Rule {
    fn from(input: &str) -> Rule {
        static RULE_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"([amsx])([><])(\d+):(\w+)").unwrap());
        let captures = RULE_REG.captures(input).unwrap();
        let sign = captures.get(2).unwrap().as_str();
        let field = captures.get(1).unwrap().as_str();
        let value = captures.get(3).unwrap().as_str();
        let route = captures.get(4).unwrap().as_str();
        if sign == ">" {
            return Superior(String::from(field), value.parse::<u32>().unwrap(), String::from(route));
        } else if sign == "<" {
            return Inferior(String::from(field), value.parse::<u32>().unwrap(), String::from(route));
        }
        Default(String::from("R"))
    }

    fn apply(&self, part: &Part) -> Option<&str> {
        return match self {
            Default(route) => Some(route),
            Inferior(field, value, route) => {
                if part.get(field).unwrap() < value {
                    return Some(route);
                }
                None
            }
            Superior(field, value, route) => {
                if part.get(field).unwrap() > value {
                    return Some(route);
                }
                None
            }
        };
    }
}

fn parse(input: &str) -> (HashMap<String, Vec<Rule>>, Vec<Part>) {
    static WORKFLOW_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"(\w+)\{(([amsx][><]\d+:\w+,)+)(\w+)}").unwrap());
    let (workflows_input, parts_input) = input.split_once("\n\n").unwrap();
    let workflows = workflows_input.lines().map(|w| {
        let captures = WORKFLOW_REG.captures(w).unwrap();
        let workflow_name = captures.get(1).unwrap().as_str();
        let default_route = captures.iter().last().unwrap().unwrap().as_str();
        let rules_raw = captures.get(2).unwrap().as_str();
        let mut rules = rules_raw
            .split(',')
            .filter(|r| r.len() > 0)
            .map(|r| Rule::from(r))
            .collect_vec();
        rules.push(Default(String::from(default_route)));
        (String::from(workflow_name), rules)
    }).collect_vec();
    let parts = parts_input.lines().map(|l| part_from(l)).collect_vec();
    let works: HashMap<_, _> = workflows.into_iter().collect();
    (works, parts)
}

fn apply_rules(part: &Part, workflows: &HashMap<String, Vec<Rule>>, route: &str) -> String {
    if route == "A" || route == "R" {
        return String::from(route);
    }
    let rules = workflows.get(route).unwrap();
    for rule in rules {
        if let Some(next) = rule.apply(part) {
            return apply_rules(part, workflows, next);
        }
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (workflows, parts) = parse(input);
    let mut result = 0;
    for part in &parts {
        let output = apply_rules(part, &workflows, "in");
        if output == "A" {
            result += part.iter().map(|(_, v)| v).sum::<u32>();
        }
    }
    Some(result)
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
