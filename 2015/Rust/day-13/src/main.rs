use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt").trim();

    let mut people = Vec::new();
    let mut buffs: HashMap<(&str, &str), i32> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.trim_end_matches(".").split_whitespace().collect();

        let main_name = parts[0];
        let lose_or_gain = parts[2];
        let points = parts[3].parse::<i32>().expect("Should parse valid points");
        let other_name = parts[10];

        let diff = match lose_or_gain {
            "lose" => -points,
            "gain" => points,
            _ => panic!("This part of input is not 'lose' or 'gain' as expected"),
        };

        if !people.contains(&parts[0]) {
            people.push(parts[0]);
        }

        buffs.insert((main_name, other_name), diff);
    }

    let p1_max = find_max_score(&people, &buffs);
    println!("Part 1. Optimal seating score = {p1_max}");

    for p in &people {
        buffs.insert((p, "me"), 0);
        buffs.insert(("me", p), 0);
    }
    people.push("me");

    let p2_max = find_max_score(&people, &buffs);
    println!("Part 2. Optimal seating score = {p2_max}");
}


fn find_max_score(people: &Vec<&str>, buffs: &HashMap<(&str, &str), i32>) -> i32 {
    let mut max_score = i32::MIN;
    let people_count = people.len();

    for perm in people.iter().permutations(people.len()) {
        // Can break early due to circular symmetry
        if perm[0] != &people[0] {
            break;
        }

        let perm_score: i32 = perm
            .iter()
            .enumerate()
            .map(|(i, &&person)| {
                let on_left = *perm[(i + people_count - 1) % people_count];
                let on_right = *perm[(i + 1) % people_count];

                buffs[&(person, on_left)] + buffs[&(person, on_right)]
            })
            .sum();

        max_score = max_score.max(perm_score);
    }

    max_score
}