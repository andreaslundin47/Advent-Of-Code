fn main() {
    let current = "cqjxjnds";

    let p1 = find_next(&current);
    println!("Part 1. Next password is: {p1}");

    let p2 = find_next(&p1);
    println!("Part 2. Next password is: {p2}");
}


fn find_next(old: &str) -> String {

    let mut word: Vec<u8> = old.chars().map(|c| c as u8 - b'a').collect();

    loop {
        for c in word.iter_mut().rev() {
            *c = (*c + 1) % 26;

            if *c != 0 { break; }
        }

        if is_valid_password(&word) { break; }
    }

    let back: String = word.iter().map(|v| (v + b'a') as char).collect();

    back
}


fn is_valid_password(word: &Vec<u8>) -> bool {
    let forbidden: Vec<u8> = vec!['i', 'l', 'o']
        .iter()
        .map(|&c| c as u8 - b'a')
        .collect();

    if word.iter().any(|v| forbidden.contains(v)) {
        return false;
    }

    if !word.windows(3).any(|w|  w[0] + 1 == w[1] && w[0] + 2 == w[2]) {
        return false;
    }

    for i in 0..word.len()-1 {
        if word[i] == word[i+1] {
            let c = word[i];
            for j in (i+2)..word.len()-1 {
                if word[j] != c && word[j] == word[j+1] {
                    return true;
                }
            }
            return false;
        }
    }

    false
}