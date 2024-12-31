use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let data = parse(input);

    part_one(&data);

    // Part 2 solved manually 
}

fn part_one(data: &Data) {
    let mut values = data.init_values.clone();
    let mut gates = VecDeque::from(data.gates.clone());

    while let Some(gate) = gates.pop_front() {
        if let (Some(&in1), Some(&in2)) = (values.get(&gate.in1), values.get(&gate.in2)) {
            let out = match gate.op {
                Operation::And => in1 & in2,
                Operation::Or => in1 | in2,
                Operation::Xor => in1 ^ in2,
            };

            values.insert(gate.out, out);
        } else {
            gates.push_back(gate);
        }
    }

    let z_bits: Vec<usize> = values
        .into_iter()
        .filter(|(name, _)| name.starts_with("z"))
        .sorted()
        .rev()
        .map(|(_, val)| val)
        .collect();

    let decimal_output = z_bits.iter().fold(0, |acc, &x| 2 * acc + x);

    println!("Part 1. {}", decimal_output);
}

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    And,
    Xor,
    Or,
}

#[derive(Debug, Clone)]
struct Gate {
    out: String,
    in1: String,
    in2: String,
    op: Operation,
}

struct Data {
    init_values: HashMap<String, usize>,
    gates: Vec<Gate>,
}

fn parse(input: &str) -> Data {
    let (values, gates) = input.split_once("\n\n").unwrap();

    let init_values = values
        .lines()
        .map(|line| {
            let (name, value) = line.split_once(": ").unwrap();
            let name = name.to_string();
            let value = value.parse::<usize>().unwrap();
            (name, value)
        })
        .collect();

    let gates = gates
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let in1 = parts[0].to_string();
            let in2 = parts[2].to_string();
            let out = parts[4].to_string();

            let op = match parts[1] {
                "AND" => Operation::And,
                "XOR" => Operation::Xor,
                "OR" => Operation::Or,
                _ => panic!("Bad input!"),
            };

            Gate { out, in1, in2, op }
        })
        .collect();

    Data { init_values, gates }
}
