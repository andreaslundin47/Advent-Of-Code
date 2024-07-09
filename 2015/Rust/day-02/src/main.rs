use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();

    let area: usize = input
        .lines()
        .map(|line| {
            let (a, b, c) = line
                .split("x")
                .map(|d| d.parse::<usize>().expect("Valid integer parse"))
                .sorted()
                .collect_tuple()
                .expect("Data on valid format");

            3 * a * b + 2 * a * c + 2 * b * c
        })
        .sum();
    println!("Part 1. Area is {area}");

    let ribbon_feet: usize = input
        .lines()
        .map(|line| {
            let (a, b, c) = line
                .split("x")
                .map(|d| d.parse::<usize>().expect("Valid integer parse"))
                .sorted()
                .collect_tuple()
                .expect("Data on valid format");

            let wrapping = 2 * a + 2 * b;
            let bow = a * b * c;

            wrapping + bow
        })
        .sum();
    println!("Part 2. Ribbon length = {ribbon_feet}");
}
