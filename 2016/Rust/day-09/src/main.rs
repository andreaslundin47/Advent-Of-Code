fn main() {
    let input = include_str!("../input.txt").trim();

    println!("Part 1. Length = {}", input.decompress_length());
    println!("Part 2. Length = {}", input.explode_length());
}

trait Decompress {
    fn decompress_length(&self) -> usize;
    fn explode_length(&self) -> usize;
}

impl Decompress for &str {
    fn decompress_length(&self) -> usize {
        let mut remaining = *self;

        let mut count = 0;

        while !remaining.is_empty() {
            if remaining.starts_with('(') {
                let (marker_code, rem) = remaining
                    .strip_prefix('(')
                    .unwrap()
                    .split_once(')')
                    .unwrap();

                let (pattern_len, repeats) = marker_code.split_once('x').unwrap();
                let length: usize = pattern_len.parse().unwrap();
                let repeats: usize = repeats.parse().unwrap();

                count += repeats * length;
                remaining = &rem[length..];
            } else {
                count += 1;
                remaining = &remaining[1..];
            }
        }

        count
    }

    fn explode_length(&self) -> usize {
        let mut remaining = *self;

        let mut count = 0;

        while !remaining.is_empty() {
            if remaining.starts_with('(') {
                let (marker_code, rem) = remaining
                    .strip_prefix('(')
                    .unwrap()
                    .split_once(')')
                    .unwrap();

                let (pattern_length, repeats) = marker_code.split_once('x').unwrap();
                let pattern_length: usize = pattern_length.parse().unwrap();
                let repeats: usize = repeats.parse().unwrap();

                let pattern = &rem[0..pattern_length];
                count += repeats * pattern.explode_length();
                remaining = &rem[pattern_length..];
            } else {
                count += 1;
                remaining = &remaining[1..];
            }
        }

        count
    }
}
