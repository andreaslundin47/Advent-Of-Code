use std::collections::HashMap;

fn main() {
    let raw_input = include_str!("../input.txt").trim();
    let program = parse(raw_input);

    let mut process_one = Process::new(program.clone());
    process_one.regs.insert('a', 7);
    process_one.run();
    println!(
        "Part 1. Register a = {}",
        process_one.regs.get(&'a').unwrap()
    );

    let mut process_two = Process::new(program.clone());
    process_two.regs.insert('a', 12);
    process_two.run();
    println!(
        "Part 2. Register a = {}",
        process_two.regs.get(&'a').unwrap()
    );
}

struct Process {
    ip: i32,
    regs: HashMap<char, i32>,
    program: Vec<Instruction>,
}

impl Process {
    fn new(program: Vec<Instruction>) -> Process {
        Process {
            ip: 0,
            regs: HashMap::new(),
            program,
        }
    }

    fn get(&self, value: &Value) -> i32 {
        match value {
            Value::Literal(literal) => *literal,
            Value::Register(register) => *self.regs.get(register).unwrap_or(&0),
        }
    }

    fn set(&mut self, register: char, value: i32) {
        self.regs.insert(register, value);
    }

    fn run(&mut self) {
        while let Some(instr) = self.program.get(self.ip as usize).cloned() {
            match instr {
                Instruction::Cpy(val, dest) => {
                    if let Value::Register(r) = dest {
                        self.set(r, self.get(&val));
                    }
                }
                Instruction::Inc(reg) => {
                    if let Value::Register(r) = reg {
                        self.set(r, self.get(&reg) + 1);
                    }
                }
                Instruction::Dec(reg) => {
                    if let Value::Register(r) = reg {
                        self.set(r, self.get(&reg) - 1);
                    }
                }
                Instruction::Jnz(arg, offset) => {
                    if 0 != self.get(&arg) {
                        self.ip += self.get(&offset) - 1;
                    }
                }
                Instruction::Tgl(offset) => {
                    self.toggle(&offset);
                }
            }

            self.ip += 1;
        }
    }

    fn toggle(&mut self, offset: &Value) {
        let offset = self.get(offset);
        let toggle_index = (self.ip + offset) as usize;

        if let Some(old_instruction) = self.program.get(toggle_index).cloned() {
            let new_instruction = match old_instruction {
                Instruction::Cpy(val, dest) => Instruction::Jnz(val, dest),
                Instruction::Inc(reg) => Instruction::Dec(reg),
                Instruction::Dec(reg) => Instruction::Inc(reg),
                Instruction::Jnz(arg, offset) => Instruction::Cpy(arg, offset),
                Instruction::Tgl(offset) => Instruction::Inc(offset),
            };

            if let Some(elem) = self.program.get_mut(toggle_index) {
                *elem = new_instruction;
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Value {
    Literal(i32),
    Register(char),
}

impl Value {
    fn new(input: &str) -> Self {
        match input.parse::<i32>() {
            Ok(lit) => Value::Literal(lit),
            Err(_) => Value::Register(input.chars().nth(0).unwrap()),
        }
    }
}

#[derive(Copy, Clone)]
enum Instruction {
    Cpy(Value, Value),
    Inc(Value),
    Dec(Value),
    Jnz(Value, Value),
    Tgl(Value),
}

fn parse(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(' ').collect();
            match parts[0] {
                "cpy" => Instruction::Cpy(Value::new(parts[1]), Value::new(parts[2])),
                "inc" => Instruction::Inc(Value::new(parts[1])),
                "dec" => Instruction::Dec(Value::new(parts[1])),
                "jnz" => Instruction::Jnz(Value::new(parts[1]), Value::new(parts[2])),
                "tgl" => Instruction::Tgl(Value::new(parts[1])),
                _ => panic!("Bad input!"),
            }
        })
        .collect()
}
