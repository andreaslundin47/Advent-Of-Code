use glam::DVec3;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();

    let hail_stones: Vec<HailStone> = input.lines().map(parse_line).collect_vec();

    let mut count = 0;

    for v in hail_stones.iter().combinations(2) {
        let (first, second) = (v[0], v[1]);

        let DVec3 {
            x: px1,
            y: py1,
            z: _,
        } = first.start;
        let DVec3 {
            x: vx1,
            y: vy1,
            z: _,
        } = first.velocity;

        let a1 = vy1;
        let b1 = -vx1;
        let c1 = px1 * vy1 - py1 * vx1;

        let DVec3 {
            x: px2,
            y: py2,
            z: _,
        } = second.start;
        let DVec3 {
            x: vx2,
            y: vy2,
            z: _,
        } = second.velocity;

        let a2 = vy2;
        let b2 = -vx2;
        let c2 = px2 * vy2 - py2 * vx2;

        if b2 * a1 - b1 * a2 == 0.0 {
            // Parallel lines!
            continue;
        }

        let x = (c1 * b2 - c2 * b1) / (b2 * a1 - b1 * a2);
        let y = (c2 * a1 - c1 * a2) / (b2 * a1 - b1 * a2);

        if (x - px1) * vx1 < 0.0 || (x - px2) * vx2 < 0.0 {
            // One or both hailstones crossed before time zero
            continue;
        }

        let min_lim = 200_000_000_000_000.0;
        let max_lim = 400_000_000_000_000.0;

        if x < min_lim || x > max_lim || y < min_lim || y > max_lim {
            // Crossing outside region of interest!
            continue;
        }

        // Valid crossing!
        count += 1;
    }

    println!("Part 1. Valid intersections = {count}");
}

#[derive(Debug)]
struct HailStone {
    start: DVec3,
    velocity: DVec3,
}

fn parse_line(input: &str) -> HailStone {
    let (start, velocity) = input.split(" @ ").collect_tuple().unwrap();
    let start = parse(start);
    let velocity = parse(velocity);

    HailStone { start, velocity }
}

fn parse(input: &str) -> DVec3 {
    let (x, y, z) = input.split(", ").collect_tuple().unwrap();

    DVec3::new(
        x.trim().parse::<f64>().unwrap(),
        y.trim().parse::<f64>().unwrap(),
        z.trim().parse::<f64>().unwrap(),
    )
}
