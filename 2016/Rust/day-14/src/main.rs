use itertools::Itertools;
use std::collections::VecDeque;

const PUZZLE_SALT: &str = "ahsbgdzn";

fn main() {
    println!("Part 1. Index = {}", key64th_at_index(1));
    println!("Part 2. Index = {}", key64th_at_index(2017));
}

fn compute_hash(index: usize, applications: usize) -> Vec<char> {
    let mut hash = format!("{PUZZLE_SALT}{index}");
    for _ in 0..applications {
        let digest = md5::compute(hash);
        hash = format!("{:x}", digest);
    }
    hash.chars().collect()
}

fn find_three_repeater(key: &Vec<char>) -> Option<char> {
    key.windows(3)
        .find_map(|window| window.iter().all_equal().then_some(window[0]))
}

fn has_five_repeater(hash: &Vec<char>, repeater: char) -> bool {
    hash.windows(5)
        .find(|win| win[0] == repeater && win.iter().all_equal())
        .is_some()
}

fn key64th_at_index(hash_applications: usize) -> usize {
    let mut subsequent_hashes = VecDeque::new();
    for i in 0..1000 {
        subsequent_hashes.push_back(compute_hash(i, hash_applications));
    }

    let mut keys_found = 0;
    let mut current_index = 0;

    while keys_found < 64 {
        let key_candidate = subsequent_hashes
            .pop_front()
            .expect("Should always contain 1000 hashes");

        subsequent_hashes.push_back(compute_hash(current_index + 1000, hash_applications));

        if let Some(repeater) = find_three_repeater(&key_candidate) {
            if subsequent_hashes
                .iter()
                .find(|follower| has_five_repeater(follower, repeater))
                .is_some()
            {
                keys_found += 1;
            }
        }

        current_index += 1;
    }

    current_index - 1
}
