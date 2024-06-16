use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let grid = parse_grid(input);
    println!("Part 1. Load = {}", part_one(&grid));
    println!("Part 2. Load = {}", part_two(&grid));
}

fn part_one(grid: &[Vec<Stone>]) -> usize {
    let rotated_grid = rotate_counter_clockwise(grid);
    let shifted_grid = shift_west(&rotated_grid);
    calculate_load(&shifted_grid)
}

fn part_two(grid: &[Vec<Stone>]) -> usize {
    let mut grid = rotate_counter_clockwise(grid);
    let mut seen_grids = HashMap::new();

    let target_cycles = 1_000_000_000;

    for cycle_index in 1..=target_cycles {
        grid = run_cycle(grid);

        if let Some(period_start_index) = seen_grids.get(&grid) {
            let period = cycle_index - period_start_index;
            let remaining_cycles = (target_cycles - period_start_index) % period;

            for _ in 0..remaining_cycles {
                grid = run_cycle(grid);
            }

            return calculate_load(&grid);
        } else {
            seen_grids.insert(grid.clone(), cycle_index);
        }
    }

    calculate_load(&grid)
}

fn run_cycle(grid: Vec<Vec<Stone>>) -> Vec<Vec<Stone>> {
    let mut grid = grid;
    for _ in 0..4 {
        let shifted = shift_west(&grid);
        grid = rotate_clockwise(&shifted);
    }
    grid
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Stone {
    Empty,
    Roller,
    Fixed,
}

fn calculate_load(grid: &[Vec<Stone>]) -> usize {
    grid
        .iter()
        .flat_map(|row| {
            row.iter()
                .enumerate()
                .filter_map(|(index, stone)| match stone {
                    Stone::Roller => Some(row.len() - index),
                    _ => None,
                })
        })
        .sum()
}

fn parse_grid(input: &str) -> Vec<Vec<Stone>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'O' => Stone::Roller,
                    '#' => Stone::Fixed,
                    '.' => Stone::Empty,
                    _ => panic!("Bad input!"),
                })
                .collect()
        })
        .collect_vec()
}

fn draw_grid(grid: &[Vec<Stone>]) {
    println!();
    grid.iter().for_each(|row| {
        let line = row
            .iter()
            .map(|tile| match tile {
                Stone::Empty => '.',
                Stone::Fixed => '#',
                Stone::Roller => 'O',
            })
            .join("");
        println!("{line}");
    })
}

fn shift_west(grid: &[Vec<Stone>]) -> Vec<Vec<Stone>> {
    grid.iter()
        .map(|old_row| {
            let mut row = vec![];

            let mut empties_seen = 0;
            let mut stones_seen = 0;

            for stone in old_row.iter() {
                match stone {
                    Stone::Empty => empties_seen += 1,
                    Stone::Roller => stones_seen += 1,
                    Stone::Fixed => {
                        for _ in 0..stones_seen {
                            row.push(Stone::Roller);
                        }
                        stones_seen = 0;

                        for _ in 0..empties_seen {
                            row.push(Stone::Empty);
                        }
                        empties_seen = 0;

                        row.push(Stone::Fixed);
                    }
                }
            }

            for _ in 0..stones_seen {
                row.push(Stone::Roller);
            }

            let remaining_empty = old_row.len() - row.len();

            for _ in 0..remaining_empty {
                row.push(Stone::Empty);
            }

            row
        })
        .collect_vec()
}

fn rotate_counter_clockwise<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    let grid = rotate_clockwise(grid);
    let grid = rotate_clockwise(&grid);
    rotate_clockwise(&grid)
}

fn rotate_clockwise<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut rotated = vec![vec![grid[0][0]; rows]; cols];

    for (j, i) in (0..rows).cartesian_product(0..cols) {
        rotated[i][rows-1-j] = grid[j][i];
    }

    rotated
}
