fn main() {
    let input = include_str!("../input.txt").trim();

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let number_pair: Vec<&str> = line.split_whitespace().collect();

        let a: usize = number_pair[0].parse().unwrap();
        let b: usize = number_pair[1].parse().unwrap();

        left_list.push(a);
        right_list.push(b);
    }

    left_list.sort();
    right_list.sort();

    let distance_sum: usize = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();

    println!("Part 1. Sum: {}", distance_sum);

    let similarity_score: usize = left_list
        .iter()
        .map(|left_value| {
            let right_occurances_count = right_list
                .iter()
                .filter(|&right_value| left_value == right_value)
                .count();

            left_value * right_occurances_count
        })
        .sum();

    println!("Part 2. Score: {}", similarity_score);
}
