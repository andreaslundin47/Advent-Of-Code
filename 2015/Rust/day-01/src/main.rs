fn main() {
    let input = include_str!("../input.txt").trim();

    let final_floor: i32 = input.chars().fold(0, |acc, step| {
        acc + match step {
            '(' => 1,
            ')' => -1,
            _ => panic!("Bad input!"),
        }
    });
    println!("Part 1. Final floor = {final_floor}");


    let mut floor = 0;

    for (position, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("Bad input!"),
        };

        if floor == -1 {
            println!("Part 2. Enters cellar at step {}", position + 1);
            return;
        }
    }
}