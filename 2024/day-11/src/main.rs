use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt").trim();

    let start_numbers: Vec<usize> = input
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect();

    println!("Part 1. Stones: {}", count(&start_numbers, 25));
    println!("Part 2. Stones: {}", count(&start_numbers, 75));
}

fn count(start_numbers: &[usize], iterations: usize) -> usize {
    let mut tally: HashMap<usize, usize> = HashMap::new();

    for n in start_numbers.iter() {
        tally.insert(*n, 1);
    }

    for _ in 0..iterations {
        let tally_counts: Vec<(usize, usize)> = tally.into_iter().collect();
        tally = HashMap::new();

        for (num, count) in tally_counts {
            if num == 0 {
                *tally.entry(1).or_insert(0) += count;
            } else if let Some((a, b)) = even_number_split(num) {
                *tally.entry(a).or_insert(0) += count;
                *tally.entry(b).or_insert(0) += count;
            } else {
                *tally.entry(2024 * num).or_insert(0) += count;
            }
        }
    }

    tally.values().copied().sum()
}

fn even_number_split(num: usize) -> Option<(usize, usize)> {
    let mut n = num;

    let mut digits = 0;
    while n > 0 {
        n /= 10;
        digits += 1;
    }

    if digits % 2 == 1 {
        return None;
    }

    let part_len = digits / 2;

    let a = num / 10usize.pow(part_len);
    let b = num % 10usize.pow(part_len);

    Some((a, b))
}
