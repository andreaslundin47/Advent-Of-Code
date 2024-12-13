mod parsing;

fn main() {
    let input = include_str!("../input.txt").trim();
    let entries: Vec<Entry> = parsing::parse_entries(input).expect("A valid parse").1;

    println!("Part 1. Cost: {}", total_cost(&entries));

    let new_entries: Vec<Entry> = entries.iter().map(update_entry).collect();
    println!("Part 2. Cost: {}", total_cost(&new_entries));
}

struct Entry {
    x_a: i64,
    x_b: i64,
    y_a: i64,
    y_b: i64,
    x_target: i64,
    y_target: i64,
}

fn update_entry(entry: &Entry) -> Entry {
    let extra_term = 10_000_000_000_000;

    Entry {
        x_target: entry.x_target + extra_term,
        y_target: entry.y_target + extra_term,
        ..*entry
    }
}

fn total_cost(entries: &[Entry]) -> i64 {
    entries
        .iter()
        .filter_map(|e| {
            // Solving the system of equations:
            //
            //   x = x_a * a + x_b * b
            //   y = y_a * a + y_b * b
            //
            //  for a and b, then calculating the cost:  cost = 3a + b

            let denominator = e.x_a * e.y_b - e.x_b * e.y_a;

            if denominator == 0 {
                // No solution
                return None;
            }

            let numerator_a = e.y_b * e.x_target - e.x_b * e.y_target;
            let numerator_b = e.x_a * e.y_target - e.y_a * e.x_target;

            if numerator_a % denominator != 0 || numerator_b % denominator != 0 {
                // Does not have an integer solution
                return None;
            }

            let a = numerator_a / denominator;
            let b = numerator_b / denominator;

            let cost = 3 * a + b;
            Some(cost)
        })
        .sum::<i64>()
}
