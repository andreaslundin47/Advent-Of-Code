use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt").trim();
    let gates: HashMap<String, Operation> = parse(input);
    let destination = Number::from("a");

    let mut store: HashMap<String, u16> = HashMap::new();
    let output_one = get_value(&destination, &gates, &mut store);
    println!("Part 1. {output_one}");

    store.clear();
    store.insert("b".to_string(), output_one);
    let output_two = get_value(&destination, &gates, &mut store);
    println!("Part 2. {output_two}");
}

enum Number {
    Literal(u16),
    Named(String),
}

impl From<&str> for Number {
    fn from(value: &str) -> Self {
        if let Ok(number) = value.parse::<u16>() {
            Number::Literal(number)
        } else {
            Number::Named(value.to_string())
        }
    }
}

enum Operation {
    Assign(Number),
    And(Number, Number),
    Or(Number, Number),
    LShift(Number, Number),
    RShift(Number, Number),
    Not(Number),
}

fn parse(input: &str) -> HashMap<String, Operation> {
    input
        .lines()
        .map(|line| {
            let (left, destination) = line
                .split_once(" -> ")
                .expect("Should have left and right side");

            let destination: String = destination.to_string();

            if left.starts_with("NOT") {
                let source = left.strip_prefix("NOT ").unwrap();
                return (destination, Operation::Not(Number::from(source)));
            }

            let parts: Vec<&str> = left.split(' ').collect();

            if parts.len() == 1 {
                return (destination, Operation::Assign(Number::from(parts[0])));
            }

            let op = parts[1];

            match op {
                "AND" => (
                    destination,
                    Operation::And(Number::from(parts[0]), Number::from(parts[2])),
                ),
                "OR" => (
                    destination,
                    Operation::Or(Number::from(parts[0]), Number::from(parts[2])),
                ),
                "LSHIFT" => (
                    destination,
                    Operation::LShift(Number::from(parts[0]), Number::from(parts[2])),
                ),
                "RSHIFT" => (
                    destination,
                    Operation::RShift(Number::from(parts[0]), Number::from(parts[2])),
                ),
                _ => panic!("Parsing a strange operation..."),
            }
        })
        .collect()
}

fn get_value(
    output: &Number,
    gates: &HashMap<String, Operation>,
    store: &mut HashMap<String, u16>,
) -> u16 {
    match output {
        Number::Literal(literal_value) => *literal_value,
        Number::Named(output_name) => {
            if let Some(stored_value) = store.get(output_name) {
                return *stored_value;
            }

            let gate: &Operation = gates.get(output_name).expect("Output should have a gate!");

            let output: u16 = match gate {
                Operation::Assign(source) => get_value(source, gates, store),
                Operation::And(left, right) => {
                    let left = get_value(left, gates, store);
                    let right = get_value(right, gates, store);
                    left & right
                }
                Operation::Or(left, right) => {
                    let left = get_value(left, gates, store);
                    let right = get_value(right, gates, store);
                    left | right
                }
                Operation::LShift(source, amount) => {
                    let source = get_value(source, gates, store);
                    let amount = get_value(amount, gates, store);
                    source << amount
                }
                Operation::RShift(source, amount) => {
                    let source = get_value(source, gates, store);
                    let amount = get_value(amount, gates, store);
                    source >> amount
                }
                Operation::Not(source) => {
                    let source = get_value(source, gates, store);
                    !source
                }
            };
            store.insert(output_name.clone(), output);
            output
        }
    }
}
