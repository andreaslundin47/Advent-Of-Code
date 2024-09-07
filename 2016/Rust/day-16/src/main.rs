const PUZZLE_INPUT: &str = "01000100010010111";

fn main() {
    println!("Part 1: {}", checksum(PUZZLE_INPUT, 272));
    println!("Part 2: {}", checksum(PUZZLE_INPUT, 35_651_584));
}

fn checksum(data: &str, target_length: usize) -> String {
    let mut current = data.to_string();

    while current.len() < target_length {
        let reverse_flipped: String = current
            .chars()
            .rev()
            .map(|d| match d {
                '0' => '1',
                '1' => '0',
                _ => panic!("Not binary digit!"),
            })
            .collect();

        current = format!("{current}0{reverse_flipped}");
    }

    current.truncate(target_length);

    while current.len() % 2 == 0 {
        current = current
            .chars()
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| if chunk[0] == chunk[1] { '1' } else { '0' })
            .collect()
    }

    current
}
