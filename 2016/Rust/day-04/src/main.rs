use itertools::Itertools;
use regex::Regex;

fn main() {
    let input = include_str!("../input.txt").trim();
    let rooms: Vec<Room> = parse(input);
    let valid_rooms: Vec<Room> = rooms.into_iter().filter(|room| room.is_valid()).collect();
    let sector_id_sum: usize = valid_rooms.iter().map(|room| room.sector_id).sum();

    println!("Part 1. Sum of valid id's = {sector_id_sum}");

    let room = valid_rooms
        .iter()
        .find(|room| room.name_cipher_shifted().contains("northpole"))
        .expect("Room exists");

    println!("Part 2. Northpole Objects room id = {}", room.sector_id);
}

#[derive(Debug)]
struct Room<'a> {
    name: &'a str,
    sector_id: usize,
    checksum: &'a str,
}

impl<'a> Room<'a> {
    fn is_valid(&self) -> bool {
        let sorted_top_five: String = self
            .name
            .chars()
            .filter(|c| c != &'-')
            .counts()
            .into_iter()
            .map(|(letter, count)| (-(count as i8), letter))
            .sorted()
            .take(5)
            .map(|(_, letter)| letter)
            .collect();

        self.checksum == sorted_top_five
    }

    fn name_cipher_shifted(&self) -> String {
        self.name
            .chars()
            .dropping_back(1)
            .map(|c| match c {
                '-' => ' ',
                _ => (b'a' + (c as u8 - b'a' + (self.sector_id % 26) as u8) % 26) as char,
            })
            .collect()
    }
}

fn parse(input: &str) -> Vec<Room> {
    let re = Regex::new(r"^([[a-z]+\-]+)([0-9]+)\[([a-z]+)\]$").expect("Valid regex");

    input
        .lines()
        .map(|line| {
            let (_, [name, nums, checksum]) = re.captures(line).unwrap().extract();
            let sector_id = nums.parse::<usize>().expect("A valid integer");

            Room {
                name,
                sector_id,
                checksum,
            }
        })
        .collect_vec()
}
