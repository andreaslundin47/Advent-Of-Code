use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let reports = parse_reports(input);

    let safe_count = reports.iter().filter(|rep| rep.is_safe()).count();
    println!("Part 1. Safe reports: {}", safe_count);

    let dampened_safe_count = reports.iter().filter(|rep| rep.is_dampened_safe()).count();
    println!("Part 2. Dampened safe reports: {}", dampened_safe_count);
}

struct Report {
    levels: Vec<u32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut all_increasing = true;
        let mut all_decresing = true;

        for (a, b) in self.levels.iter().tuple_windows() {
            let diff = b.abs_diff(*a);

            if diff < 1 || diff > 3 {
                return false;
            } else if b > a {
                all_decresing = false;
            } else {
                all_increasing = false;
            }
        }

        all_increasing || all_decresing
    }

    fn is_dampened_safe(&self) -> bool {
        for remove_level_index in 0..self.levels.len() {
            let modified_report = Report {
                levels: self
                    .levels
                    .iter()
                    .enumerate()
                    .filter_map(|(i, lvl)| (i != remove_level_index).then_some(*lvl))
                    .collect(),
            };

            if modified_report.is_safe() {
                return true;
            }
        }

        return false;
    }
}

fn parse_reports(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|line| Report {
            levels: line
                .split_ascii_whitespace()
                .map(|num| num.parse().unwrap())
                .collect(),
        })
        .collect()
}
