use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::Regex;

advent_of_code::solution!(14);

type Grid = Vec<Vec<char>>;

fn get_weight(rocks: &Grid) -> u64 {
    let result = rocks.iter()
        .map(|l|
            l.iter().copied().enumerate()
                .filter(|(_, c)| c == &'O')
                .map(|(i, _)| i + 1)
                .sum::<usize>()
        )
        .sum::<usize>();
    result as u64
}

fn slide(grid: &Grid) -> Grid {
    static ROCKS_REG: Lazy<Regex> = Lazy::new(|| Regex::new(r"(#*)([O.]+)(#*)").unwrap());
    let result = grid.iter()
        .map(|l| l.iter().collect::<String>())
        .map(|l| {
            ROCKS_REG.captures_iter(&l)
                .map(|c| {
                    let start = &l[c.get(1).unwrap().range()];
                    let end = &l[c.get(3).unwrap().range()];
                    let middle_sorted = &l[c.get(2).unwrap().range()].chars().sorted().collect::<String>();
                    return start.to_owned() + &middle_sorted + end;
                })   //c.get(2).unwrap()
                .collect_vec()
                .join("")
        })
        .join("\n");
    to_grid(&result)
}

fn get_grid(input: &str) -> Grid {
    let chars = to_grid(input);
    rotate(&chars)
}

fn to_grid(input: &str) -> Grid {
    let chars = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    chars
}

fn rotate(grid: &Grid) -> Grid {
    let new_grid = (0..grid[0].len())
        .map(|i| grid.iter().map(|row| row[i]).rev().collect_vec())
        .collect_vec();
    new_grid
}

fn print_grid(grid: Grid) {
    let result = grid.iter().map(|l| l.iter().collect::<String>())
        .join("\n");
    println!("==============\n{}", result);
    println!("current weight = {}", get_weight(&grid));
}

fn cycle(mut grid: Grid) -> Grid {
    for _ in 0..4 {
        grid = slide(&grid);
        grid = rotate(&grid);
    }
    grid
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut grid = get_grid(input);
    grid = slide(&grid);
    let result = get_weight(&grid);
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = get_grid(input);
    let mut cache = vec![grid.clone()];
    loop {
        grid = cycle(grid);
        if let Some(i) = cache.iter().position(|g| g == &grid) {
            let cycle_length = cache.len() - i;
            let index = i + (1_000_000_000 - i) % cycle_length;
            return Some(get_weight(&cache[index]));
        }
        cache.push(grid.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
