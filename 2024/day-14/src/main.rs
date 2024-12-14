use std::{collections::HashSet, i32};

use glam::IVec2;

mod parsing;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn main() {
    let input = include_str!("../input.txt").trim();
    let robots: Vec<Robot> = parsing::parse_robots(input).expect("A valid parse").1;

    part_one(robots.clone());
    part_two(robots.clone());
}

fn part_one(mut robots: Vec<Robot>) {
    for _step in 0..100 {
        for robot in robots.iter_mut() {
            robot.step();
        }
    }

    println!("Part 1. Safety Factor: {}", safety_factor(&robots));
}

fn part_two(mut robots: Vec<Robot>) {
    let mut score = f64::MAX;
    let mut min_step = 0;

    for step in 0..100_000 {
        for robot in robots.iter_mut() {
            robot.step();
        }

        let new_score = compactness_score(&robots);

        if new_score < score {
            min_step = step;
            //print_robots(&robots, step);
        }

        score = score.min(new_score);
    }

    println!("Part 2. Time step: {}", min_step);
}

fn safety_factor(robots: &[Robot]) -> usize {
    let ul = robots
        .iter()
        .filter(|r| r.pos.x < WIDTH / 2 && r.pos.y < HEIGHT / 2)
        .count();

    let ur = robots
        .iter()
        .filter(|r| r.pos.x > WIDTH / 2 && r.pos.y < HEIGHT / 2)
        .count();

    let ll = robots
        .iter()
        .filter(|r| r.pos.x < WIDTH / 2 && r.pos.y > HEIGHT / 2)
        .count();

    let lr = robots
        .iter()
        .filter(|r| r.pos.x > WIDTH / 2 && r.pos.y > HEIGHT / 2)
        .count();

    ul * ur * ll * lr
}

fn compactness_score(robots: &[Robot]) -> f64 {
    // Multiply variance of x with variance of y

    let mean_x: f64 = robots.iter().map(|r| r.pos.x as f64).sum::<f64>() / robots.len() as f64;
    let var_x: f64 = robots
        .iter()
        .map(|r| (r.pos.x as f64 - mean_x).powf(2.0))
        .sum::<f64>()
        / robots.len() as f64;

    let mean_y: f64 = robots.iter().map(|r| r.pos.y as f64).sum::<f64>() / robots.len() as f64;
    let var_y: f64 = robots
        .iter()
        .map(|r| (r.pos.y as f64 - mean_y).powf(2.0))
        .sum::<f64>()
        / robots.len() as f64;

    var_x * var_y
}

fn print_robots(robots: &[Robot], step: usize) {
    let positions = robots
        .iter()
        .map(|r| (r.pos.x, r.pos.y))
        .collect::<HashSet<(i32, i32)>>();

    println!("Time step: {step}");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if positions.contains(&(x, y)) {
                print!("*");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

#[derive(Debug, Clone)]
struct Robot {
    id: usize,
    pos: IVec2,
    vel: IVec2,
}

impl Robot {
    fn step(&mut self) {
        self.pos.x = (self.pos.x + self.vel.x).rem_euclid(WIDTH);
        self.pos.y = (self.pos.y + self.vel.y).rem_euclid(HEIGHT);
    }
}
