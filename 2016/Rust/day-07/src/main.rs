use std::collections::HashSet;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();

    let supports_tls = input.lines().filter(supports_tls).count();
    println!("Part 1. Supports TLS = {}", supports_tls);

    let supports_ssl = input.lines().filter(supports_ssl).count();
    println!("Part 2. Supports SSL = {}", supports_ssl);
}

fn supports_tls(ip: &&str) -> bool {
    let (supernets, hypernets) = get_sequences(ip);

    if hypernets.iter().any(|s| has_abba(s)) {
        return false;
    }

    if supernets.iter().any(|s| has_abba(s)) {
        return true;
    }

    false
}

fn supports_ssl(ip: &&str) -> bool {
    let (supernets, hypernets) = get_sequences(ip);

    let abas: Vec<String> = hypernets.iter().flat_map(|s| get_abas(s)).collect();
    let babs: HashSet<String> = supernets.iter().flat_map(|s| get_abas(s)).collect();

    for aba in abas {
        let a = aba.chars().next().unwrap();
        let b = aba.chars().nth(1).unwrap();
        let bab = format!("{b}{a}{b}");
        if babs.contains(&bab) {
            return true;
        }
    }

    false
}

fn has_abba(seq: &str) -> bool {
    for (a, b, c, d) in seq.chars().tuple_windows() {
        if a == d && b == c && a != b {
            return true;
        }
    }
    false
}

fn get_abas(chars: &str) -> HashSet<String> {
    chars
        .chars()
        .tuple_windows()
        .filter(|(a, b, c)| a == c && a != b)
        .map(|(a, b, c)| format!("{a}{b}{c}"))
        .collect()
}

fn get_sequences(ip: &&str) -> (Vec<String>, Vec<String>) {
    let mut supernets = vec![];
    let mut hypernets = vec![];

    let mut current = vec![];

    for c in ip.chars() {
        match c {
            '[' => {
                supernets.push(current.into_iter().collect());
                current = vec![];
            }
            ']' => {
                hypernets.push(current.into_iter().collect());
                current = vec![];
            }
            _ => {
                current.push(c);
            }
        }
    }

    if !current.is_empty() {
        supernets.push(current.into_iter().collect());
    }

    (supernets, hypernets)
}
