use std::{str::FromStr, collections::HashSet};

struct Card {
    winning_numbers: HashSet<usize>,
    our_numbers: HashSet<usize>,
}

impl FromStr for Card {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = line.split_once(":").unwrap();
        let (winning_numbers, our_numbers) = numbers.split_once("|").unwrap();

        let winning_numbers: HashSet<usize> = winning_numbers
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        let our_numbers: HashSet<usize> = our_numbers
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        Ok( Card { winning_numbers, our_numbers } )
    }
}

impl Card {
    fn correct_numbers_count(&self) -> usize {
        self.winning_numbers.intersection(&self.our_numbers).count()
   }
    fn score(&self) -> usize {
        let correct = self.correct_numbers_count();

        match correct {
            0 => 0,
            _ => 2usize.pow(correct as u32 - 1)
        }
    }
}

fn solve_part_one(cards: &Vec<Card>) {
    let sum: usize = cards.iter().map(|card| card.score()).sum();
    println!("Part 1. Sum = {sum}");
}

fn solve_part_two(cards: &Vec<Card>) {
    let corrects_on_cards: Vec<usize> = cards
        .iter()
        .map(|card| card.correct_numbers_count())
        .collect();

    let mut card_amounts: Vec<usize> = vec![1 ; cards.len()];

    for (card_number, &correct_numbers_on_card) in corrects_on_cards.iter().enumerate() {
        for i in 1..=correct_numbers_on_card {
            card_amounts[card_number + i] += card_amounts[card_number];
        }
    }

    println!("Part 2. Number of cards: {}", card_amounts.iter().sum::<usize>());
}


fn main() {
    let raw_input = include_str!("../input.txt").trim();

    let cards: Vec<Card> = raw_input
        .lines().map(|line| Card::from_str(line).unwrap()).collect();

    solve_part_one(&cards);
    solve_part_two(&cards);
}