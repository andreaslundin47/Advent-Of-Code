use glam::I64Vec2;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let red_tiles: Vec<I64Vec2> = input
        .lines()
        .filter_map(|line| {
            line.split_once(',').and_then(|(x, y)| {
                let x = x.parse::<i64>().ok()?;
                let y = y.parse::<i64>().ok()?;
                Some(I64Vec2::new(x, y))
            })
        })
        .collect();

    let max_area: u64 = red_tiles
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1))
        .max()
        .expect("Should have area for at least one pair");

    println!("Part 1. {max_area}");

    // The AoC input is nice, and allows for a simplified solution that is not
    // guaranteed to work on a generalized problem

    let edge_hit_boxes: Vec<HitBox> = red_tiles
        .iter()
        .cycle()
        .take(red_tiles.len() + 1)
        .tuple_windows()
        .map(|(a, b)| HitBox::new(a, b))
        .collect();

    let mut max_area = 0;

    for (a, b) in red_tiles.iter().tuple_combinations() {
        let candidate_area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);

        if candidate_area <= max_area {
            continue;
        }

        let candidate_box = HitBox::new(a, b);

        if edge_hit_boxes
            .iter()
            .any(|hit_box| hit_box.overlap(&candidate_box))
        {
            continue;
        }

        max_area = candidate_area;
    }

    println!("Part 2. {max_area}");
}

struct HitBox {
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

impl HitBox {
    fn new(a: &I64Vec2, b: &I64Vec2) -> Self {
        let min_x = a.x.min(b.x);
        let max_x = a.x.max(b.x);
        let min_y = a.y.min(b.y);
        let max_y = a.y.max(b.y);

        Self {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    fn overlap(&self, other: &Self) -> bool {
        !(other.min_x >= self.max_x
            || other.max_x <= self.min_x
            || other.min_y >= self.max_y
            || other.max_y <= self.min_y)
    }
}
