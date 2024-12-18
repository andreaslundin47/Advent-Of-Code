use std::collections::{HashSet, VecDeque};

use glam::IVec2;

fn main() {
    let input = include_str!("../input.txt").trim();
    let bytes: Vec<IVec2> = parse(input);
    let goal = IVec2::new(70, 70);

    // Part 1
    let blocks: HashSet<&IVec2> = bytes.iter().take(1024).collect();
    let dist = shortest_distance(goal, &blocks).unwrap();
    println!("Part 1. Shortest path: {dist}");

    // Part 2
    let byte_indicies: Vec<usize> = (0..bytes.len()).collect();

    let index_of_blocking = byte_indicies.partition_point(|&i| {
        let blocks = bytes.iter().take(i + 1).collect();
        shortest_distance(goal, &blocks).is_some()
    });

    let first_blocking_byte = bytes[index_of_blocking];
    println!("Part 2. First blocking byte: {first_blocking_byte}");
}

fn shortest_distance(goal: IVec2, blocks: &HashSet<&IVec2>) -> Option<i32> {
    let start = IVec2::new(0, 0);
    let mut seen = HashSet::from([start]);
    let mut queue = VecDeque::from([(start, 0)]);

    while let Some((current, current_dist)) = queue.pop_front() {
        if current == goal {
            return Some(current_dist);
        }

        for direction in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
            let neighbour = current + direction;

            if neighbour.x < 0 || neighbour.y < 0 || neighbour.x > goal.x || neighbour.y > goal.y {
                continue;
            }

            if blocks.contains(&neighbour) {
                continue;
            }

            if seen.contains(&neighbour) {
                continue;
            }

            seen.insert(neighbour);
            queue.push_back((neighbour, current_dist + 1));
        }
    }

    None
}

fn parse(input: &str) -> Vec<IVec2> {
    input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();

            IVec2::new(x, y)
        })
        .collect()
}
