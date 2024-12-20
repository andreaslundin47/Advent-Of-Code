use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let world = parse(input);

    println!("Part 1. 100+ cheats: {}", count_cheats(&world, 2, 2));
    println!("Part 2. 100+ cheats: {}", count_cheats(&world, 1, 20));
}

fn count_cheats(world: &World, cheat_min_len: usize, cheat_max_len: usize) -> usize {
    let start_dists = get_distances(world.start, &world);
    let goal_dists = get_distances(world.goal, &world);

    let fair_shortest = *goal_dists
        .get(&world.start)
        .expect("This distance should exist");

    let mut hundred_plus_cheats = 0;

    for (x, y) in (1..world.width - 1).cartesian_product(1..world.height - 1) {
        let cheat_start = IVec2::new(x, y);

        if !world.is_path(&cheat_start) {
            continue;
        }

        let cheat_exits = get_cheat_paths(cheat_start, cheat_min_len, cheat_max_len, &world);

        for (cheat_exit, cheat_length) in cheat_exits {
            let st = start_dists.get(&cheat_start);
            let ex = goal_dists.get(&cheat_exit);

            if let (Some(&st), Some(&ex)) = (st, ex) {
                let cheating_shortest = st + cheat_length + ex;

                if cheating_shortest < fair_shortest {
                    let cheating_save = fair_shortest - cheating_shortest;

                    if cheating_save >= 100 {
                        hundred_plus_cheats += 1;
                    }
                }
            }
        }
    }

    hundred_plus_cheats
}

fn get_distances(start: IVec2, world: &World) -> HashMap<IVec2, usize> {
    let mut queue = VecDeque::from([(start, 0)]);
    let mut distances = HashMap::from([(start, 0)]);

    while let Some((current, curent_dist)) = queue.pop_front() {
        for direction in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
            let neighbour = current + direction;

            if world.is_path(&neighbour) && !distances.contains_key(&neighbour) {
                distances.insert(neighbour, curent_dist + 1);
                queue.push_back((neighbour, curent_dist + 1));
            }
        }
    }

    distances
}

fn get_cheat_paths(
    start: IVec2,
    min_len: usize,
    max_len: usize,
    world: &World,
) -> Vec<(IVec2, usize)> {
    let mut seen = HashSet::from([start]);
    let mut queue = VecDeque::from([(start, 0)]);
    let mut cheats = Vec::new();

    while let Some((current, current_dist)) = queue.pop_front() {
        if current_dist > max_len {
            break;
        }

        if current_dist >= min_len && world.is_path(&current) {
            cheats.push((current, current_dist));
        }

        for direction in [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y] {
            let neighbour = current + direction;

            if world.contains(&neighbour) && !seen.contains(&neighbour) {
                seen.insert(neighbour);
                queue.push_back((neighbour, current_dist + 1));
            }
        }
    }

    cheats
}

#[derive(Debug)]
struct World {
    walls: HashSet<IVec2>,
    width: i32,
    height: i32,
    start: IVec2,
    goal: IVec2,
}

impl World {
    fn contains(&self, node: &IVec2) -> bool {
        node.x >= 0 && node.y >= 0 && node.x < self.width && node.y < self.height
    }

    fn is_path(&self, node: &IVec2) -> bool {
        self.contains(node) && !self.walls.contains(node)
    }
}

fn parse(input: &str) -> World {
    let mut walls = HashSet::new();
    let mut start = IVec2::new(-1, -1);
    let mut goal = IVec2::new(-1, -1);
    let mut width = -1;
    let mut height = -1;

    for (y, row) in input.lines().enumerate() {
        width = row.len() as i32;
        height = y as i32 + 1;

        for (x, c) in row.chars().enumerate() {
            let vec = IVec2::new(x as i32, y as i32);
            match c {
                '#' => {
                    walls.insert(vec);
                }
                'S' => {
                    start = vec;
                }
                'E' => {
                    goal = vec;
                }
                _ => (),
            }
        }
    }

    World {
        walls,
        start,
        goal,
        width,
        height,
    }
}
