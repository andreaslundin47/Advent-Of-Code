use itertools::Itertools;

#[derive(Eq, PartialEq)]
struct Galaxy {
    x: i64,
    y: i64,
}

impl Galaxy {
    fn from(x: i64, y: i64) -> Galaxy {
        Galaxy { x, y }
    }

    fn distance(&self, other: &Galaxy) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
    let raw = include_str!("../input.txt").trim();
    solve_part_one(&raw);
    solve_part_two(&raw);
}

fn solve_part_one(input: &str) {
    let mut galaxies = parse_galaxies(&input);
    separate_galaxies(&mut galaxies, 2);
    let dists = sum_distances(&galaxies);

    println!("Part 1. Sum of distances = {dists}");
}

fn solve_part_two(input: &str) {
    let mut galaxies = parse_galaxies(&input);
    separate_galaxies(&mut galaxies, 1_000_000);
    let dists = sum_distances(&galaxies);

    println!("Part 2. Sum of distances = {dists}");
}

fn separate_galaxies(galaxies: &mut Vec<Galaxy>, gap_multiplier: i64) {
    // Expand in x - direction

    let min_x = galaxies.iter().map(|g| g.x).min().unwrap();
    let max_x = galaxies.iter().map(|g| g.x).max().unwrap();

    let xs: Vec<i64> = galaxies.iter().map(|g| g.x).collect();

    let void_xs: Vec<i64> = (min_x..=max_x).filter(|x| !xs.contains(x)).collect();

    for galaxy in galaxies.iter_mut() {
        let gaps = void_xs.iter().filter(|x| galaxy.x > **x).count() as i64;
        galaxy.x += gaps * (gap_multiplier - 1);
    }

    // Expand in y - direction

    let min_y = galaxies.iter().map(|g| g.y).min().unwrap();
    let max_y = galaxies.iter().map(|g| g.y).max().unwrap();

    let ys: Vec<i64> = galaxies.iter().map(|g| g.y).collect();

    let void_ys: Vec<i64> = (min_y..=max_y).filter(|y| !ys.contains(y)).collect();

    for galaxy in galaxies.iter_mut() {
        let gaps = void_ys.iter().filter(|y| galaxy.y > **y).count() as i64;
        galaxy.y += gaps * (gap_multiplier - 1);
    }
}

fn sum_distances(galaxies: &Vec<Galaxy>) -> i64 {
    galaxies.iter().combinations(2).enumerate().map(|(_i,pair_vec)| {
        let g1 = pair_vec[0];
        let g2 = pair_vec[1];
        g1.distance(g2)
    })
    .sum()
}

fn print_galaxies(galaxies: &Vec<Galaxy>) {
    let xs = galaxies.iter().map(|g| g.x).sorted();
    let ys = galaxies.iter().map(|g| g.y).sorted();

    let min_x = xs.clone().min().unwrap();
    let max_x = xs.max().unwrap();

    let min_y = ys.clone().min().unwrap();
    let max_y = ys.max().unwrap();

    println!("--------------------------------");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if galaxies.contains(&Galaxy::from(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("--------------------------------");
}

fn parse_galaxies(input: &str) -> Vec<Galaxy> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c != '.' {
                        Some( Galaxy::from(x as i64, y as i64) )
                    } else {
                        None
                    }
                })
            })
        .collect()
}