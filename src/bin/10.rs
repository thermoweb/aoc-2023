use std::collections::HashMap;

use itertools::Itertools;
use colored::Colorize;
use regex::Regex;

use crate::CellType::{Ground, Pipe, Start};
use crate::Direction::{East, North, South, West};

advent_of_code::solution!(10);

type Grid = Vec<Vec<Cell>>;

#[derive(Debug)]
struct Maze {
    grid: Grid,
    start: Option<Cell>,
}

impl Maze {
    fn from(input: &str) -> Maze {
        let mut grid: Vec<_> = vec![];
        let mut start = None;
        for (l, line) in input.lines().enumerate() {
            let mut line_cells: Vec<_> = vec![];
            for (c, column) in line.chars().enumerate() {
                let cell = Cell { cell_type: CellType::from(column), coords: (l, c) };
                if column == 'S' {
                    start = Some(cell.clone());
                }
                line_cells.push(cell);
            }
            grid.push(line_cells);
        }

        Maze { grid, start }
    }
    fn get_next_cells(&self, cell: (usize, usize)) -> HashMap<crate::Direction, &crate::Cell> {
        self.get_neighbours(&self.grid[cell.0][cell.1])
    }

    fn get_neighbours(&self, cell: &Cell) -> HashMap<Direction, &Cell> {
        let mut neighbours = HashMap::new();

        if cell.coords.0 > 0 {
            neighbours.insert(North, &self.grid[cell.coords.0 - 1][cell.coords.1]);
        }

        if cell.coords.0 < self.grid.len() - 1 {
            neighbours.insert(South, &self.grid[cell.coords.0 + 1][cell.coords.1]);
        }

        if cell.coords.1 > 0 {
            neighbours.insert(West, &self.grid[cell.coords.0][cell.coords.1 - 1]);
        }

        if cell.coords.1 < self.grid[cell.coords.0].len() - 1 {
            neighbours.insert(East, &self.grid[cell.coords.0][cell.coords.1 + 1]);
        }

        neighbours
    }
}

#[derive(Debug, Clone)]
struct Cell {
    cell_type: CellType,
    coords: (usize, usize),
}

impl Cell {
    fn is_pipe(&self) -> bool {
        match self.cell_type {
            Pipe(_, _) => true,
            _ => false,
        }
    }

    fn can_go(&self, from: &Direction) -> bool {
        if let Pipe(source, target) = &self.cell_type {
            if from == source || from == target {
                return true;
            }
        } else if self.cell_type == Start {
            return true;
        }
        false
    }
}

#[derive(Debug, Clone, PartialEq)]
enum CellType {
    Pipe(Direction, Direction),
    Ground,
    Start,
}

impl CellType {
    fn from(char: char) -> CellType {
        match char {
            '.' => Ground,
            'S' => Start,
            '|' => Pipe(North, South),
            '-' => Pipe(West, East),
            'L' => Pipe(North, East),
            'J' => Pipe(North, West),
            '7' => Pipe(South, West),
            'F' => Pipe(South, East),
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            North => South,
            East => West,
            South => North,
            West => East,
        }
    }
}

fn get_path(input: &str) -> Vec<(usize, usize)> {
    let maze = Maze::from(input);
    //println!("{:?}", maze);
    let start = &maze.start.clone().unwrap();
    // println!("starting in {:?}", start.coords);
    let neighbours = maze.get_neighbours(&start);
    let pipes = neighbours.iter().filter(|(d, c)| c.is_pipe() && c.can_go(&d.opposite())).collect_vec();
    let mut next = pipes[0].1.coords.clone();
    let mut steps = vec![start.coords];
    while next != start.coords {
        // println!("I'm in {:?}", next);
        let mut possible_direct = vec![];
        if let Pipe(from, to) = &maze.grid[next.0][next.1].cell_type {
            // println!("{:?}, {:?}", from, to);
            possible_direct.push(from);
            possible_direct.push(to);
        }
        let new_cell = next;
        next = maze.get_next_cells(next)
            .iter()
            // .inspect(|(d,c)| println!("go {:?} to {:?} from {:?} ? -> {} - {:?} {:?}", d, c.coords, next, c.can_go(&d.opposite()), c, possible_direct.contains(d)))
            .filter(|(d, c)| c.can_go(&d.opposite()) && possible_direct.contains(d) && c.coords != *steps.last().unwrap())
            // .inspect(|(d,c)| println!("going {:?} {:?} from {:?} -> {:?}", d, c.coords, next, c))
            .collect_vec()[0].1.coords;
        // println!("next = {:?}", next);
        steps.push(new_cell);
    }
    steps
}

pub fn part_one(input: &str) -> Option<u32> {
    let steps = get_path(input);
    // println!("{:?}.len() = {}", steps, steps.len());
    Some((steps.len() / 2) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let bottom_u_reg = Regex::new(r"L(-*)J").unwrap();
    let top_u_reg = Regex::new(r"F(-*)7").unwrap();
    let cross_fj_reg = Regex::new(r"F(-*)J").unwrap();
    let cross_l7_reg = Regex::new(r"L(-*)7").unwrap();
    let steps = get_path(input);
    let mut inner_cells = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut crossed = 0;
        let mut new_line = top_u_reg.replace_all(&line.replace("S", "J"), "-$1-").to_string();
        new_line = bottom_u_reg.replace_all(&new_line, "-$1-").to_string();
        new_line = cross_fj_reg.replace_all(&new_line, "|$1-").to_string();
        new_line = cross_l7_reg.replace_all(&new_line, "|$1-").to_string();
        for (x, c) in new_line.chars().enumerate() {
            if steps.contains(&(y, x)) {
                if c != '-' {
                    crossed += 1;
                }
                print!("{}", c.to_string().green());
            } else {
                if crossed % 2 == 1 {
                    inner_cells.push((y, x));
                    print!("{}", "I".to_string().red());
                } else {
                    print!("{}", ".".to_string().white());
                }
            }
        }
        println!();
    }
    // print_maze(input, steps, inner_cells.clone());
    Some(inner_cells.len() as u32)
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
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let result = part_two(input);
        assert_eq!(result, Some(8));
    }
}
