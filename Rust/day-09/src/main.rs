fn main() {
    let raw_input = include_str!("../input.txt").trim();
    let sequences = parse_input(&raw_input);
    solve_part_one(&sequences);
    solve_part_two(&sequences);
}

fn solve_part_one(sequences: &Vec<Vec<i32>>) {
    let sum: i32 = sequences.iter().map(|seq| next_value(seq)).sum();
    println!("Part 1. Sum = {sum}");
}

fn solve_part_two(sequences: &Vec<Vec<i32>>) {
    let sum: i32 = sequences.iter().map(|seq| preceeding_value(seq)).sum();
    println!("Part 2. Sum = {sum}");
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let sequences = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse().expect("Valid i32"))
                .collect()
        })
        .collect();

    sequences
}

fn next_value(sequence: &Vec<i32>) -> i32 {
    if sequence.iter().all(|&value| value == 0) {
        0
    } else {
        let diffs: Vec<i32> = sequence
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        sequence.last().expect("Should have a last element").clone() + next_value(&diffs)
    }
}

fn preceeding_value(sequence: &Vec<i32>) -> i32 {
    let rev_seq = sequence.iter().cloned().rev().collect();
    next_value(&rev_seq)
}