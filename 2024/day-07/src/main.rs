fn main() {
    let input = include_str!("../input.txt").trim();
    let equations = parse(input);

    let sum: usize = equations
        .iter()
        .filter_map(|eq| is_valid_one(eq).then_some(eq.result))
        .sum();

    println!("Part 1. Sum: {}", sum);

    let sum: usize = equations
        .iter()
        .filter_map(|eq| is_valid_two(eq).then_some(eq.result))
        .sum();

    println!("Part 2. Sum: {}", sum);
}

fn is_valid_one(equation: &Equation) -> bool {
    can_equate_one(equation.result, 0, &equation.numbers)
}

fn is_valid_two(equation: &Equation) -> bool {
    can_equate_two(equation.result, 0, &equation.numbers)
}

fn can_equate_one(target: usize, acc: usize, remaining: &[usize]) -> bool {
    if remaining.len() == 0 {
        return acc == target;
    }

    if acc > target {
        return false;
    }

    let multiplied = acc * remaining[0];

    if can_equate_one(target, multiplied, &remaining[1..]) {
        return true;
    }

    let added = acc + remaining[0];

    if can_equate_one(target, added, &remaining[1..]) {
        return true;
    }

    false
}

fn can_equate_two(target: usize, acc: usize, remaining: &[usize]) -> bool {
    if remaining.len() == 0 {
        return acc == target;
    }

    if acc > target {
        return false;
    }

    if can_equate_two(target, concatenate(acc, remaining[0]), &remaining[1..]) {
        return true;
    }

    if can_equate_two(target, acc * remaining[0], &remaining[1..]) {
        return true;
    }

    if can_equate_two(target, acc + remaining[0], &remaining[1..]) {
        return true;
    }

    false
}

fn concatenate(a: usize, b: usize) -> usize {
    let mut exp = 0;
    while b / 10usize.pow(exp) != 0 {
        exp += 1;
    }

    if b == 0 {
        exp = 1;
    }

    a * 10usize.pow(exp) + b
}

struct Equation {
    result: usize,
    numbers: Vec<usize>,
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();
            let result = left.parse().unwrap();
            let numbers = right
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();

            Equation { result, numbers }
        })
        .collect()
}
