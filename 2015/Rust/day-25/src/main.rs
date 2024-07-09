use std::num::ParseIntError;

const MULTIPLIER: usize = 252533;
const DIVISOR: usize = 33554393;
const FIRST_NUMBER: usize = 20151125;

fn main() {
    let input = include_str!("../input.txt").trim();
    let (row, col) = parse(input).expect("Valid parse");

    let ordinal = position_to_ordinal(row, col);
    let number = calculate_number(ordinal);

    println!("Part 1. Number {} has value = {}", ordinal, number);
}

fn position_to_ordinal(row: usize, col: usize) -> usize {
    let n = row + col - 2;
    (n * (1 + n)) / 2 + col
}

fn calculate_number(n: usize) -> usize {
    let mut output = FIRST_NUMBER;

    for _ in 2..=n {
        output = (output * MULTIPLIER) % DIVISOR;
    }

    output
}

fn parse(i: &str) -> Result<(usize, usize), ParseIntError> {
    let parts: Vec<&str> = i.split_whitespace().collect();
    let row = parts[parts.len()-3].trim_end_matches(",").parse::<usize>()?;
    let col = parts[parts.len()-1].trim_end_matches(".").parse::<usize>()?;

    Ok( (row, col) )
}

