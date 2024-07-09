use std::collections::VecDeque;

fn main() {
    const TARGET_VOLUME: usize = 150;

    let input = include_str!("../input.txt").trim();

    let container_volumes: Vec<usize> = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let mut number_of_containers_used = vec![];
    let mut combos = VecDeque::from([(0usize, 0usize, container_volumes.len())]);

    while let Some((volume, containers_used, last_index_used)) = combos.pop_front() {
        if volume > TARGET_VOLUME {
            continue;
        } else if volume == TARGET_VOLUME {
            number_of_containers_used.push(containers_used);
        } else if last_index_used > 0 {
            let new_container_volume = container_volumes[last_index_used - 1];

            combos.push_back((
                volume + new_container_volume,
                containers_used + 1,
                last_index_used - 1,
            ));

            combos.push_back((volume, containers_used, last_index_used - 1));
        }
    }

    let valid_count = number_of_containers_used.len();
    let min = number_of_containers_used
        .iter()
        .min()
        .expect("Should have found valid solutions");
    let min_combo_count = number_of_containers_used
        .iter()
        .filter(|c| c == &min)
        .count();

    println!("Part 1. Valid combinations = {valid_count}");
    println!("Part 2. Number of combinations using only {min} containers = {min_combo_count}");
}
