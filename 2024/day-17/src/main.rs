use std::char::from_digit;

fn main() {
    let mut process = Process {
        a: 61657405,
        b: 0,
        c: 0,
        program: vec![2, 4, 1, 2, 7, 5, 4, 3, 0, 3, 1, 7, 5, 5, 3, 0],
    };

    let out = process.run();
    println!("Part 1. Program output: {out}");

    part_two(&process);
}

fn part_two(process: &Process) {
    let reversed_program: Vec<u64> = process.program.iter().cloned().rev().collect();

    fn find_solution(current_a: u64, reversed_program: &[u64]) -> Option<u64> {
        if reversed_program.is_empty() {
            return Some(current_a);
        }

        for a_last in 0..8 {
            let a = (current_a << 3) | a_last;

            let b = a % 8;
            let b = b ^ 2;
            let c = a >> b;
            let b = b ^ c;
            let b = b ^ 7;

            if b % 8 == reversed_program[0] {
                if let Some(solution_a) = find_solution(a, &reversed_program[1..]) {
                    return Some(solution_a);
                }
            }
        }

        None
    }

    if let Some(a) = find_solution(0, &reversed_program) {
        println!("Part 2. Value of a = {a}");
    }
}

struct Process {
    a: u64,
    b: u64,
    c: u64,
    program: Vec<u64>,
}

impl Process {
    fn run(&mut self) -> String {
        let mut ip: i64 = 0;
        let max_i = self.program.len() as i64 - 1;

        let mut output = vec![];

        while ip >= 0 && ip < max_i {
            let op_code = self.program[ip as usize];
            let operand = self.program[(ip + 1) as usize];

            match op_code {
                0 => self.a >>= self.combo_operand(operand),
                1 => self.b ^= operand,
                2 => self.b = self.combo_operand(operand) % 8,
                3 => {
                    if self.a != 0 {
                        ip = operand as i64 - 2;
                    }
                }
                4 => self.b ^= self.c,
                5 => output.push(self.combo_operand(operand) % 8),
                6 => self.b = self.a >> self.combo_operand(operand),
                7 => self.c = self.a >> self.combo_operand(operand),
                _ => panic!("Not an expected opcode!"),
            }

            ip += 2;
        }

        let chars: Vec<String> = output
            .iter()
            .map(|&digit| from_digit(digit as u32, 10).unwrap().to_string())
            .collect();

        chars.join(",")
    }

    fn combo_operand(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Not an expected operand!"),
        }
    }
}
