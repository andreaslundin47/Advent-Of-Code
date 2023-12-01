fn main() {
    let input = include_str!("../input.txt").trim();
    part_one(input);
    part_two(input);
}


fn part_one(input: &str) {

    let sum: u32 = input.lines().map(|line| {

        let digits: Vec<u32> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        let first = digits.first().unwrap();
        let last = digits.last().unwrap();

        first * 10 + last
    })
    .sum();

    println!("Part 1: Sum = {sum}");
}


fn part_two(input: &str) {

    let digit_words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let sum: u32 = input.lines().map(|line| {

        let digits: Vec<u32> = line
            .chars()
            .enumerate()
            .filter_map(|(i,c)| {
                if c.is_digit(10) {
                    c.to_digit(10)
                }
                else {
                    let pos = digit_words.iter().position(|word| line[i..].starts_with(word));
                    pos.map_or(None, |p| Some(p as u32 + 1))
                }
            })
            .collect::<Vec<u32>>();

        digits.first().unwrap() * 10 + digits.last().unwrap()
    })
    .sum();

    println!("Part 2: Sum = {sum}");
}