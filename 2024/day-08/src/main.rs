use glam::IVec2;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let world = parse(input);

    part_one(&world);
    part_two(&world);
}

fn part_one(world: &World) {
    let antennas = &world.antennas;

    let antionde_count_1: Vec<IVec2> = antennas
        .iter()
        .cartesian_product(antennas)
        .filter(|(a1, a2)| a1 != a2 && a1.frequency == a2.frequency)
        .flat_map(|(a1, a2)| {
            let delta = a2.position - a1.position;

            let antinode_one = a2.position + delta;
            let antinode_two = a1.position - delta;

            [
                world.contains(&antinode_one).then_some(antinode_one),
                world.contains(&antinode_two).then_some(antinode_two),
            ]
            .into_iter()
            .flatten()
        })
        .unique()
        .collect();

    let antionde_count_2 = antionde_count_1.len();

    println!("Part 1: Antinodes: {}", antionde_count_2);
}

fn part_two(world: &World) {
    let antennas = &world.antennas;

    let antinodes_count = antennas
        .iter()
        .cartesian_product(antennas)
        .filter(|(a1, a2)| a1 != a2 && a1.frequency == a2.frequency)
        .flat_map(|(a1, a2)| {
            let delta = a2.position - a1.position;

            let antinodes_one = (0..)
                .map(move |i| a2.position + delta * i)
                .take_while(|pos| world.contains(pos));

            let antinodes_two = (0..)
                .map(move |i| a1.position - delta * i)
                .take_while(|pos| world.contains(pos));

            antinodes_one.chain(antinodes_two)
        })
        .unique()
        .count();

    println!("Part 2: Antinodes: {}", antinodes_count);
}

#[derive(PartialEq)]
struct Antenna {
    position: IVec2,
    frequency: char,
}

struct World {
    width: i32,
    height: i32,
    antennas: Vec<Antenna>,
}

impl World {
    fn contains(&self, pos: &IVec2) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }
}

fn parse(input: &str) -> World {
    let height = input.lines().count() as i32;
    let width = input.lines().next().unwrap().len() as i32;

    let antennas = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().filter_map(move |(x, c)| {
                (c != '.').then_some(Antenna {
                    position: IVec2::new(x as i32, y as i32),
                    frequency: c,
                })
            })
        })
        .collect();

    World {
        width,
        height,
        antennas,
    }
}
