use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(12);

fn get_pattern(input: &str) -> Vec<usize> {
    static OPERATIONAL_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.+").unwrap());
    let binding = OPERATIONAL_REG.replace_all(input, ".");
    binding
        .split('.')
        .map(|p| p.len())
        .filter(|p| p > &0)
        .collect_vec()
}

fn possible_records(input: Vec<String>, pattern: Vec<usize>) -> Vec<String> {
    let mut output = vec![];

    let record_to_process = input.iter()
        .filter(|s| s.contains('?'));
    if record_to_process.clone().count() == 0 {
        return input;
    }
    let processed_records = record_to_process
        .flat_map(|s| vec![s.replacen('?', ".", 1), s.replacen('?', "#", 1)])
        .collect_vec();
    output.extend(processed_records);
    return possible_records(output, pattern);
}

fn get_valid_records(input: Vec<String>, pattern: Vec<usize>) -> Vec<String> {
    return possible_records(input, pattern.clone()).iter()
        .filter(|p| get_pattern(p) == pattern)
        .map(|s| String::from(s))
        .collect_vec();
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input.lines()
        .map(|l| l.split_once(' ').unwrap())
        .map(|(record, expected)| {
            let exp = expected.split(',').map(|n| n.parse::<usize>().unwrap()).collect_vec();
            (exp.clone(), get_valid_records(vec![String::from(record)], exp))
        })
        // .inspect(|(exp, records)| println!("{:?} , {:?}", exp, records))
        .map(|(_, records)| records.iter().count())
        .sum::<usize>();
    // println!("{:?}", result);
    Some(result as u32)
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
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_possible_records() {
        let input = vec![String::from("???.###")];
        let pattern = vec![1, 1, 3];
        let result = possible_records(input, pattern);
        println!("{:?}", result);
    }

    #[test]
    fn test_get_pattern() {
        assert_eq!(get_pattern("####.#...#..."), vec![4, 1, 1]);
    }
}
