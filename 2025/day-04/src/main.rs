use glam::IVec2;
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");
    let paper_rolls = parse(input);

    let movable_rolls = paper_rolls
        .iter()
        .filter(|&&roll| get_neighbours(&paper_rolls, roll).count() < 4)
        .count();
    println!("Part 1. {movable_rolls}");

    let mut remaining_rolls = paper_rolls;
    let mut remove_candidates = remaining_rolls.iter().cloned().collect::<VecDeque<_>>();
    let mut remove_count = 0;

    while let Some(candidate) = remove_candidates.pop_front() {
        if remaining_rolls.contains(&candidate) {
            if get_neighbours(&remaining_rolls, candidate).count() < 4 {
                remaining_rolls.remove(&candidate);
                remove_count += 1;
                for n in get_neighbours(&remaining_rolls, candidate) {
                    remove_candidates.push_back(n);
                }
            }
        }
    }
    println!("Part 2. {}", remove_count);
}

fn get_neighbours(rolls: &HashSet<IVec2>, pos: IVec2) -> impl Iterator<Item = IVec2> {
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter_map(move |(dx, dy)| {
            let n = IVec2::new(pos.x + dx, pos.y + dy);
            (n != pos && rolls.contains(&n)).then_some(n)
        })
}

fn parse(s: &str) -> HashSet<IVec2> {
    s.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(x, c)| (c == '@').then_some(IVec2::new(x as i32, y as i32)))
        })
        .collect()
}
