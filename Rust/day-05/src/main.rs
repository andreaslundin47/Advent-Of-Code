use itertools::Itertools;

fn main() {
    let passwords: Vec<&str> = include_str!("../input.txt").trim().lines().collect();

    let valid = passwords.iter().filter(|pw| part_one_validator(pw)).count();
    println!("Part 1. Number of nice passwords = {}", valid);

    let valid = passwords.iter().filter(|pw| part_two_validator(pw)).count();
    println!("Part 2. Number of nice passwords = {}", valid);
}

fn part_one_validator(pw: &str) -> bool {
    if pw.chars().filter(|c| "aeiou".contains(*c)).count() < 3 {
        return false;
    }

    if pw.chars().tuple_windows().filter(|(x, y)| x == y).count() == 0 {
        return false;
    }

    for bad in ["ab", "cd", "pq", "xy"] {
        if pw.contains(bad) {
            return false;
        }
    }

    true
}

fn part_two_validator(pw: &str) -> bool {
    if pw
        .chars()
        .tuple_windows()
        .filter(|(x, _, z)| x == z)
        .count()
        == 0
    {
        return false;
    }

    for (i, (x, y)) in pw.chars().zip(pw.chars().skip(1)).enumerate() {
        let pair = format!("{x}{y}");
        if let Some(j) = pw.rfind(pair.as_str()) {
            if j >= i + 2 {
                return true;
            }
        }
    }

    false
}
