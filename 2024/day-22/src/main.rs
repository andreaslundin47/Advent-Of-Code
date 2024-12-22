use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let seeds: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
    part_one(&seeds);
    part_two(&seeds);
}

fn part_one(seeds: &Vec<usize>) {
    let sum: usize = seeds
        .iter()
        .map(|&seed| {
            let mut number = seed;
            for _ in 0..2000 {
                number = next_number(number);
            }
            number
        })
        .sum();

    println!("Part 1. Sum of 2000th generated numbers: {}", sum);
}

fn part_two(seeds: &Vec<usize>) {
    let mut total_bananas_per_change: HashMap<Change, usize> = HashMap::new();

    for &seed in seeds {
        let changes = get_changes_and_bananas(seed);

        for (change, bananas) in changes {
            *total_bananas_per_change.entry(change).or_insert(0) += bananas as usize;
        }
    }

    if let Some((_change, bananas)) = total_bananas_per_change
        .iter()
        .max_by_key(|(_, &bananas)| bananas)
    {
        println!("Part 2. Maximum bananas: {}", bananas);
    }
}

fn next_number(number: usize) -> usize {
    let prune_number = 16_777_216;

    let a1 = 64 * number;
    let a2 = a1 ^ number;
    let a3 = a2 % prune_number;

    let b1 = a3 >> 5;
    let b2 = b1 ^ a3;
    let b3 = b2 % prune_number;

    let c1 = b3 << 11;
    let c2 = c1 ^ b3;
    let c3 = c2 % prune_number;

    c3
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Change {
    signature: (i8, i8, i8, i8),
}

fn get_changes_and_bananas(seed: usize) -> HashMap<Change, i8> {
    let iterations = 2000;

    let numbers: Vec<usize> = (0..iterations).fold(vec![seed], |mut acc, _| {
        acc.push(next_number(acc[acc.len() - 1]));
        acc
    });

    let prices: Vec<i8> = numbers.iter().map(|num| (num % 10) as i8).collect();

    let price_diffs: Vec<Change> = prices
        .iter()
        .tuple_windows()
        .map(|(&a, &b, &c, &d, &e)| Change {
            signature: (b - a, c - b, d - c, e - d),
        })
        .collect();

    let diffs_and_prices: Vec<(Change, i8)> =
        std::iter::zip(price_diffs.into_iter(), prices.into_iter().skip(4)).collect();

    let mut bananas_for_first_change = HashMap::new();

    for (change, price) in diffs_and_prices {
        if !bananas_for_first_change.contains_key(&change) {
            bananas_for_first_change.insert(change, price);
        }
    }

    bananas_for_first_change
}
