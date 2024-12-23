use std::{collections::HashMap, usize};

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let codes: Vec<&str> = input.lines().collect();

    println!("Part 1. {}", complexity(&codes, 2));
    println!("Part 2. {}", complexity(&codes, 25));
}

fn complexity(codes: &Vec<&str>, intermediate_robots: usize) -> usize {
    let mut complexity_sum = 0;

    for code in codes {
        let presses = minimum_button_presses_for_code(code, intermediate_robots);

        let code_number = code
            .strip_suffix("A")
            .and_then(|c| c.parse::<usize>().ok())
            .unwrap();

        let complexity = code_number * presses;

        complexity_sum += complexity;
    }

    complexity_sum
}

fn numpad_moves(code: &str) -> Vec<String> {
    let key_locations = HashMap::from([
        ('A', (2, 3)),
        ('0', (1, 3)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
    ]);

    let mut moves = vec!["".to_string()];

    for (c1, c2) in std::iter::once('A').chain(code.chars()).tuple_windows() {
        let (sx, sy) = key_locations.get(&c1).unwrap();
        let (ex, ey) = key_locations.get(&c2).unwrap();

        let dx = ex - sx;
        let dy = ey - sy;

        let dx = if dx >= 0 {
            ">".repeat(dx as usize)
        } else {
            "<".repeat(-dx as usize)
        };

        let dy = if dy >= 0 {
            "v".repeat(dy as usize)
        } else {
            "^".repeat(-dy as usize)
        };

        let not_dy_first = ['7', '4', '1'].contains(&c1) && ['0', 'A'].contains(&c2);
        let not_dx_first = ['7', '4', '1'].contains(&c2) && ['0', 'A'].contains(&c1);

        let current_moves = moves;
        moves = vec![];

        if dx.is_empty() || dy.is_empty() {
            for m in current_moves {
                moves.push(format!("{}{}{}A", m, dx, dy));
            }
        } else {
            for m in current_moves {
                if !not_dx_first {
                    moves.push(format!("{}{}{}A", m, dx, dy));
                }

                if !not_dy_first {
                    moves.push(format!("{}{}{}A", m, dy, dx));
                }
            }
        }
    }

    moves
}

fn get_keypad_paths(from: char, to: char) -> Vec<String> {
    let paths = match (from, to) {
        ('A', 'A') => vec!["A"],
        ('A', '<') => vec!["v<<A", "<v<A"],
        ('A', '>') => vec!["vA"],
        ('A', '^') => vec!["<A"],
        ('A', 'v') => vec!["<vA", "v<A"],
        ('<', 'A') => vec![">>^A", ">^>A"],
        ('<', '<') => vec!["A"],
        ('<', '>') => vec![">>A"],
        ('<', '^') => vec![">^A"],
        ('<', 'v') => vec![">A"],
        ('>', 'A') => vec!["^A"],
        ('>', '<') => vec!["<<A"],
        ('>', '>') => vec!["A"],
        ('>', '^') => vec!["<^A", "^<A"],
        ('>', 'v') => vec!["<A"],
        ('^', 'A') => vec![">A"],
        ('^', '<') => vec!["v<A"],
        ('^', '>') => vec!["v>A", "v>A"],
        ('^', '^') => vec!["A"],
        ('^', 'v') => vec!["vA"],
        ('v', 'A') => vec!["^>A", ">^A"],
        ('v', '<') => vec!["<A"],
        ('v', '>') => vec![">A"],
        ('v', '^') => vec!["^A"],
        ('v', 'v') => vec!["A"],
        _ => panic!("Unexpected inputs!"),
    };

    paths.iter().map(|p| p.to_string()).collect()
}

#[derive(Hash, Eq, PartialEq)]
struct Entry {
    from: char,
    to: char,
    robots_between: usize,
}

fn minimum_button_presses_for_code(code: &str, robot_layers: usize) -> usize {
    let keypad_button_sequences = numpad_moves(code);

    let mut min_presses = usize::MAX;
    let mut memo: HashMap<Entry, usize> = HashMap::new();

    for button_sequence in keypad_button_sequences {
        let mut presses = 0;

        for (a, b) in std::iter::once('A')
            .chain(button_sequence.chars())
            .tuple_windows()
        {
            presses += go_and_press_count(a, b, robot_layers - 1, &mut memo);
        }

        min_presses = min_presses.min(presses);
    }

    min_presses
}

fn go_and_press_count(
    from: char,
    to: char,
    robots_between: usize,
    memos: &mut HashMap<Entry, usize>,
) -> usize {
    let memo_entry = Entry {
        from,
        to,
        robots_between,
    };

    if robots_between == 0 {
        let paths = get_keypad_paths(from, to);
        return paths[0].len();
    }

    if let Some(&presses) = memos.get(&memo_entry) {
        return presses;
    }

    let button_sequences: Vec<String> = get_keypad_paths(from, to);
    let mut min_presses = usize::MAX;

    for button_sequence in button_sequences {
        let mut presses = 0;
        for (a, b) in std::iter::once('A')
            .chain(button_sequence.chars())
            .tuple_windows()
        {
            presses += go_and_press_count(a, b, robots_between - 1, memos);
        }

        min_presses = min_presses.min(presses);
    }

    memos.insert(memo_entry, min_presses);

    min_presses
}
