use priority_queue::PriorityQueue;
use std::{cmp::Reverse, collections::HashSet};

const PUZZLE_INPUT: i32 = 1350;
const TARGET: (i32, i32) = (31, 39);
const DISTANCE_LIMIT: i32 = 50;

fn main() {
    let start = (1, 1);

    let mut within_distance = 0;
    let mut target_distance = None;

    let mut seen = HashSet::new();
    seen.insert(start);

    let mut queue = PriorityQueue::new();
    queue.push(start, Reverse(0));

    while let Some((current, Reverse(current_distance))) = queue.pop() {
        if current_distance > DISTANCE_LIMIT && target_distance.is_some() {
            break;
        }

        if current_distance <= DISTANCE_LIMIT {
            within_distance += 1;
        }

        if current == TARGET {
            target_distance = Some(current_distance);
        }

        for neighbour in neighbours(current) {
            if !seen.contains(&neighbour) && is_open_space(neighbour) {
                queue.push(neighbour, Reverse(current_distance + 1));
                seen.insert(neighbour);
            }
        }
    }

    println!("Part 1. Steps = {}", target_distance.unwrap());
    println!("Part 2. Within distance = {}", within_distance);
}

fn is_open_space((x, y): (i32, i32)) -> bool {
    let mut number = x * x + 3 * x + 2 * x * y + y + y * y + PUZZLE_INPUT;
    let mut ones = 0;

    while number > 0 {
        if number % 2 == 1 {
            ones += 1;
        }
        number >>= 1;
    }

    ones % 2 == 0
}

fn neighbours((x, y): (i32, i32)) -> Vec<(i32, i32)> {
    let offsets = [(1, 0), (-1, 0), (0, 1), (0, -1)];

    offsets
        .into_iter()
        .filter_map(|(dx, dy)| {
            let (xx, yy) = (x + dx, y + dy);
            (xx >= 0 && yy >= 0).then_some((xx, yy))
        })
        .collect()
}
