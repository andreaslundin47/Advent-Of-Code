use std::collections::HashMap;
use primes::is_prime;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    combinator::{map as nom_map, opt, recognize},
    multi::separated_list1,
    sequence::{pair, preceded},
    Finish, IResult,
};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = include_str!("../input.txt").trim();
    let program: Vec<DuetInstruction> = parse(input);

    let mut program = ProgramRunner::new(0, &program);
    let mul_count = program.count_muls();

    println!("Part 1. Mul is used {mul_count} times");
}

fn part_two() {
    // Found by manually inspecting the input.txt file
    let mut h = 0;
    for i in 0..=1000 {
        let b = 108400 + 17 * i;
        if !is_prime(b) {
            h += 1;
        }
    }

    println!("Part 2. Final h is {h}");
}

struct ProgramRunner<'a> {
    program: &'a Vec<DuetInstruction>,
    registers: HashMap<char, i64>,
    ip: usize,
}

impl ProgramRunner<'_> {
    fn new(id: i64, program: &Vec<DuetInstruction>) -> ProgramRunner {
        let registers = HashMap::from([('p', id)]);
        ProgramRunner {
            program,
            registers,
            ip: 0,
        }
    }

    fn count_muls(&mut self) -> usize {

        let mut mul_count = 0;

        while self.ip < self.program.len() {

            let DuetInstruction { op, acc, value } = self.program[self.ip];

            let arg_1: i64 = match acc {
                RegVal::Register(reg) => *self.registers.entry(reg).or_insert(0),
                RegVal::Value(v) => v,
            };

            let arg_2: Option<i64> = value.map(|v| match v {
                RegVal::Register(reg) => *self.registers.entry(reg).or_insert(0),
                RegVal::Value(v) => v,
            });

            use DuetAction::*;

            match op {
               Set => {
                    if let RegVal::Register(register) = acc {
                        self.registers
                            .entry(register)
                            .and_modify(|e| *e = arg_2.expect("A valid integer"));
                    }
                }
                Sub => {
                    if let RegVal::Register(register) = acc {
                        self.registers
                            .entry(register)
                            .and_modify(|e| *e = arg_1 - arg_2.expect("A valid integer"));
                    }
                }
                Mul => {
                    mul_count += 1;
                    if let RegVal::Register(register) = acc {
                        self.registers
                            .entry(register)
                            .and_modify(|e| *e = arg_1 * arg_2.expect("A valid integer"));
                    }
                }
                Jnz => {
                    if arg_1 != 0 {
                        let offset = arg_2.expect("A valid integer");
                        self.ip = (self.ip as i64 + offset - 1) as usize;
                    }
                }
            }

            self.ip += 1;
        }

        mul_count
    }
}

#[derive(Debug, Copy, Clone)]
enum DuetAction {
    Set,
    Sub,
    Mul,
    Jnz,
}

#[derive(Debug, Copy, Clone)]
enum RegVal {
    Register(char),
    Value(i64),
}

#[derive(Debug, Copy, Clone)]
struct DuetInstruction {
    op: DuetAction,
    acc: RegVal,
    value: Option<RegVal>,
}

fn parse_register(i: &str) -> IResult<&str, RegVal> {
    nom_map(alpha1, |c: &str| {
        RegVal::Register(c.chars().next().unwrap())
    })(i)
}

fn parse_imediate(i: &str) -> IResult<&str, RegVal> {
    nom_map(recognize(pair(opt(tag("-")), digit1)), |v: &str| {
        RegVal::Value(v.parse::<i64>().unwrap())
    })(i)
}

fn parse_reg_val(i: &str) -> IResult<&str, RegVal> {
    alt((parse_register, parse_imediate))(i)
}

fn instruction(i: &str) -> IResult<&str, DuetInstruction> {
    let (i, op) = alpha1(i)?;
    let (i, _) = tag(" ")(i)?;
    let (i, acc) = parse_reg_val(i)?;

    let (i, value) = opt(preceded(tag(" "), parse_reg_val))(i)?;

    let op = match op {
        "set" => DuetAction::Set,
        "sub" => DuetAction::Sub,
        "mul" => DuetAction::Mul,
        "jnz" => DuetAction::Jnz,
        _ => panic!("Found invalid instruction type!"),
    };

    Ok((i, DuetInstruction { op, acc, value }))
}

fn parse(i: &str) -> Vec<DuetInstruction> {
    let (_, instructions) = separated_list1(newline, instruction)(i)
        .finish()
        .expect("Valid parse");
    instructions
}
