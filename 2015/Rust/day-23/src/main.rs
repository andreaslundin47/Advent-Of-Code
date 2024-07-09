fn main() {
    let input = include_str!("../input.txt").trim();
    let instructions = parse(input);

    let regs = execute(&instructions, [0, 0]);
    println!("Part 1. b = {}", regs[1]);

    let regs = execute(&instructions, [1, 0]);
    println!("Part 2. b = {}", regs[1]);
}

fn execute(program: &Vec<Instr>, start_state: [i128; 2]) -> [i128; 2] {
    let mut regs: [i128; 2] = start_state;
    let mut ip: i8 = 0;

    while ip >= 0 && ip < program.len() as i8 {
        let current = &program[ip as usize];

        match current {
            Instr::Hlf(c) if c == &'a' => regs[0] /= 2,
            Instr::Hlf(c) if c == &'b' => regs[1] /= 2,
            Instr::Tpl(c) if c == &'a' => regs[0] = regs[0].checked_mul(3).unwrap(),
            Instr::Tpl(c) if c == &'b' => regs[1] = regs[1].checked_mul(3).unwrap(),
            Instr::Inc(c) if c == &'a' => regs[0] += 1,
            Instr::Inc(c) if c == &'b' => regs[1] += 1,
            Instr::Jmp(offset) => ip = ip + offset - 1,
            Instr::Jie(c, offset) if c == &'a' => {
                if regs[0] % 2 == 0 {
                    ip += offset - 1
                }
            }
            Instr::Jie(c, offset) if c == &'b' => {
                if regs[1] % 2 == 0 {
                    ip += offset - 1
                }
            }
            Instr::Jio(c, offset) if c == &'a' => {
                if regs[0] == 1 {
                    ip += offset - 1
                }
            }
            Instr::Jio(c, offset) if c == &'b' => {
                if regs[1] == 1 {
                    ip += offset - 1
                }
            }
            _ => panic!("Found bad instruction while running!"),
        }

        ip += 1;
    }

    regs
}

#[derive(Debug)]
enum Instr {
    Hlf(char),
    Tpl(char),
    Inc(char),
    Jmp(i8),
    Jie(char, i8),
    Jio(char, i8),
}

fn parse(input: &str) -> Vec<Instr> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            match parts[0] {
                "hlf" => {
                    let reg = parts[1].chars().next().unwrap();
                    Instr::Hlf(reg)
                }
                "tpl" => {
                    let reg = parts[1].chars().next().unwrap();
                    Instr::Tpl(reg)
                }
                "inc" => {
                    let reg = parts[1].chars().next().unwrap();
                    Instr::Inc(reg)
                }
                "jmp" => {
                    let offset = parts[1].parse::<i8>().unwrap();
                    Instr::Jmp(offset)
                }
                "jie" => {
                    let reg = parts[1].chars().next().unwrap();
                    let offset = parts[2].parse::<i8>().unwrap();
                    Instr::Jie(reg, offset)
                }
                "jio" => {
                    let reg = parts[1].chars().next().unwrap();
                    let offset = parts[2].parse::<i8>().unwrap();
                    Instr::Jio(reg, offset)
                }
                _ => panic!("Bad input?!"),
            }
        })
        .collect()
}
