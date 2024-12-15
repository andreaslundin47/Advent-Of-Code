use std::collections::{HashSet, VecDeque};

use glam::IVec2;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let (raw_grid, raw_moves) = input.split_once("\n\n").unwrap();

    let moves: Vec<char> = raw_moves.chars().filter(|&c| c != '\n').collect();

    let world_one = parse_grid(&raw_grid, false);
    part_one(&world_one, &moves);

    let world_two = parse_grid(&raw_grid, true);
    part_two(&world_two, &moves);
}

fn part_one(world: &World, moves: &[char]) {
    let mut robot_position = world.robot;
    let walls = world.walls.clone();
    let mut boxes = world.boxes.clone();

    for direction in moves.iter() {
        let delta = match direction {
            '^' => IVec2::NEG_Y,
            'v' => IVec2::Y,
            '<' => IVec2::NEG_X,
            '>' => IVec2::X,
            _ => panic!(),
        };

        let robot_destination = robot_position + delta;

        if walls.contains(&robot_destination) {
            continue;
        }

        if !boxes.contains(&robot_destination) {
            robot_position = robot_destination;
            continue;
        }

        for i in 2.. {
            let look = robot_position + delta * i;

            if walls.contains(&look) {
                break;
            }

            if !boxes.contains(&look) {
                boxes.remove(&robot_destination);
                boxes.insert(look);
                robot_position = robot_destination;
                break;
            }
        }
    }

    let gps = calc_gps_sum(&boxes);
    println!("Part 1. GPS sum: {}", gps);
}

fn part_two(world: &World, moves: &[char]) {
    let mut robot_position = world.robot;
    let walls = world.walls.clone();
    let mut boxes = world.boxes.clone();

    for direction in moves.iter() {
        let delta = match direction {
            '^' => IVec2::NEG_Y,
            'v' => IVec2::Y,
            '<' => IVec2::NEG_X,
            '>' => IVec2::X,
            _ => panic!(),
        };

        let robot_destination = robot_position + delta;

        if walls.contains(&robot_destination) {
            continue;
        }

        match direction {
            '<' | '>' => move_left_right(&mut robot_position, delta, &mut boxes, &walls),
            '^' | 'v' => move_up_down(&mut robot_position, delta, &mut boxes, &walls),
            _ => panic!("Weird direction..."),
        }
    }

    let gps = calc_gps_sum(&boxes);
    println!("Part 2. GPS sum: {}", gps);
}

fn move_left_right(
    robot_position: &mut IVec2,
    delta: IVec2,
    boxes: &mut HashSet<IVec2>,
    walls: &HashSet<IVec2>,
) {
    let shift = 2 * delta;

    let first_check = match delta {
        IVec2::NEG_X => *robot_position + shift,
        IVec2::X => *robot_position + delta,
        _ => panic!("Weird delta..."),
    };

    let mut check_if_free = VecDeque::from([first_check]);
    let mut boxes_to_move = HashSet::new();

    while let Some(look) = check_if_free.pop_front() {
        if walls.contains(&look) {
            if delta == IVec2::NEG_X && !walls.contains(&(look + IVec2::X)) {
                // The left-most box does leave a 1-gap before the left wall. Can move!
                break;
            }
            return;
        }

        if boxes.contains(&look) {
            boxes_to_move.insert(look);
            check_if_free.push_back(look + shift);
        }
    }

    for b in boxes_to_move {
        boxes.remove(&b);
        boxes.insert(b + delta);
    }

    *robot_position = *robot_position + delta;
}

fn move_up_down(
    robot_position: &mut IVec2,
    delta: IVec2,
    boxes: &mut HashSet<IVec2>,
    walls: &HashSet<IVec2>,
) {
    let robot_destination = *robot_position + delta;

    let mut check_if_free = VecDeque::from([robot_destination]);
    let mut boxes_to_move = HashSet::new();

    while let Some(look) = check_if_free.pop_front() {
        if walls.contains(&look) {
            return;
        }
        if boxes.contains(&look) {
            boxes_to_move.insert(look);
            check_if_free.push_back(look + delta);
            check_if_free.push_back(look + delta + IVec2::X);
        } else if boxes.contains(&(look + IVec2::NEG_X)) {
            boxes_to_move.insert(look + IVec2::NEG_X);
            check_if_free.push_back(look + IVec2::NEG_X + delta);
            check_if_free.push_back(look + delta);
        }
    }

    if !boxes_to_move.is_empty() {
        // Box height is 1, so need to move them in the correct order to not lose boxes on the way
        match delta {
            IVec2::NEG_Y => {
                for b in boxes_to_move.into_iter().sorted_by(|a, b| a.y.cmp(&b.y)) {
                    boxes.remove(&b);
                    boxes.insert(b + delta);
                }
            }
            IVec2::Y => {
                for b in boxes_to_move.into_iter().sorted_by(|a, b| b.y.cmp(&a.y)) {
                    boxes.remove(&b);
                    boxes.insert(b + delta);
                }
            }
            _ => panic!("Weird delta..."),
        };
    }

    *robot_position = *robot_position + delta;
}

fn calc_gps_sum(boxes: &HashSet<IVec2>) -> usize {
    boxes
        .iter()
        .map(|b| {
            let gps = 100 * b.y + b.x;
            gps as usize
        })
        .sum()
}

fn print_world(robot: IVec2, walls: &HashSet<IVec2>, boxes: &HashSet<IVec2>) {
    let x_max = walls.iter().map(|w| w.x).max().unwrap();
    let y_max = walls.iter().map(|w| w.y).max().unwrap();

    println!();
    for y in 0..=y_max {
        let mut x = 0;
        while x <= x_max {
            let pos = IVec2::new(x, y);

            if pos == robot {
                print!("@");
            } else if walls.contains(&pos) {
                print!("#");
            } else if boxes.contains(&pos) {
                print!("[]");
                x += 1;
            } else {
                print!(".");
            }
            x += 1;
        }
        println!();
    }
}

struct World {
    robot: IVec2,
    walls: HashSet<IVec2>,
    boxes: HashSet<IVec2>,
}

fn parse_grid(input: &str, make_wider: bool) -> World {
    let mut robot = IVec2::default();
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let x = x as i32;
            let y = y as i32;

            if c == '@' {
                if make_wider {
                    robot = IVec2::new(2 * x, y);
                } else {
                    robot = IVec2::new(x, y);
                }
            }

            if c == '#' {
                if make_wider {
                    walls.insert(IVec2::new(2 * x, y));
                    walls.insert(IVec2::new(2 * x + 1, y));
                } else {
                    walls.insert(IVec2::new(x, y));
                }
            }

            if c == 'O' {
                if make_wider {
                    boxes.insert(IVec2::new(2 * x, y));
                } else {
                    boxes.insert(IVec2::new(x, y));
                }
            }
        }
    }

    World {
        robot,
        walls,
        boxes,
    }
}
