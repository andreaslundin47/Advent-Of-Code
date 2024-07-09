use itertools::{Itertools, Position};
use std::cmp::Ordering;

// --------------------------------------------------------------------
fn main() {
    let raw_input = include_str!("../input.txt").trim();

    let winnings_one: usize = raw_input
        .lines()
        .map(|line| {
            let (cards, bid) = get_hand(line);
            Hand::new(cards, bid)
        })
        .sorted()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum();
    println!("Part 1. Winnings = {winnings_one}");

    let winnings_two: usize = raw_input
        .lines()
        .map(|line| {
            let (cards, bid) = get_hand(line);
            Hand::new_with_joker_rule(cards, bid)
        })
        .sorted()
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum();
    println!("Part 2. Winnings = {winnings_two}");
}

fn get_hand(line: &str) -> (&str, usize) {
    let (cards, bid) = line.split_once(' ').unwrap();
    (cards, bid.parse::<usize>().unwrap())
}
// --------------------------------------------------------------------
#[derive(PartialEq, Eq)]
struct Hand {
    hand_type: HandType,
    cards: Vec<Card>,
    bid: usize,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            self.hand_type.cmp(&other.hand_type)
        } else {
            self.cards.cmp(&other.cards)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(cards: &str, bid: usize) -> Self {
        Self {
            hand_type: HandType::new(cards),
            cards: cards.chars().map(Card::new).collect(),
            bid,
        }
    }

    fn new_with_joker_rule(cards: &str, bid: usize) -> Self {
        Self {
            hand_type: HandType::new_with_joker_rule(cards),
            cards: cards.chars().map(Card::new_with_joker_rule).collect(),
            bid,
        }
    }
}
// --------------------------------------------------------------------
#[derive(PartialEq, Eq)]
struct Card {
    rank: usize,
    char: char,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Card {
    fn new(char: char) -> Self {
        let rank = match char {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            _ => char.to_digit(10).unwrap() as usize,
        };

        Self { rank, char }
    }

    fn new_with_joker_rule(char: char) -> Self {
        let rank = match char {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            _ => char.to_digit(10).unwrap() as usize,
        };

        Self { rank, char }
    }
}
// --------------------------------------------------------------------
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl HandType {
    fn from_dist(dist: &str) -> Self {
        match dist {
            "5" => HandType::Five,
            "41" => HandType::Four,
            "32" => HandType::FullHouse,
            "311" => HandType::Three,
            "221" => HandType::TwoPair,
            "2111" => HandType::OnePair,
            "11111" => HandType::HighCard,
            _ => unreachable!("Impossible with 5 cards"),
        }
    }

    fn new(cards: &str) -> Self {
        assert_eq!(5, cards.len());
        let counts = cards.chars().counts().values().sorted().rev().join("");
        HandType::from_dist(&counts)
    }

    fn new_with_joker_rule(cards: &str) -> Self {
        match cards.chars().counts().get(&'J') {
            None => HandType::new(cards),
            Some(jokers) if jokers == &5 => HandType::Five,
            Some(jokers) => {
                let dist = cards
                    .chars()
                    .counts()
                    .iter()
                    .filter_map(|(value, count)| (value != &'J').then_some(count))
                    .sorted()
                    .rev()
                    .with_position()
                    .map(|(pos, count)| match pos {
                        Position::First | Position::Only => count + jokers,
                        _ => *count,
                    })
                    .join("");

                HandType::from_dist(&dist)
            }
        }
    }
}
