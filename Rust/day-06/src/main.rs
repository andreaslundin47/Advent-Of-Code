use itertools::Itertools;
use std::{collections::HashMap, iter::from_fn};

fn main() {
    let input = include_str!("../input.txt").trim();

    let mut row_iterators: Vec<_> = input.lines().map(|line| line.chars()).collect();

    let counts: Vec<HashMap<char, usize>> = from_fn(|| {
        let column_counts = row_iterators.iter_mut().filter_map(|c| c.next()).counts();

        (!column_counts.is_empty()).then_some(column_counts)
    })
    .collect();

    let message_one = counts
        .iter()
        .map(|col_count| {
            col_count
                .iter()
                .sorted_by_key(|letter_and_count| letter_and_count.1)
                .map(|(letter, _)| letter)
                .last()
                .unwrap()
        })
        .join("");

    println!("Part 1. Message = {}", message_one);

    let message_two = counts
        .iter()
        .map(|col_count| {
            col_count
                .iter()
                .sorted_by_key(|letter_and_count| letter_and_count.1)
                .map(|(letter, _)| letter)
                .next()
                .unwrap()
        })
        .join("");

    println!("Part 2. Message = {}", message_two);
}
