use std::iter::successors;

fn main() {
    let input = include_str!("../input.txt").trim();
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let sum: u32 = input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));

            let first = digits.next().unwrap();

            if let Some(last) = digits.last() {
                10 * first + last
            } else {
                11 * first
            }
        })
        .sum();

    println!("Part 1: Sum = {sum}");
}

fn part_two(input: &str) {
    let digit_words = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let sum: usize = input
        .lines()
        .map(|line| {
            let line_substrings = successors(Some(line), |s| (s.len() > 1).then_some(&s[1..]));

            let mut digits = line_substrings.filter_map(|sub| {
                if let Some(index) = digit_words.iter().position(|word| sub.starts_with(word)) {
                    Some(index + 1)
                } else {
                    let first_char = sub.chars().next().unwrap();
                    first_char.to_digit(10).map(|d| d as usize)
                }
            });

            let first = digits.next().unwrap();

            if let Some(last) = digits.last() {
                10 * first + last
            } else {
                11 * first
            }
        })
        .sum();

    println!("Part 2: Sum = {sum}");
}
