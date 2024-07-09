const PUZZLE_INPUT: &str = "ffykfhsq";
const PASSWORD_LENGTH: usize = 8;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let state = State::new();
    let mut password = vec![];

    for (_, digest) in (0..PASSWORD_LENGTH).zip(state) {
        password.push( num_to_hex(digest[2]) );
   }

    let pw: String = password.iter().collect();
    println!("Part 1. Password = {}", pw);
}

fn part_two() {
    let mut state = State::new();
    let mut password = [None; PASSWORD_LENGTH];

    while password.contains(&None) {
        while let Some(digest) = state.next() {
            let position = digest[2] as usize;

            if position < 8 && password[position].is_none() {
                let digit = digest[3] >> 4; 
                password[position] = Some(num_to_hex(digit));
                break;
            }
        }
    }

    let pw: String = password.into_iter().filter_map(|c| c).collect();
    println!("Part 2. Password = {}", pw);
}

fn num_to_hex(num: u8) -> char {
    (if num < 10 { b'0' + num } else { b'a' + (num - 10) }) as char
}

struct State {
    index: usize,
}

impl State {
    fn new() -> Self {
        State { index: 0 }
    }
}

impl Iterator for State {
    type Item = md5::Digest;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let input = format!("{}{}", PUZZLE_INPUT, self.index);
            self.index += 1;
            let digest = md5::compute(input);

            if 0 == digest[0] && 0 == digest[1] && 0 == digest[2] & 0xf0 {
                return Some(digest);
            }
        }
    }
}