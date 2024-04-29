use itertools::Itertools;
use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../input.txt").trim();
    let operations = parse(input);

    part_one(&operations);
    part_two(&operations);
}

fn part_one(operations: &[Operation]) {
    let mut grid = [[false; 1000]; 1000];

    for op in operations.iter() {
        for x in op.x_range.clone() {
            for y in op.y_range.clone() {
                match op.kind {
                    Kind::On => grid[y][x] = true,
                    Kind::Off => grid[y][x] = false,
                    Kind::Toggle => grid[y][x] = !grid[y][x],
                }
            }
        }
    }

    let on_count: usize = grid
        .iter()
        .map(|row| row.iter().filter(|v| **v).count())
        .sum();

    println!("Part 1. {on_count}");
}

fn part_two(operations: &[Operation]) {
    let mut grid = [[0u8; 1000]; 1000];

    for op in operations.iter() {
        for x in op.x_range.clone() {
            for y in op.y_range.clone() {
                grid[y][x] = match op.kind {
                    Kind::On => grid[y][x] + 1,
                    Kind::Off => grid[y][x].saturating_sub(1),
                    Kind::Toggle => grid[y][x] + 2,
                }
            }
        }
    }

    let on_count: usize = grid
        .iter()
        .map(|row| row.iter().map(|v| *v as usize).sum::<usize>())
        .sum();

    println!("Part 2. {on_count}");
}

fn parse(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();

            let (from, to, kind) = match parts[1] {
                "on" => (2, 4, Kind::On),
                "off" => (2, 4, Kind::Off),
                _ => (1, 3, Kind::Toggle),
            };

            let (x1, y1): (usize, usize) = parts[from]
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            let (x2, y2): (usize, usize) = parts[to]
                .split(',')
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();

            Operation {
                kind,
                x_range: x1..=x2,
                y_range: y1..=y2,
            }
        })
        .collect()
}

#[derive(Debug)]
enum Kind {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
struct Operation {
    kind: Kind,
    x_range: RangeInclusive<usize>,
    y_range: RangeInclusive<usize>,
}
