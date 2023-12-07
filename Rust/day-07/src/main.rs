use std::str::FromStr;
use std::cmp::Ordering;
use itertools::Itertools;

// --------------------------------------------------------------------
#[derive(Debug, PartialEq, Ord, Eq)]
struct Card {
    rank: usize,
    symbol: char,
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.rank.cmp(&other.rank))
    }
}

impl Card {
    fn create(symbol: char, joker_rule: bool) -> Card {

        let rank_ordering = match joker_rule {
            false => "23456789TJQKA",
            true => "J23456789TQKA",
        };

        let rank = rank_ordering
                .chars()
                .position(|c| c == symbol)
                .expect("A valid rank");
        Card { rank, symbol: symbol }
    }
}

// --------------------------------------------------------------------
#[derive(Debug, Ord, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: usize,
}

impl Hand {
    fn create(input: &str, joker_rule: bool) -> Hand {
        let (raw_cards, raw_bid) = input.split_once(" ").expect("Row as a pair");

       let cards: Vec<Card> = raw_cards
            .chars()
            .map(|c| Card::create(c, joker_rule))
            .collect();

        let mut hand_type: HandType = raw_cards.parse().expect("A valid hand type");

        if joker_rule {
            hand_type = hand_type.joker_upgrade(raw_cards);
        }

        let bid = raw_bid.parse::<usize>().expect("A valid bid");

        Hand { cards, hand_type, bid }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type != other.hand_type {
            self.hand_type.partial_cmp(&other.hand_type)
        }
        else {
            for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                match c1.partial_cmp(c2) {
                    Some(Ordering::Equal) => continue,
                    _ => return c1.partial_cmp(c2),
                }
            }
            Some(Ordering::Equal)
        }
    }
}
// --------------------------------------------------------------------
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl FromStr for HandType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        assert!(s.len() == 5, "A hand must have five cards");

        let counts = s.chars().counts().values().cloned().collect::<Vec<usize>>();

        if counts.contains(&5) {
            Ok(HandType::Five)
        }
        else if counts.contains(&4) {
            Ok(HandType::Four)
        }
        else if counts.contains(&3) && counts.contains(&2) {
            Ok(HandType::FullHouse)
        }
        else if counts.contains(&3) {
            Ok(HandType::Three)
        }
        else if counts.contains(&2) && counts.len() <= 3 {
            Ok(HandType::TwoPair)
        }
        else if counts.contains(&2) && counts.len() == 4 {
            Ok(HandType::OnePair)
        }
        else if counts.len() == 5 {
            Ok(HandType::HighCard)
        }
        else {
            unreachable!("Should have found a hand by now");
        }
    }
}

impl HandType {
    fn joker_upgrade(&self, card_string: &str) -> HandType {
        // TODO: Improve this ugly thing!
        let unique_non_joker = card_string.chars().filter(|&c| c != 'J').counts().values().len();
        let counts = card_string.chars().counts();
        let joker_count = counts.get(&'J').unwrap_or(&0).clone();

        match (unique_non_joker, joker_count) {
            (_, 5) => HandType::Five,
            (_, 4) => HandType::Five,
            (2, 3) => HandType::Four,
            (1, 3) => HandType::Five,
            (1, 2) => HandType::Five,
            (2, 2) => HandType::Four,
            (3, 2) => HandType::Three,
            (4, 1) => HandType::OnePair,
            (3, 1) => HandType::Three,
            (2, 1) => match self {
                    HandType::TwoPair => HandType::FullHouse,
                    HandType::Three => HandType::Four,
                    _ => unreachable!("Should not be possible!"),
            }
            (1, 1) => HandType::Five,
            _ => *self
        }
    }
}
// --------------------------------------------------------------------

fn total_winnings(hands: &mut Vec<Hand>) -> usize {
    let mut total_winnings = 0;

    hands.sort();


    for (index, hand) in hands.iter().enumerate() {
        let rank = index + 1;
        let winnings = (*hand).bid * rank;
        total_winnings += winnings;
    }

    total_winnings
}

fn solve_part_one(input: &str) {

    let mut hands: Vec<Hand> = input
            .lines()
            .map(|line| Hand::create(line, false))
            .collect();
 
    println!("Part 1. Total winnings = {}", total_winnings(&mut hands));
}

fn solve_part_two(input: &str) {

    let mut hands: Vec<Hand> = input
            .lines()
            .map(|line| Hand::create(line, true))
            .collect();
 
    println!("Part 2. Total winnings = {}", total_winnings(&mut hands));
}

fn main() {
    //let raw_input = include_str!("../sample1-input.txt").trim();
    //let raw_input = include_str!("../sample2-input.txt").trim();
    let raw_input = include_str!("../input.txt").trim();
    solve_part_one(&raw_input);
    solve_part_two(&raw_input);
}
