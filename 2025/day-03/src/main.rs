use std::cmp::Reverse;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();

    let sequences: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let sum1 = sequences.iter().map(|seq| joltage(seq, 2)).sum::<usize>();
    println!("Part 1. {sum1}");
    let sum2 = sequences.iter().map(|seq| joltage(seq, 12)).sum::<usize>();
    println!("Part 2. {sum2}");
}

fn joltage(seq: &[usize], number_of_digits: usize) -> usize {
    let (joltage_number, _) =
        (1..=number_of_digits).fold((0, 0), |(joltage_number, start_index), digit_number| {
            let (digit, Reverse(i)) = seq
                .iter()
                .dropping(start_index)
                .dropping_back(number_of_digits - digit_number)
                .enumerate()
                .map(|(i, &digit)| (digit, Reverse(i)))
                .max()
                .unwrap();

            (10 * joltage_number + digit, start_index + i + 1)
        });

    joltage_number
}
