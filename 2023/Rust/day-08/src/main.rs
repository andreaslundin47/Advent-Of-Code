use std::{
    cmp::{max, min},
    collections::BTreeMap,
};

use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline},
    multi::separated_list1,
    sequence::{delimited, separated_pair},
    IResult,
};

fn main() {
    let raw_input = include_str!("../input.txt").trim();
    solve_part_one(&raw_input);
    solve_part_two(&raw_input);
}

fn solve_part_one(input: &str) {
    let (moves, directions) = parse_input(input);

    let mut current_position = "AAA";

    if !directions.contains_key(current_position) {
        println!("Part 1. Starting position 'AAA' is not within the input");
        return;
    }

    for (step, lr_move) in moves.chars().cycle().enumerate() {
        if current_position == "ZZZ" {
            println!("Part 1. Steps to reach 'ZZZ': {}", step);
            return;
        }

        let node = directions.get(current_position).expect("Node should exist");

        current_position = match lr_move {
            'L' => node.left,
            'R' => node.right,
            _dir => panic!("Unexpected direction: {_dir}"),
        }
    }
}

fn solve_part_two(input: &str) {
    let (moves, directions) = parse_input(input);

    let positions_ending_in_a = directions
        .keys()
        .cloned()
        .filter(|&name| name.chars().last() == Some('A'))
        .collect::<Vec<&str>>();

    let mut cycle_lengths = vec![];

    for start_position in positions_ending_in_a {
        let mut current_position = start_position;
        let mut visited_states: BTreeMap<(&str, usize), usize> = BTreeMap::new();

        for (global_index, (cycle_index, lr_move)) in moves.chars().enumerate().cycle().enumerate()
        {
            let current_state = (current_position, cycle_index);

            if visited_states.contains_key(&current_state) {
                let previous_step = visited_states.get(&current_state).unwrap();
                let cycle_length = global_index - previous_step;
                cycle_lengths.push(cycle_length);
                break;
            }

            visited_states.insert(current_state, global_index);

            let node = directions.get(current_position).expect("Node should exist");

            current_position = match lr_move {
                'L' => node.left,
                'R' => node.right,
                _dir => panic!("Unexpected direction: {_dir}"),
            }
        }
    }

    let steps: usize = cycle_lengths.into_iter().reduce(|a, b| lcm(a, b)).unwrap();
    println!("Part 2. All at nodes ending in 'Z' at step: {steps}");
}

fn lcm(a: usize, b: usize) -> usize {
    let greater = max(a, b);
    let smallest = min(a, b);

    let mut i = greater;

    while i % smallest != 0 {
        i += greater;
    }

    return i;
}

struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_input(input: &str) -> (&str, BTreeMap<&str, Node>) {
    let (moves, raw_directions) = input.split_once("\n\n").unwrap();

    let (_, nodes) =
        separated_list1(newline, parse_node)(raw_directions).expect("Valid parse of nodes");

    let directions = nodes
        .into_iter()
        .map(|node| (node.name, node))
        .collect::<BTreeMap<&str, Node>>();

    (moves, directions)
}

fn parse_node(i: &str) -> IResult<&str, Node> {
    let (i, name) = alphanumeric1(i)?;
    let (i, _) = tag(" = ")(i)?;
    let (i, (left, right)) = parse_children(i)?;
    Ok((i, Node { name, left, right }))
}

fn parse_children(i: &str) -> IResult<&str, (&str, &str)> {
    let (input, left_right) = delimited(
        tag("("),
        separated_pair(alphanumeric1, tag(", "), alphanumeric1),
        tag(")"),
    )(i)?;
    Ok((input, left_right))
}
