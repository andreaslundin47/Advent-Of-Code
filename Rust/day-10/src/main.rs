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

    let mut chars = s.chars();

    let mut current_char = match chars.next() {
        Some(ch) => ch,
        None => return output,
    };

    let mut current_count = 1;

    for c in chars {
        if c != current_char {
            output += &format!("{current_count}{current_char}");
            current_char = c;
            current_count = 0;
        }
        current_count += 1;
    }

    output += &format!("{current_count}{current_char}");

    output
}   