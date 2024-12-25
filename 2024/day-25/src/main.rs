use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let (keys, locks) = parse(input);

    let valid_pairs: usize = Itertools::cartesian_product(keys.iter(), locks.iter())
        .filter(|(key, lock)| key.unlocks(lock))
        .count();

    println!("Pairs: {}", valid_pairs);
}

#[derive(Debug)]
struct Key {
    heights: [i8; 5],
}

impl Key {
    fn unlocks(&self, lock: &Lock) -> bool {
        std::iter::zip(self.heights.iter(), lock.heights.iter()).all(|(k, h)| k + h <= 5)
    }
}

#[derive(Debug)]
struct Lock {
    heights: [i8; 5],
}

fn parse(input: &str) -> (Vec<Key>, Vec<Lock>) {
    let mut keys = vec![];
    let mut locks = vec![];

    for block in input.split("\n\n") {
        if let Some(top_line) = block.lines().next() {
            let is_key = top_line.chars().all(|c| c != '#');
            let mut heights = [-1; 5];

            for row in block.lines() {
                for (h, c) in std::iter::zip(heights.iter_mut(), row.chars()) {
                    if c == '#' {
                        *h += 1;
                    }
                }
            }

            if is_key {
                keys.push(Key { heights });
            } else {
                locks.push(Lock { heights })
            }
        }
    }

    (keys, locks)
}
