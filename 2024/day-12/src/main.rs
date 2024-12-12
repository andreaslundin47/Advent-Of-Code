use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;
use itertools::Itertools;

const NORTH: IVec2 = IVec2::NEG_Y;
const SOUTH: IVec2 = IVec2::Y;
const WEST: IVec2 = IVec2::NEG_X;
const EAST: IVec2 = IVec2::X;

struct RegionData {
    area: usize,
    perimeter_length: usize,
    sides_count: usize,
}

fn main() {
    let input = include_str!("../input.txt").trim();
    let nodes = parse(input);

    let regions: Vec<RegionData> = nodes
        .clone()
        .drain()
        .fold(
            (Vec::new(), HashSet::new()),
            |(mut regions, mut visited), (position, region): (IVec2, char)| {
                if !visited.contains(&position) {
                    let (region_nodes, region_data) = get_region(position, region, &nodes);
                    regions.push(region_data);
                    visited.extend(region_nodes);
                }

                (regions, visited)
            },
        )
        .0;

    let cost_one: usize = regions.iter().map(|r| r.area * r.perimeter_length).sum();
    println!("Part 1. Cost: {}", cost_one);

    let cost_two: usize = regions.iter().map(|r| r.area * r.sides_count).sum();
    println!("Part 2. Cost: {}", cost_two);
}

fn get_region(
    start: IVec2,
    start_region: char,
    all_nodes: &HashMap<IVec2, char>,
) -> (HashSet<IVec2>, RegionData) {
    let mut seen = HashSet::from([start]);
    let mut perimeter_edges = HashSet::new();

    let mut queue = VecDeque::from([start]);

    while let Some(position) = queue.pop_front() {
        for offset in [NORTH, SOUTH, WEST, EAST] {
            let neighbour = position + offset;

            if let Some(neighbour_region) = all_nodes.get(&neighbour) {
                let in_same_region = *neighbour_region == start_region;

                if in_same_region {
                    let is_new_node = !seen.contains(&neighbour);

                    if is_new_node {
                        seen.insert(neighbour);
                        queue.push_back(neighbour);
                    }
                } else {
                    perimeter_edges.insert((offset, position));
                }
            } else {
                // Outside map, so is an edge
                perimeter_edges.insert((offset, position));
            }
        }
    }
    let area = seen.len();
    let perimeter_length = perimeter_edges.len();

    let sides_count: usize = perimeter_edges
        .into_iter()
        .into_group_map()
        .iter()
        .map(|(&direction, positions)| match direction {
            NORTH | SOUTH => positions
                .iter()
                .map(|&IVec2 { x, y }| (y, x))
                .into_group_map()
                .values()
                .map(|x_with_same_y| count_contiguous_segments(x_with_same_y))
                .sum(),
            WEST | EAST => positions
                .iter()
                .map(|&IVec2 { x, y }| (x, y))
                .into_group_map()
                .values()
                .map(|y_with_same_x| count_contiguous_segments(y_with_same_x))
                .sum(),
            _ => 0,
        })
        .sum();

    let region_data = RegionData {
        area,
        perimeter_length,
        sides_count,
    };

    (seen, region_data)
}

fn count_contiguous_segments(parts: &[i32]) -> usize {
    parts
        .iter()
        .sorted()
        .tuple_windows()
        .filter(|(&a, &b)| a != b - 1)
        .count()
        + 1
}

fn parse(input: &str) -> HashMap<IVec2, char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| (IVec2::new(x as i32, y as i32), c))
        })
        .collect()
}
