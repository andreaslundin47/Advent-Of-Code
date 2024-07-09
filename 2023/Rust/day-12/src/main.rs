use std::collections::HashMap;

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{digit1, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult, Parser,
};

fn main() {
    //let input = include_str!("../sample1-input.txt").trim();
    let input = include_str!("../input.txt").trim();
    let records = parse(input);
    part_one(&records);
    part_two(&records);
}

fn part_one(records: &[ConditionRecord]) {
    let sum: usize = records
        .iter()
        .map(|record| {
            let mut memory = HashMap::new();
            count_combinations(record.clone(), &mut memory)
        })
        .sum();
    println!("Part 1. Sum of combination counts = {sum}");
}

fn part_two(records: &[ConditionRecord]) {
    let sum: usize = records
        .iter()
        .map(|record| {
            let sequence = [record.sequence; 5].join("?");
            let groups = record.groups.repeat(5);

            let extended_record = ConditionRecord {
                sequence: &sequence,
                groups,
            };

            let mut memory = HashMap::new();
            count_combinations(extended_record, &mut memory)
        })
        .sum();
    println!("Part 2. Sum of combination counts = {sum}");
}

fn count_combinations(record: ConditionRecord, memory: &mut HashMap<SearchState, usize>) -> usize {
    let sequence: &str = record.sequence;
    let groups: &Vec<usize> = &record.groups;

    let state = SearchState {
        seq: sequence.to_string(),
        groups: groups.clone(),
    };

    if let Some(count) = memory.get(&state) {
        return *count;
    }

    if groups.is_empty() {
        if sequence.contains('#') {
            return 0;
        } else {
            return 1;
        }
    }

    let minimum_space_needed = groups.iter().sum::<usize>() + groups.len() - 1;

    if sequence.len() < minimum_space_needed {
        return 0;
    }

    let beginning = &sequence[..groups[0]];

    let first_is_not_broken = !sequence.starts_with('#');
    let is_valid_group_placement = beginning.chars().all(|c| c != '.');
    let is_valid_ending = groups.len() == 1 || sequence.chars().nth(groups[0]).unwrap() != '#';
    let can_place_first_group = is_valid_group_placement && is_valid_ending;

    let mut count = 0;

    if can_place_first_group {
        let shift = groups[0] + if groups.len() > 1 { 1 } else { 0 };
        let next_record = ConditionRecord {
            sequence: &sequence[shift..],
            groups: groups[1..].to_vec(),
        };
        count += count_combinations(next_record, memory);
    }

    if first_is_not_broken {
        let next_not_placing = ConditionRecord {
            sequence: &sequence[1..],
            groups: groups.to_vec(),
        };
        count += count_combinations(next_not_placing, memory);
    }

    memory.insert(state, count);

    count
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct SearchState {
    seq: String,
    groups: Vec<usize>,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct ConditionRecord<'a> {
    sequence: &'a str,
    groups: Vec<usize>,
}

fn parse(i: &str) -> Vec<ConditionRecord> {
    let (_, records) = separated_list1(newline, record)(i)
        .finish()
        .expect("A valid parse");

    records
}

fn record(i: &str) -> IResult<&str, ConditionRecord> {
    let (i, (sequence, groups)) = separated_pair(
        is_a(".?#"),
        space1,
        separated_list1(
            tag(","),
            digit1.map(|d: &str| d.parse::<usize>().expect("An integer")),
        ),
    )(i)?;

    Ok((i, ConditionRecord { sequence, groups }))
}
