use itertools::Itertools;
advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<i32> {
    let galaxies = parse_univers(input);
    println!("{:?}", galaxies);
    let expanded_universe = expand_universe(input, galaxies.clone());
    let galaxies_expanded = parse_univers(&expanded_universe);
    println!("{:?}", galaxies_expanded);
    let mut sum = 0;
    for (i, galaxy) in galaxies_expanded.iter().enumerate() {
        for j in i + 1..galaxies_expanded.len() {
            let dist = (galaxies_expanded[j].0 as isize - galaxies_expanded[i].0 as isize).abs() + (galaxies_expanded[j].1 as isize - galaxies_expanded[i].1 as isize).abs();
            sum += dist;
            println!("{:?}, {:?} -> {}", galaxies_expanded[i], galaxies_expanded[j], dist);
        }
    }
    Some(sum as i32)
}

fn expand_universe(input: &str, galaxies: Vec<(usize, usize)>) -> String {
    let lines_to_expand = input
        .lines()
        .enumerate()
        .filter(|(_, l)| !l.contains("#"))
        .map(|(i, _)| i)
        .collect_vec();
    let column_galaxy = galaxies.iter().map(|(_, i)| i).sorted().dedup().collect_vec();
    let mut output = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut line_output = vec![];
        for (x, c) in line.chars().enumerate() {
            line_output.push(c);
            if !column_galaxy.contains(&&x) {
                line_output.push(c);
            }
        }
        let outline = line_output.iter().collect::<String>();
        if lines_to_expand.contains(&y) {
            output.push(outline.clone());
        }
        output.push(outline);
    }
    output.join("\n")
}

fn parse_univers(input: &str) -> Vec<(usize, usize)> {
    let mut output = vec![];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                output.push((y, x));
            }
        }
    }
    output
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
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
