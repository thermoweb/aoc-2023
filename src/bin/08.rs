use std::collections::HashMap;

use regex::Regex;

use crate::Direction::{Left, Right};

advent_of_code::solution!(8);

type Map = HashMap<String, Node>;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn from_char(char: char) -> Direction {
        match char {
            'L' => Left,
            'R' => Right,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl Node {
    fn next(&self, direction: &Direction) -> &String {
        match direction {
            Left => &self.left,
            Right => &self.right,
        }
    }
}

fn parse_nodes(input: &str) -> Map {
    let mut nodes = HashMap::new();
    let node_reg = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    input.lines().for_each(|line| {
        let captures = node_reg.captures(line).unwrap();
        let node_name = String::from(&line[captures.get(1).unwrap().range()]);
        let left = String::from(&line[captures.get(2).unwrap().range()]);
        let right = String::from(&line[captures.get(3).unwrap().range()]);
        nodes.insert(node_name, Node { left, right });
    });

    nodes
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input.chars().map(Direction::from_char).collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let (raw_path, raw_nodes) = input.split_once("\n\n").unwrap();
    let nodes = parse_nodes(raw_nodes);
    let directions = parse_directions(raw_path);
    let mut step: u32 = 0;

    let start_node = nodes.get("AAA").unwrap();
    let mut current_node = start_node;
    let mut next_move = &directions[step as usize];
    let mut next_node = current_node.next(next_move);

    while !next_node.eq("ZZZ") {
        step += 1;
        current_node = nodes.get(next_node).unwrap();
        next_move = &directions[step as usize % directions.len()];
        next_node = current_node.next(next_move);
        // println!("{:?} -> {:?} = {:?} ----> {:?}", current_node, next_move, next_node, nodes.get(next_node).unwrap());
    }

    Some(step + 1)
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
