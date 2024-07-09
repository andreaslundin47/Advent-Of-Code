use glam::{IVec2, IVec3};
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

fn main() {
    let input = include_str!("../input.txt").trim();

    let bricks: Vec<Brick> = parse_bricks(input)
        .into_iter()
        .sorted_by(|a, b| a.zs.start().cmp(b.zs.start()))
        .collect();

    let mut lowest_available_z: HashMap<IVec2, i32> = HashMap::new();
    let mut final_positions: HashMap<IVec3, usize> = HashMap::new();

    // Let all bricks fall to their final positions

    for brick in bricks.iter() {
        let start_z = *brick.zs.start();

        let final_z: i32 = brick
            .xy_shadow()
            .iter()
            .map(|v| *lowest_available_z.get(v).unwrap_or(&1))
            .max()
            .unwrap();

        let drop_distance = IVec3::new(0, 0, start_z - final_z);

        let block_height = brick.zs.end() - brick.zs.start() + 1;

        brick.xy_shadow().into_iter().for_each(|xy| {
            lowest_available_z.insert(xy, final_z + block_height);
        });

        brick.component_blocks().into_iter().for_each(|block_pos| {
            let final_pos = block_pos - drop_distance;

            final_positions.insert(final_pos, brick.index);
        });
    }

    // Determine the supporting relationships between bricks

    let mut supports: HashMap<usize, HashSet<usize>> = HashMap::new();
    let mut supported_by: HashMap<usize, HashSet<usize>> = HashMap::new();

    final_positions.iter().for_each(|(&block_pos, index)| {
        let above = block_pos + IVec3::Z;

        if let Some(above_index) = final_positions.get(&above) {
            if above_index != index {
                supports
                    .entry(*index)
                    .and_modify(|e| {
                        e.insert(*above_index);
                    })
                    .or_insert(HashSet::from([*above_index]));
            }
        }

        let below = block_pos + IVec3::NEG_Z;

        if let Some(below_index) = final_positions.get(&below) {
            if below_index != index {
                supported_by
                    .entry(*index)
                    .and_modify(|e| {
                        e.insert(*below_index);
                    })
                    .or_insert(HashSet::from([*below_index]));
            }
        }
    });

    // If a brick supports other bricks and at least one of them is supported by just one brick,
    // i.e. this one, then it can't be removed without any others falling.

    let count_sole_support_bricks: usize = supports
        .values()
        .filter(|supported_bricks| {
            supported_bricks
                .iter()
                .any(|other_index| supported_by.get(other_index).unwrap().len() == 1)
        })
        .count();

    let count_safe_bricks = bricks.len() - count_sole_support_bricks;

    println!(
        "Part 1. Bricks that can be removed = {:?}",
        count_safe_bricks
    );

    let collateral_sum: usize = supports
        .keys()
        .map(|brick_index| count_bricks_toppled(brick_index, &supports, &supported_by))
        .sum();

    println!("Part 2. Total bricks falling = {}", collateral_sum);
}

fn count_bricks_toppled(
    index: &usize,
    supports: &HashMap<usize, HashSet<usize>>,
    supported_by: &HashMap<usize, HashSet<usize>>,
) -> usize {
    let mut removed = HashSet::from([index]);
    let mut queue = VecDeque::from([index]);

    while let Some(current_id) = queue.pop_front() {

        if let Some(bricks_supported_by_current) = supports.get(current_id) {
            for supported_block_id in bricks_supported_by_current {
                if removed.contains(supported_block_id) {
                    continue;
                }

                let supports = supported_by.get(supported_block_id).unwrap();

                if supports.iter().all(|on| removed.contains(on)) {
                    removed.insert(supported_block_id);
                    queue.push_back(supported_block_id);
                }
            }
        }
    }

    // Only count the bricks that fall because of the first brick removed
    removed.len() - 1
}

#[derive(Debug)]
struct Brick {
    index: usize,
    xs: RangeInclusive<i32>,
    ys: RangeInclusive<i32>,
    zs: RangeInclusive<i32>,
}

impl Brick {
    fn xy_shadow(&self) -> Vec<IVec2> {
        self.xs
            .clone()
            .flat_map(|x| self.ys.clone().map(move |y| IVec2::new(x, y)))
            .collect()
    }

    fn component_blocks(&self) -> Vec<IVec3> {
        self.xs
            .clone()
            .flat_map(|x| {
                self.ys
                    .clone()
                    .flat_map(move |y| self.zs.clone().map(move |z| IVec3::new(x, y, z)))
            })
            .collect_vec()
    }
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (left, right) = line.split('~').collect_tuple().unwrap();

            let (x1, y1, z1) = left
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap();

            let (x2, y2, z2) = right
                .split(',')
                .map(|v| v.parse().unwrap())
                .collect_tuple()
                .unwrap();

            Brick {
                index: i,
                xs: x1..=x2,
                ys: y1..=y2,
                zs: z1..=z2,
            }
        })
        .collect()
}
