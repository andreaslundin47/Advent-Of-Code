fn main() {
    let input: Vec<&str> = include_str!("../input.txt").trim().split(',').collect();

    // Part 1
    let sum: usize = input.iter().map(|step| aoc_hash(step)).sum();
    println!("Part 1. Sum of hashes: {sum}");

    // Part 2
    let mut aoc_map = AoCHashMap::new();

    for op in input.iter() {
        if op.contains('=') {
            let (key, value) = op.split_once('=').expect("Valid key-value pair");
            let value = value.parse::<usize>().expect("A digit");
            aoc_map.insert(key, value);
        } else if op.contains('-') {
            let key = op.strip_suffix('-').expect("key followed by a '-' only");
            aoc_map.remove(key);
        }
    }

    let total_power = aoc_map.total_focusing_power();
    println!("Part 2. Total power: {total_power}");
}

fn aoc_hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

struct AoCHashMap {
    boxes: Vec<Vec<(String, usize)>>,
}

impl AoCHashMap {
    fn new() -> AoCHashMap {
        let mut boxes = vec![];
        for _ in 0..256 {
            boxes.push(vec![]);
        }

        AoCHashMap { boxes }
    }

    fn insert(&mut self, key: &str, value: usize) {
        let hash = aoc_hash(key);
        let key = key.to_string();

        if let Some(index) = self.boxes[hash].iter().position(|entry| entry.0 == key) {
            self.boxes[hash][index] = (key, value);
        } else {
            self.boxes[hash].push((key, value));
        }
    }

    fn remove(&mut self, key: &str) {
        let hash = aoc_hash(key);
        let key = key.to_string();

        if let Some(index) = self.boxes[hash].iter().position(|entry| entry.0 == key) {
            self.boxes[hash].remove(index);
        }
    }

    fn total_focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_index, map_box)| {
                map_box
                    .iter()
                    .enumerate()
                    .map(|(entry_index, lens)| {
                        let focal_length = lens.1;
                        (box_index + 1) * (entry_index + 1) * focal_length
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}
