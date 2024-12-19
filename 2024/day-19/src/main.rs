fn main() {
    let input = include_str!("../input.txt").trim();
    let (parts, designs) = parse(input);

    let ways_to_build: Vec<usize> = designs
        .iter()
        .map(|design| combinations_count(design, &parts))
        .collect();

    let possible_designs: usize = ways_to_build.iter().filter(|&&ways| ways > 0).count();
    println!("Part 1. Possible designs: {}", possible_designs);

    let all_combinations: usize = ways_to_build.iter().sum();
    println!("Part 2. Sum of ways: {}", all_combinations);
}

fn combinations_count(design: &str, parts: &[&str]) -> usize {
    // Bottom-up dynamic programming solution.
    // Builds a vec where the value at index i will be the number of
    // ways to construct the substring of the i leading characters

    let mut substring_combinations = [0].repeat(design.len() + 1);

    // One way to construct the length zero empty string
    substring_combinations[0] = 1;

    for len in 1..=design.len() {
        let leading_substring = &design[0..len];

        substring_combinations[len] = parts
            .iter()
            .map(|part| {
                if leading_substring.ends_with(part) && part.len() <= len {
                    substring_combinations[len - part.len()]
                } else {
                    0
                }
            })
            .sum();
    }

    substring_combinations[substring_combinations.len() - 1]
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let (parts, designs) = input.split_once("\n\n").unwrap();
    let parts: Vec<&str> = parts.split(", ").collect();
    let designs = designs.lines().collect();

    (parts, designs)
}
