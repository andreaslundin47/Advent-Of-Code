use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let maze = Maze::parse(input);

    let mut reduced_graph = HashMap::new();

    for key_node in maze.key_nodes.keys() {
        for (pair, dist) in maze.bfs(*key_node) {
            reduced_graph.insert(pair, dist);
        }
    }

    let wire_key_names: Vec<char> = maze
        .key_nodes
        .values()
        .filter(|name| name != &&'0')
        .cloned()
        .collect();

    let shortest_dist_one: usize = wire_key_names
        .iter()
        .permutations(wire_key_names.len())
        .map(|perm| {
            std::iter::once(&'0')
                .chain(perm)
                .tuple_windows()
                .map(|(node_start, node_end)| {
                    reduced_graph
                        .get(&(*node_start, *node_end))
                        .expect("Should exist")
                })
                .sum()
        })
        .min()
        .expect("At least one distance");

    println!("Part 1. Shortest path = {}", shortest_dist_one);

    let shortest_dist_two: usize = wire_key_names
        .iter()
        .permutations(wire_key_names.len())
        .map(|perm| {
            std::iter::once(&'0')
                .chain(perm)
                .chain(std::iter::once(&'0'))
                .tuple_windows()
                .map(|(node_start, node_end)| {
                    reduced_graph
                        .get(&(*node_start, *node_end))
                        .expect("Should exist")
                })
                .sum()
        })
        .min()
        .expect("At least one distance");

    println!("Part 2. Shortest path = {}", shortest_dist_two);
}

#[derive(Debug)]
struct Maze {
    nodes: HashSet<IVec2>,
    key_nodes: HashMap<IVec2, char>,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let mut nodes = HashSet::new();
        let mut key_nodes = HashMap::new();

        for (y, row) in input.lines().enumerate() {
            for (x, node) in row.chars().enumerate() {
                if node == '#' {
                    continue;
                }

                let pos = IVec2::new(x as i32, y as i32);
                nodes.insert(pos);

                if node != '.' {
                    key_nodes.insert(pos, node);
                }
            }
        }

        Maze { nodes, key_nodes }
    }

    fn neighbours(&self, current: &IVec2) -> Vec<IVec2> {
        let offsets = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];
        offsets
            .into_iter()
            .map(|off| current + off)
            .filter(|neighbour| self.nodes.contains(neighbour))
            .collect()
    }

    fn bfs(&self, start: IVec2) -> HashMap<(char, char), usize> {
        let start_name = *self.key_nodes.get(&start).expect("key node to extist");

        let mut seen = HashSet::from([start]);
        let mut queue = VecDeque::from([(start, 0)]);
        let mut pair_dists = HashMap::new();

        while let Some((current_node, dist)) = queue.pop_front() {
            if self.key_nodes.contains_key(&current_node) && current_node != start {
                if let Some(&name) = self.key_nodes.get(&current_node) {
                    pair_dists.insert((start_name, name), dist);
                }
            }

            for neighbour in self.neighbours(&current_node) {
                if !seen.contains(&neighbour) {
                    queue.push_back((neighbour, dist + 1));
                    seen.insert(neighbour);
                }
            }
        }

        pair_dists
    }
}
