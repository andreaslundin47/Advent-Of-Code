use glam::IVec2;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt").trim();
    let world = World::parse(&input);
    solve_part_one(&world, 64);
    solve_part_two(&world);
}

#[derive(Debug)]
struct World {
    plots: HashSet<IVec2>,
    start: IVec2,
    width: usize,
}

impl World {
    fn parse(input: &str) -> World {
        let mut plots = HashSet::new();
        let mut start: Option<IVec2> = None;

        for (y, row) in input.lines().enumerate() {
            for (x, c) in row.chars().enumerate() {
                if c != '#' {
                    plots.insert(IVec2::new(x as i32, y as i32));
                    if c == 'S' {
                        start = Some(IVec2::new(x as i32, y as i32));
                    }
                }
            }
        }

        let width = plots.iter().map(|p| p.x).max().unwrap() as usize + 1;
        let start = start.expect("A starting position");

        World {
            plots,
            start,
            width,
        }
    }

    fn valid_neighbours(&self, pos: IVec2) -> Vec<IVec2> {
        let IVec2 { x, y } = pos;
        vec![
            IVec2::new(x - 1, y),
            IVec2::new(x + 1, y),
            IVec2::new(x, y - 1),
            IVec2::new(x, y + 1),
        ]
        .into_iter()
        .filter(|v| self.plots.contains(&v))
        .collect()
    }
}

fn bfs(world: &World, start: &IVec2) -> HashMap<IVec2, usize> {

        let mut queue: Vec<IVec2> = vec![*start];
        let mut seen: HashSet<IVec2> = HashSet::from([*start]);
        let mut distances: HashMap<IVec2, usize> = HashMap::from([(*start, 0)]);

        while !queue.is_empty() {
            let current = queue.remove(0);

            for neighbour in world.valid_neighbours(current) {
                if !seen.contains(&neighbour) {
                    queue.push(neighbour);
                    seen.insert(neighbour);
                    distances.insert(neighbour, distances.get(&current).unwrap() + 1);
                }
            }
        }

        distances
}

fn solve_part_one(world: &World, target_steps: i32) {
    let dists = bfs(world, &world.start);
    let count = dists.values().filter(|d| **d % 2 == 0 && **d <= 64).count();
    println!("Part 1. Number of unique plots {target_steps} steps away are {count}");
}

fn solve_part_two(world: &World) {
    let target_steps: usize = 26_501_365;

    let dists = bfs(&world, &world.start);

    let even_corners = dists.values().filter(|d| **d % 2 == 0 && **d > 65).count();
    let odd_corners = dists.values().filter(|d| **d % 2 == 1 && **d > 65).count();

    let even_full = dists.values().filter(|d| **d % 2 == 0).count();
    let odd_full = dists.values().filter(|d| **d % 2 == 1).count();

    // n is how many times the world repeats after leaving the starting one
    let n = (target_steps - (world.width / 2)) / world.width;

    let sum = (n+1).pow(2) * odd_full + n.pow(2) * even_full - (n+1) * odd_corners + n * even_corners;

    println!("Part 2. {sum}");
}