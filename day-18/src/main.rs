use std::collections::HashMap;

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

    let sounds_emitted = program.run_until_wait(vec![]);
    let last_sound = sounds_emitted.last().unwrap();

    println!("Part 1. Last played sound = {last_sound}");
}

fn part_two() {
    let input = include_str!("../input.txt").trim();
    let program: Vec<DuetInstruction> = parse(input);

    let mut program_zero = ProgramRunner::new(0, &program);
    let mut program_one = ProgramRunner::new(1, &program);

    let mut queue = vec![];
    let mut count_from_program_one = 0;

    loop {
        queue = program_zero.run_until_wait(queue);
        let program_0_sent_nothing = queue.len() == 0;

        queue = program_one.run_until_wait(queue);
        let program_1_sent_nothing = queue.len() == 0;

        count_from_program_one += queue.len();

        if program_0_sent_nothing && program_1_sent_nothing {
            break;
        }
    }

    println!("Part 2. Program 1 sends {count_from_program_one} values");
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

    fn run_until_wait(&mut self, inputs: Vec<i64>) -> Vec<i64> {
        let mut rcv_queue = inputs;
        let mut snd_queue = vec![];

        loop {
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
                Snd => {
                    snd_queue.push(arg_1);
                }
                Rcv => match acc {
                    RegVal::Register(register) => {
                        if rcv_queue.is_empty() {
                            break;
                        } else {
                            self.registers.entry(register).and_modify(|e| {
                                *e = rcv_queue.remove(0);
                            });
                        }
                    }
                    RegVal::Value(value) => {
                        if value > 0 {
                            break;
                        }
                    }
                },
                Set => {
                    if let RegVal::Register(register) = acc {
                        self.registers
                            .entry(register)
                            .and_modify(|e| *e = arg_2.expect("A valid integer"));
                    }
                }
                Add => {
                    if let RegVal::Register(register) = acc {
                        self.registers
                            .entry(register)
                            .and_modify(|e| *e = arg_1 + arg_2.expect("A valid integer"));
                    }
                }
                Mul => {
                    if let RegVal::Register(register) = acc {
                        self.registers
                            .entry(register)
                            .and_modify(|e| *e = arg_1 * arg_2.expect("A valid integer"));
                    }
                }
                Mod => {
                    if let RegVal::Register(register) = acc {
                        self.registers
                            .entry(register)
                            .and_modify(|e| *e = arg_1 % arg_2.expect("A valid integer"));
                    }
                }
                Jgz => {
                    if arg_1 > 0 {
                        let offset = arg_2.expect("A valid integer");
                        self.ip = (self.ip as i64 + offset - 1) as usize;
                    }
                }
            }

            self.ip += 1;
        }

        snd_queue
    }
}

#[derive(Debug, Copy, Clone)]
enum DuetAction {
    Snd,
    Rcv,
    Set,
    Add,
    Mul,
    Mod,
    Jgz,
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
        "snd" => DuetAction::Snd,
        "rcv" => DuetAction::Rcv,
        "set" => DuetAction::Set,
        "add" => DuetAction::Add,
        "mul" => DuetAction::Mul,
        "mod" => DuetAction::Mod,
        "jgz" => DuetAction::Jgz,
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
