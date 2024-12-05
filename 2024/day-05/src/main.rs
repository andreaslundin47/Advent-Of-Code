use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let (rules, orderings) = parse(input);

    let mut precedence_map: HashMap<i32, HashSet<i32>> = HashMap::new();

    for rule in rules.iter() {
        precedence_map
            .entry(rule.before)
            .or_insert_with(HashSet::new)
            .insert(rule.after);
    }

    let compare = |a: &i32, b: &i32| -> Ordering {
        if let Some(follows_b) = precedence_map.get(b) {
            if follows_b.contains(a) {
                return Ordering::Greater;
            }
        }

        if let Some(follows_a) = precedence_map.get(a) {
            if follows_a.contains(b) {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    };

    let sum_one: i32 = orderings
        .iter()
        .map(|ordering| {
            let is_correct_order = ordering
                .iter()
                .tuple_windows()
                .all(|(a, b)| compare(a, b) != Ordering::Greater);

            match is_correct_order {
                true => ordering[ordering.len() / 2],
                false => 0,
            }
        })
        .sum();

    println!("Part 1. Sum: {}", sum_one);

    let sum_two: i32 = orderings
        .iter()
        .map(|order| {
            let sorted: PageOrdering = order.iter().cloned().sorted_by(compare).collect();

            match sorted != *order {
                true => sorted[sorted.len() / 2],
                false => 0,
            }
        })
        .sum();

    println!("Part 2. Sum: {}", sum_two);
}

struct Rule {
    before: i32,
    after: i32,
}

type PageOrdering = Vec<i32>;

fn parse(input: &str) -> (Vec<Rule>, Vec<PageOrdering>) {
    let (raw_rules, raw_ordering) = input.split_once("\n\n").unwrap();

    let rules = raw_rules
        .lines()
        .map(|line| {
            let (be, af) = line.split_once('|').unwrap();
            Rule {
                before: be.parse::<i32>().unwrap(),
                after: af.parse::<i32>().unwrap(),
            }
        })
        .collect();

    let orderings = raw_ordering
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    (rules, orderings)
}
