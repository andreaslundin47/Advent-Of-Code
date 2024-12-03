use regex::Regex;

fn main() {
    let input = include_str!("../input.txt").trim();
    let instructions = parse_instructions(input);

    part_one(&instructions);
    part_two(&instructions);
}

enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

fn part_one(instructions: &[Instruction]) {
    let sum_one: i32 = instructions
        .iter()
        .map(|instr| match instr {
            Instruction::Mul(a, b) => a * b,
            _ => 0,
        })
        .sum();

    println!("Part 1. Sum: {}", sum_one);
}

fn part_two(instructions: &[Instruction]) {
    let mut sum_two = 0;
    let mut include = true;

    for instr in instructions.iter() {
        match instr {
            Instruction::Do => include = true,
            Instruction::Dont => include = false,
            Instruction::Mul(a, b) if include => sum_two += a * b,
            _ => (),
        }
    }

    println!("Part 2. Sum: {}", sum_two);
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let re_instr = Regex::new(r"mul\([0-9]{1, 3},[0-9]{1,3}\)|do\(\)|don't\(\)").unwrap();
    let re_mul = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    re_instr
        .find_iter(input)
        .map(|m| {
            let str_instr = m.as_str();

            match str_instr {
                "do()" => Instruction::Do,
                "don't()" => Instruction::Dont,
                _ => re_mul
                    .captures_iter(str_instr)
                    .next()
                    .map(|caps| {
                        let (_, [left, right]) = caps.extract();
                        Instruction::Mul(
                            left.parse::<i32>().unwrap(),
                            right.parse::<i32>().unwrap(),
                        )
                    })
                    .unwrap(),
            }
        })
        .collect()
}
