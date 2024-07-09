use glam::IVec2;
use std::collections::HashSet;

const LEFT: IVec2 = IVec2::NEG_Y;
const RIGHT: IVec2 = IVec2::Y;

fn main() {
    let input = include_str!("../input.txt").trim();

    let mut position = IVec2::ZERO;
    let mut direction = IVec2::NEG_Y;

    let mut tracker = Tracker::default();

    for instruction in input.split(", ") {
        let (turn_direction, steps) = parse(instruction);

        direction = turn_direction.rotate(direction);

        for _ in 0..steps {
            position += direction;
            tracker.add(position);
        }
    }

    let final_distance = position.x.abs() + position.y.abs();
    println!("Part 1. Distance = {}", final_distance);

    if let Some(position) = tracker.first_revisit {
        let distance = position.x.abs() + position.y.abs();
        println!("Part 2. Distance = {}", distance);
    }
}

#[derive(Default)]
struct Tracker {
    visited: HashSet<IVec2>,
    first_revisit: Option<IVec2>,
}

impl Tracker {
    fn add(&mut self, pos: IVec2) {
        if let Some(repeat) = self.visited.get(&pos) {
            if self.first_revisit.is_none() {
                self.first_revisit = Some(*repeat);
            }
        }
        self.visited.insert(pos);
    }
}

fn parse(instruction: &str) -> (IVec2, i32) {
    let (letter, number) = instruction.split_at(1);
    let direction = match letter {
        "L" => LEFT,
        "R" => RIGHT,
        _ => panic!("Unexpected input!"),
    };
    let steps = number.parse::<i32>().expect("A positive integer");

    (direction, steps)
}
