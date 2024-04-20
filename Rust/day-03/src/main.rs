use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt").trim();
    let mut visited: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    let mut current = (0, 0);
    for d in input.chars() {
        current = step(current, d);
        visited.insert(current);
    }
    println!("Part 1. Homes visited = {}", visited.len());

    visited.clear();

    let mut currents = [(0, 0), (0, 0)];
    let mut active = 0;
    for d in input.chars() {
        currents[active] = step(currents[active], d);
        visited.insert(currents[active]);
        active = (active + 1) % 2;
    }

    println!("Part 2. Homes visited = {}", visited.len());
}

fn step(current: (i32, i32), direction: char) -> (i32, i32) {
    let (dx, dy) = match direction {
        '^' => (0, -1),
        'v' => (0, 1),
        '<' => (-1, 0),
        '>' => (1, 0),
        _ => panic!("Bad input character!"),
    };

    (current.0 + dx, current.1 + dy)
}
