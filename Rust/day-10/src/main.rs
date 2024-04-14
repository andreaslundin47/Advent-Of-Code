use itertools::Itertools;

fn main() {
    let input = "1321131112";

    let mut current = String::from(input);

    for _ in 0..40 {
        current = look_and_say(&current);
    }
    let length = current.len();
    println!("Part 1. Length = {length}");

    for _ in 0..10 {
        current = look_and_say(&current);
    }
    let length = current.len();
    println!("Part 2. Length = {length}");
}


fn look_and_say(s: &str) -> String {
    let mut output = String::new();

    for (key, group) in &s.chars().group_by(|c| *c) {
        let count = group.count();
        output += &format!("{count}{key}");
    }

    output
}   