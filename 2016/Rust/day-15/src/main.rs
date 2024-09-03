fn main() {
    let input = include_str!("../input.txt").trim();
    let mut discs = parse(input);

    let start_time = crt_sieve(&discs);
    println!("Part 1. Start time = {}", start_time);

    let new_disc = Disc {
        positions: 11,
        at_zero: 0,
    };
    discs.push(new_disc);

    let start_time = crt_sieve(&discs);
    println!("Part 2. Start time = {}", start_time);
}

fn parse(input: &str) -> Vec<Disc> {
    input
        .lines()
        .map(|line| {
            let line = line.replace(".", "");
            let parts: Vec<&str> = line.split(' ').collect();
            let positions = parts[3].parse().unwrap();
            let at_zero = parts[11].parse().unwrap();
            Disc { positions, at_zero }
        })
        .collect()
}

struct Disc {
    positions: usize,
    at_zero: usize,
}

fn crt_sieve(discs: &[Disc]) -> usize {
    let mut current = 0;
    let mut step_length = 1;

    for (index, disc) in discs.iter().enumerate() {
        let time_offset = index + 1;
        let offset = disc.at_zero + time_offset;

        while (current + offset) % disc.positions != 0 {
            current += step_length;
        }

        step_length *= disc.positions;
    }

    current
}
