use itertools::Itertools;

fn main() {
    let raw_input = include_str!("../input.txt").trim();
    let (durations, records) = parse_input(&raw_input);
    solve_part_one(&durations, &records);
    solve_part_two(&durations, &records);
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    let (durations, records) = input
        .lines()
        .map(|line| {
            line
                .split_once(":")
                .map(|(_label, numbers_part)| {
                    numbers_part
                        .split_whitespace()
                        .map(|number| {
                            number
                                .parse::<u32>()
                                .expect("Should parse a single integer")
                        })
                })
                .expect("Should have a parsed list of integers")
                .collect::<Vec<u32>>()
        })
        .collect_tuple::<(Vec<u32>, Vec<u32>)>()
        .expect("Should have collected a tuple of integer vectors");

    (durations, records)
}

fn solve_part_one(durations: &Vec<u32>, records: &Vec<u32>) {
    let possibilities_per_race: Vec<usize> = durations
            .iter()
            .zip(records.iter())
            .map(|(&duration, &record_distance)| {
                (0..duration).filter(|t| {
                    let velocity = t;
                    let moving_duration = duration - t;
                    let distance = velocity * moving_duration;

                    distance > record_distance
                })
                .count()
            })
            .collect::<Vec<usize>>();

        let product: usize = possibilities_per_race.iter().product();
        println!("Part 1. Product is {product}");
}

fn solve_part_two(durations: &Vec<u32>, records: &Vec<u32>) {
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
    
    let winning_ways: usize = (0..duration)
            .filter(|t| {
                let velocity = t;
                let moving_duration = duration - t;
                let distance = velocity * moving_duration;
                
                distance > record_distance
            })
            .count();
    
    println!("Part 2. Ways to win {winning_ways}");
}