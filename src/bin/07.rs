use itertools::Itertools;

use crate::Card::*;
use crate::HandType::*;

advent_of_code::solution!(7);

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn from_cards(cards: &Vec<Card>) -> HandType {
        let jokers = get_num_of_jokers(cards.clone());
        let hand: Vec<_> = cards
            .iter()
            .sorted()
            .dedup()
            .map(|card| {
                let total = cards
                    .iter()
                    .filter(|c| c.eq(&card))
                    .count();
                if !Joker.eq(card) {
                    total + jokers
                } else {
                    total
                }
            })
            .sorted()
            .rev()
            .collect::<Vec<_>>();
        let best_cards = (hand.get(0).unwrap_or(&0), get_unique_count(cards.clone()));
        match best_cards {
            (5, _) => FiveKind,
            (4, _) => FourKind,
            (3, 2) => FullHouse,
            (3, _) => ThreeKind,
            (2, 3) => TwoPair,
            (2, _) => OnePair,
            _ => HighCard,
        }
    }
}

fn get_num_of_jokers(cards: Vec<Card>) -> usize {
    cards.iter().filter(|c| Joker.eq(c)).count()
}

fn get_unique_count(cards: Vec<Card>) -> usize {
    let hand_without_jokers = cards.iter().filter(|c| !Joker.eq(c));
    hand_without_jokers
        .sorted()
        .dedup()
        .count()
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn from(input: &str, j: Card) -> Hand {
        let (cards_raw, bid_raw) = input.split_once(' ').unwrap();
        let cards = cards_raw.chars().map(|c| Card::from_str(c, j.to_owned()).unwrap()).collect::<Vec<_>>();
        let hand_type = HandType::from_cards(&cards);
        let bid = bid_raw.parse::<u32>().unwrap();
        Hand { cards, hand_type, bid }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
enum Card {
    Ace,
    King,
    Queen,
    Janitor,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl Card {
    fn from_str(str: char, j: Card) -> Option<Card> {
        match str {
            'A' => Some(Ace),
            'K' => Some(King),
            'Q' => Some(Queen),
            'J' => Some(j),
            'T' => Some(Ten),
            '9' => Some(Nine),
            '8' => Some(Eight),
            '7' => Some(Seven),
            '6' => Some(Six),
            '5' => Some(Five),
            '4' => Some(Four),
            '3' => Some(Three),
            '2' => Some(Two),
            _ => None
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let result: Vec<_> = input
        .lines()
        .map(|l| Hand::from(l, Janitor))
        .sorted()
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (rank, hand) in result.iter().rev().enumerate() {
        sum = sum + (rank as u32 + 1) * hand.bid;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result: Vec<_> = input
        .lines()
        .map(|l| Hand::from(l, Joker))
        .sorted()
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (rank, hand) in result.iter().rev().enumerate() {
        sum = sum + (rank as u32 + 1) * hand.bid;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
