fn main() {
    let input = include_str!("../input.txt").trim();

    let sum: usize = input
        .lines()
        .map(count_invisible_chars)
        .sum();
    println!("Part 1. Sum = {sum}");

    let sum: usize = input
        .lines()
        .map(count_added_chars)
        .sum();
    println!("Part 2. Sum = {sum}");
}


fn count_added_chars(s: &str) -> usize {
    let added: usize = s
        .chars()
        .map(|c| {
            match c {
                '"' => 1,
                '\\' => 1,
                _ => 0,
            }
        })
        .sum();

    added + 2
}


fn count_invisible_chars(s: &str) -> usize {
    let mut invisible = 2;
    let mut chars = s.chars();

    while let Some(c) = chars.next() {
        if c != '\\' {
            continue;
        }

        if let Some(n) = chars.next() {
            match n {
                '\\' => invisible += 1,
                '"' => invisible += 1,
                'x' => {
                    invisible += 3;
                    chars.next();
                    chars.next();
                },
                _ => ()
            }
        }
    }

    invisible
}