use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

fn main() {
    let raw_input = include_str!("../input.txt").trim();
    let topographical_map = parse(raw_input);
    let trail_heads = find_trail_heads(&topographical_map);

    let evaluations: Vec<Evaluation> = trail_heads
        .iter()
        .map(|&head| evaluate_trail_head(head, &topographical_map))
        .collect();

    let score: usize = evaluations.iter().map(|ev| ev.score).sum();
    println!("Part 1. Total score: {}", score);

    let rating: usize = evaluations.iter().map(|ev| ev.rating).sum();
    println!("Part 2. Total rate:  {}", rating);
}

fn parse(input: &str) -> HashMap<IVec2, u8> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().map(move |(x, c)| {
                let height = c.to_digit(10).unwrap() as u8;
                (IVec2::new(x as i32, y as i32), height)
            })
        })
        .collect()
}

fn find_trail_heads(topo_map: &HashMap<IVec2, u8>) -> Vec<IVec2> {
    topo_map
        .iter()
        .filter(|(_, &height)| height == 0)
        .map(|(&vec, _)| vec)
        .collect()
}

struct Evaluation {
    score: usize,
    rating: usize,
}

fn evaluate_trail_head(head: IVec2, topo_map: &HashMap<IVec2, u8>) -> Evaluation {
    let mut queue = VecDeque::from([(head, 0)]);
    let mut seen = HashSet::from([head]);
    let mut score = 0;
    let mut rating = 0;

    while let Some((current_pos, current_height)) = queue.pop_front() {
        if current_height == 9 {
            rating += 1;
        }

        for offset in [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y] {
            let neighbour_pos = current_pos + offset;

            if let Some(&neighbour_height) = topo_map.get(&neighbour_pos) {
                if neighbour_height != current_height + 1 {
                    continue;
                }

                queue.push_back((neighbour_pos, neighbour_height));

                if !seen.contains(&neighbour_pos) && neighbour_height == 9 {
                    seen.insert(neighbour_pos);
                    score += 1;
                }
            }
        }
    }

    Evaluation { score, rating }
}
