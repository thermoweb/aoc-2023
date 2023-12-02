use std::collections::HashMap;

advent_of_code::solution!(2);

const BLUE_THRESHOLD: i32 = 14;
const RED_THRESHOLD: i32 = 12;
const GREEN_THRESHOLD: i32 = 13;
/*
const THRESHOLDS: HashMap<&str, i32> = HashMap::from([
    ("blue", 14),
    ("blue", 14),
    ("blue", 14)]);
*/
#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

impl Game {
    pub fn is_possible(&self) -> bool {
        self.sets.iter().filter(|s| !s.possible).count() == 0
    }

    pub fn get_green(&self) -> u32 {
        self.sets.iter().map(|s| s.green).max().unwrap() as u32
    }

    pub fn get_red(&self) -> u32 {
        self.sets.iter().map(|s| s.red).max().unwrap() as u32
    }

    pub fn get_blue(&self) -> u32 {
        self.sets.iter().map(|s| s.blue).max().unwrap() as u32
    }
}

#[derive(Debug, Clone)]
struct Set {
    blue: i32,
    red: i32,
    green: i32,
    possible: bool,
}

fn map_set(str: &str) -> Set {
    let mut colors = HashMap::new();
    str.split(", ").for_each(|s| {
        let vec = s.trim().split(" ").collect::<Vec<_>>();
        let n = vec[0].parse::<i32>().unwrap();
        let c = vec[1];
        colors.insert(c, n);
    }); // blue = s.split(" ").collect::<Vec<_>>()[0].parse::<i32>().unwrap()
    let blue = colors.get("blue").or(Some(&0)).unwrap().clone();
    let red = colors.get("red").or(Some(&0)).unwrap().clone();
    let green = colors.get("green").or(Some(&0)).unwrap().clone();
    let possible = blue <= BLUE_THRESHOLD && red <= RED_THRESHOLD && green <= GREEN_THRESHOLD;
    Set {
        blue,
        red,
        green,
        possible,
    }
}

fn map_games(str: &str) -> Game {
    let splits: Vec<_> = str
        .split(":")
        .collect::<Vec<_>>();
    let id = splits[0].split(" ").collect::<Vec<_>>().last().unwrap().parse::<i32>().unwrap();
    let sets: &Vec<_> = &splits[1]
        .split(";")
        .map(map_set)
        .collect();
    let game = Game { id: id.clone(), sets: sets.to_vec() };
    //println!("{:?}", game);
    game
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(map_games)
        .filter(Game::is_possible)
        .map(|g| g.id as u32)
        .sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(map_games)
        .map(|g| g.get_green() * g.get_red() * g.get_blue())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
