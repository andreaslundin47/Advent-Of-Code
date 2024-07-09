use glam::I64Vec2;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, digit1, newline},
    multi::separated_list1,
    Finish, IResult,
};
//----------------------------------------------------------------------------------------
fn main() {
    let input = include_str!("../input.txt").trim();
    let instructions = parse(input);

    let steps_1 = instructions.iter().map(|instr| instr.steps).collect_vec();

    let area_1 = area_and_boundary(&steps_1);

    println!("Part 1. Cubics meters = {area_1}");

    let steps_2 = instructions
        .iter()
        .map(|instr| instr.color_steps)
        .collect_vec();

    let area_2 = area_and_boundary(&steps_2);

    println!("Part 1. Cubics meters = {area_2}");
}

fn area_and_boundary(steps: &[I64Vec2]) -> i64 {
    // We want to count interior points (I) and boundary points (B), i.e. I + B
    //
    // From Pick's Theorem: A = I + B/2 - 1  =>  I + B = A + 1 + B / 2
    //
    // We get the area (A) from the Shoelace Theorem

    let coordinates: Vec<I64Vec2> = steps
        .iter()
        .fold(vec![I64Vec2::new(0, 0)], |mut acc, step| {
            let next_position = *acc.last().unwrap() + *step;
            acc.push(next_position);
            acc
        });

    let area: i64 = coordinates
        .iter()
        .tuple_windows()
        .map(|(a, b)| a.x * b.y - a.y * b.x)
        .sum::<i64>()
        .abs()
        / 2;

    let boundary_points: i64 = steps
        .iter()
        .map(|step| {
            step.x.abs() + step.y.abs()
        })
        .sum();

    area + 1 + boundary_points / 2
}
//----------------------------------------------------------------------------------------
#[derive(Debug)]
struct DigInstruction {
    steps: I64Vec2,
    color_steps: I64Vec2,
}

fn parse(i: &str) -> Vec<DigInstruction> {
    let (_, instructions) = separated_list1(newline, instruction)(i)
        .finish()
        .expect("A valid parse");

    instructions
}

fn instruction(i: &str) -> IResult<&str, DigInstruction> {
    let (i, dir) = alpha1(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, steps) = digit1(i)?;
    let (i, _) = tag(" (#")(i)?;
    let (i, color) = alphanumeric1(i)?;
    let (i, _) = tag(")")(i)?;

    // For part 1
    let direction = match dir {
        "U" => I64Vec2::NEG_Y,
        "D" => I64Vec2::Y,
        "L" => I64Vec2::NEG_X,
        "R" => I64Vec2::X,
        _ => panic!("Invalid direction in input!"),
    };

    let step_length = steps.parse::<i64>().expect("A valid integer number");

    let steps = direction * step_length;

    // For part 2
    let dir = color.chars().last().unwrap();

    let color_direction = match dir {
        '0' => I64Vec2::X,
        '1' => I64Vec2::Y,
        '2' => I64Vec2::NEG_X,
        '3' => I64Vec2::NEG_Y,
        _ => panic!("Invalid color direction in input!"),
    };

    let color_length =
        i64::from_str_radix(&color[..5], 16).expect("A valid integer from hex string");

    let color_steps = color_direction * color_length;

    Ok((i, DigInstruction { steps, color_steps }))
}
//----------------------------------------------------------------------------------------
