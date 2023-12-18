use once_cell::sync::Lazy;
use regex::Regex;

use crate::Operation::{Place, Remove};

advent_of_code::solution!(15);

#[derive(Debug)]
struct Lens<'a> {
    label: &'a str,
    focal: i32,
}

#[derive(Debug)]
enum Operation<'a> {
    Remove(&'a str),
    Place(Lens<'a>),
}

impl<'a> Operation<'a> {
    fn from_str(input: &str) -> Operation {
        static OPT_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\w+)([=-])([1-9])?$").unwrap());
        let wtf = OPT_REG.captures(input);
        let captures = wtf.unwrap();
        let label = &input[captures.get(1).unwrap().range()];
        let lens_power_group = captures.get(3);
        return if lens_power_group.is_some() {
            let lens_power = input[captures.get(3).unwrap().range()].parse::<i32>().unwrap();
            Place(Lens { label: label, focal: lens_power })
        } else {
            Remove(label)
        };
    }
}

fn apply_hash(input: &str) -> u64 {
    let mut current = 0;
    for letter in input.replace('\n', "").replace('\r', "").chars() {
        current += letter as u64;
        current *= 17;
        current %= 256;
    }
    current
}

pub fn part_one(input: &str) -> Option<u64> {
    input.split(',')
        .map(|s| apply_hash(s))
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    const LENS_BOX: Vec<Lens> = Vec::new();
    let mut lens_boxes = [LENS_BOX; 256];   //[Vec<Lens>; 256]
    for operation in input.split(',').map(|o| Operation::from_str(o)) {
        match operation {
            Remove(label) => {
                let _ = &lens_boxes[apply_hash(label) as usize].retain(|l| l.label != label);
            }
            Place(lens) => {
                let box_number = apply_hash(lens.label);
                let lens_box = &mut lens_boxes[box_number as usize];
                if let Some(previous) = lens_box.iter_mut().find(|l| l.label == lens.label) {
                    previous.focal = lens.focal;
                } else {
                    lens_box.push(lens);
                }
            }
        }
    }
    let result = lens_boxes.iter().enumerate()
        .filter(|(_, b)| b.len() > 0)
        .map(|(box_number, lenses)|
            lenses.iter().enumerate()
                .map(|(slot, lens)| (1 + box_number) * (slot + 1) * lens.focal as usize)
                .sum::<usize>()
        )
        .sum::<usize>();
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_apply_hash() {
        let result = apply_hash(&"HASH");
        assert_eq!(result, 52);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
