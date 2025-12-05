use std::ops::RangeInclusive;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    let (fresh_ranges, ingredient_ids) = parse(input);

    let mut count_fresh = 0;

    for ingredient in ingredient_ids.iter() {
        if fresh_ranges.iter().any(|range| range.contains(ingredient)) {
            count_fresh += 1;
        }
    }

    println!("Part 1. {count_fresh}");

    let sorted_ranges = fresh_ranges
        .into_iter()
        .sorted_by_key(|r| *r.start())
        .collect::<Vec<_>>();

    let mut nonoverlapping_ranges = vec![];
    let mut current_range = sorted_ranges[0].clone();

    for range in sorted_ranges {
        if range.start() <= current_range.end() {
            current_range = (*current_range.start())..=(*current_range.end().max(range.end()));
        } else {
            nonoverlapping_ranges.push(current_range);
            current_range = range;
        }
    }
    nonoverlapping_ranges.push(current_range);

    let sum_ranges: usize = nonoverlapping_ranges
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum();

    println!("Part 2. {sum_ranges}");
}

fn parse(s: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (ranges, ids) = s.trim().split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (lower, upper) = line.split_once('-').unwrap();
            lower.parse::<usize>().unwrap()..=upper.parse::<usize>().unwrap()
        })
        .collect();

    let ids = ids.lines().map(|id| id.parse::<usize>().unwrap()).collect();

    (ranges, ids)
}
