advent_of_code::solution!(1);

fn replace_numbers(str: &str) -> String {
    let mut replace = String::from(str);
    for (number, rep) in [
        ("one", "one1one"),
        ("two", "two2two"),
        ("three", "three3three"),
        ("four", "four4four"),
        ("five", "five5five"),
        ("six", "six6six"),
        ("seven", "seven7seven"),
        ("eight", "eight8eight"),
        ("nine", "nine9nine"),
    ] {
        replace = replace.replace(number, rep);
    }
    replace
}

fn concat(first: char, second: char) -> u32 {
    let mut s = String::from(first);
    s.push(second);
    s.parse::<u32>().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>())
        .map(|v| concat(v[0], v[v.len() - 1]))
        .sum::<u32>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .lines()
        .map(replace_numbers)
        .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<Vec<_>>())
        .map(|v| concat(v[0], v[v.len() - 1]))
        .sum::<u32>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = part_one(str);
        // let result = part_one(&advent_of_code::template::read_file("examples", DAY));    // not valid because the example input changed
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
