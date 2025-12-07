use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

fn main() {
    let input = include_str!("../input.txt").trim();
    let world = parse(input);

    let mut seen = HashMap::from([(world.start, 1)]);
    let mut queue = VecDeque::new();
    queue.push_back(world.start);

    let mut split_count: usize = 0;
    let mut timelines: usize = 0;

    while let Some(current_pos) = queue.pop_front() {
        let current_paths = *seen
            .get(&current_pos)
            .expect("A beam in the queue should be in seen");

        if current_pos.y >= world.height {
            timelines += current_paths;
            continue;
        }

        let underneath_pos = current_pos + IVec2::Y;
        let is_above_splitter = world.splitters.contains(&underneath_pos);

        if is_above_splitter {
            split_count += 1;
        }

        let neighbour_deltas = if is_above_splitter {
            vec![IVec2::new(-1, 1), IVec2::new(1, 1)]
        } else {
            vec![IVec2::new(0, 1)]
        };

        for delta in neighbour_deltas {
            let neighbour = current_pos + delta;

            if !seen.contains_key(&neighbour) {
                queue.push_back(neighbour);
            }

            seen.entry(neighbour)
                .and_modify(|paths_count| *paths_count += current_paths)
                .or_insert(current_paths);
        }
    }

    println!("Part 1. {split_count}");
    println!("Part 2. {timelines}");
}

struct World {
    splitters: HashSet<IVec2>,
    start: IVec2,
    height: i32,
}

fn parse(s: &str) -> World {
    let mut start = IVec2::ZERO;
    let mut height = 0;
    let mut splitters = HashSet::new();

    for (y, row) in s.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            match c {
                'S' => start = IVec2::new(x as i32, y as i32),
                '^' => {
                    splitters.insert(IVec2::new(x as i32, y as i32));
                }
                _ => (),
            };
        }
        height = y as i32 + 1;
    }

    World {
        splitters,
        start,
        height,
    }
}
