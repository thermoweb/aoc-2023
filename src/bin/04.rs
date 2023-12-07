use std::collections::HashMap;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Card {
    winning_numbers: Vec<i32>,
    card_numbers: Vec<i32>,
    num_of_copy: i32,
}

impl Card {
    fn get_score(&self) -> u32 {
        let wins = self.get_winning_numbers();
        if wins == 0 {
            return 0;
        }
        2_u32.pow(wins - 1)
    }

    fn get_winning_numbers(&self) -> u32 {
        self.card_numbers
            .iter()
            .filter(|n| self.winning_numbers.contains(n))
            .count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let splits = input
        .lines()
        .map(|lines| {
            let card_split = lines.split(": ").collect::<Vec<_>>();
            let card = card_split.last().unwrap().split(" | ").collect::<Vec<_>>();
            Card {
                winning_numbers: card
                    .first()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
                card_numbers: card
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
                num_of_copy: 1,
            }
        })
        .map(|c| c.get_score())
        .sum::<u32>();
    Some(splits)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut won_card = HashMap::new();
    let splits = input
        .lines()
        .map(|l| {
            let card_split = l.split(": ").collect::<Vec<_>>();
            let card = card_split.last().unwrap().split(" | ").collect::<Vec<_>>();
            let card_id = card_split
                .first()
                .map(|c| c.split_whitespace().last().unwrap())
                .unwrap()
                .parse::<i32>()
                .unwrap();
            let card = Card {
                winning_numbers: card
                    .first()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
                card_numbers: card
                    .last()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
                num_of_copy: 1 + won_card.get(&card_id).unwrap_or(&0),
            };
            let num_winned = card.get_winning_numbers() as i32;
            for i in card_id + 1..card_id + 1 + num_winned {
                let current_value = won_card.get(&i).unwrap_or(&0);
                // println!("win {} copies of card {}", (1 * card.num_of_copy), i);
                won_card.insert(i, current_value + card.num_of_copy);
            }
            card
        })
        // .inspect(|c| println!("{:?}", c))
        .map(|c| c.num_of_copy as u32)
        .sum::<u32>();
    Some(splits)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
