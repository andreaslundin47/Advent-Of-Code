use std::str::FromStr;
use regex::Regex;

struct Card {
    winning_numbers: Vec<u32>,
    numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(": ").collect();
        let num_parts: Vec<&str> = parts[1].split(" | ").collect();

        let pattern: Regex = Regex::new(r"\d+").expect("Should be a valid regexp");

        let winning_numbers: Vec<u32>= pattern
            .find_iter(num_parts[0])
            .map(|c| c.as_str().parse().expect("Should parse an u32"))
            .collect();

        let numbers: Vec<u32>= pattern
            .find_iter(num_parts[1])
            .map(|c| c.as_str().parse().expect("Should parse an u32"))
            .collect();

        Ok( Card { winning_numbers, numbers } )
    }
}

impl Card {
    fn count_winning_numbers(&self) -> usize {
        let matches: usize = self.winning_numbers
            .iter()
            .filter(|w| self.numbers.contains(w))
            .count();
        
        matches
    }
    fn score(&self) -> u32 {
        let wins: usize = self.count_winning_numbers();

        match wins {
            0 => 0,
            _ => 2u32.pow(wins as u32 - 1)
        }
    }
}

fn solve_part_one(cards: &Vec<Card>) {
    let sum: u32 = cards.iter().map(|c| c.score()).sum();
    println!("Part 1. Sum = {sum}");
}

fn solve_part_two(cards: &Vec<Card>) {
    let win_counts: Vec<usize> = cards
        .iter()
        .map(|c| c.count_winning_numbers())
        .collect();

    let mut card_counts: Vec<u32> = vec![1 ; cards.len()];

    for i in 0..card_counts.len() {
        let card_wins: usize = win_counts[i];

        for j in 1..=card_wins {
            if (i+j) < cards.len() {
                card_counts[i+j] = card_counts[i+j] + card_counts[i]
            }
        }
    }

    let total_cards: u32 = card_counts.iter().sum();   

    println!("Part 2. Number of cards: {total_cards}");
}


fn main() {
    let raw_input = include_str!("../input.txt").trim();

    let cards: Vec<Card> = raw_input
        .lines()
        .map(|line| Card::from_str(line).expect("Should parse a card"))
        .collect();

    solve_part_one(&cards);
    solve_part_two(&cards);
}
