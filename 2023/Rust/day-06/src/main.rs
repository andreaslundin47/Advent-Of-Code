use itertools::Itertools;

fn main() {
    let raw_input = include_str!("../input.txt").trim();
    let (durations, records) = parse_input(&raw_input);
    solve_part_one(&durations, &records);
    solve_part_two(&durations, &records);
}
// ----------------------------------------------------------------------------
fn solve_part_one(durations: &Vec<usize>, records: &Vec<usize>) {
    let possibilities_per_race: Vec<usize> = durations
        .iter()
        .zip(records.iter())
        .map(|(&duration, &record_distance)| count_valid_options(duration, record_distance))
        .collect::<Vec<usize>>();

    let product: usize = possibilities_per_race.iter().product();
    println!("Part 1. Product is {product}");
}
// ----------------------------------------------------------------------------
fn solve_part_two(durations: &Vec<usize>, records: &Vec<usize>) {
    let duration: usize = durations
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<usize>()
        .expect("Should concatenate the numbers");

    let record_distance: usize = records
        .iter()
        .map(|n| n.to_string())
        .collect::<String>()
        .parse::<usize>()
        .expect("Should concatenate the numbers");

    let winning_ways: usize = count_valid_options(duration, record_distance);
    println!("Part 2. Ways to win {winning_ways}");
}
// ----------------------------------------------------------------------------
fn count_valid_options(duration: usize, record_distance: usize) -> usize {
    let (start, end) = solve_for_interval(duration, record_distance);
    end - start + 1
}

//   t * (d - t) = r => dt - t^2 = r => t^2 - dt + r = 0
//   => t = d/2 +- sqrt(d^2/4 - r)
fn solve_for_interval(duration: usize, record: usize) -> (usize, usize) {
    let (d, r) = (duration as f64, record as f64);
    let left = d / 2.0 - (d.powi(2) / 4.0 - r).sqrt();
    let right = d / 2.0 + (d.powi(2) / 4.0 - r).sqrt();

    (left.ceil() as usize, right.floor() as usize)
}
// ----------------------------------------------------------------------------
fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (durations, records) = input
        .lines()
        .map(|line| {
            line.split_once(":")
                .map(|(_label, numbers_part)| {
                    numbers_part.split_whitespace().map(|number| {
                        number
                            .parse::<usize>()
                            .expect("Should parse a single integer")
                    })
                })
                .expect("Should have a parsed list of integers")
                .collect::<Vec<usize>>()
        })
        .collect_tuple::<(Vec<usize>, Vec<usize>)>()
        .expect("Should have collected a tuple of integer vectors");

    (durations, records)
}
// ----------------------------------------------------------------------------
