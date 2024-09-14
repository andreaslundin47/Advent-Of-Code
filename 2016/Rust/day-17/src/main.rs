use std::collections::VecDeque;

const PUZZLE_INPUT: &str = "edjrjqaa";

fn main() {
    println!(
        "Part 1. Moves: {}",
        find_shortest_path().expect("A valid result")
    );
    println!("Part 2. Length: {}", find_longest_path());
}

struct State {
    x: i32,
    y: i32,
    moves: String,
}

fn is_inside(state: &State) -> bool {
    state.x >= 0 && state.x < 4 && state.y >= 0 && state.y < 4
}

fn get_neighbours(state: &State) -> Vec<State> {
    let passcode = format!("{}{}", PUZZLE_INPUT, state.moves);
    let digest = md5::compute(passcode);
    let door_status: Vec<bool> = format!("{:x}", digest)
        .chars()
        .take(4)
        .map(|c| "bcdef".contains(c))
        .collect();

    let directions = [("U", (-1, 0)), ("D", (1, 0)), ("L", (0, -1)), ("R", (0, 1))];

    directions
        .iter()
        .zip(door_status.iter())
        .filter_map(|(dir, is_open)| {
            let (dir_sym, (dx, dy)) = dir;

            is_open.then_some(State {
                x: state.x + dx,
                y: state.y + dy,
                moves: format!("{}{}", state.moves, dir_sym),
            })
        })
        .filter(is_inside)
        .collect()
}

fn find_shortest_path() -> Option<String> {
    let mut queue = VecDeque::new();

    queue.push_back(State {
        x: 0,
        y: 0,
        moves: "".to_string(),
    });

    while let Some(current_state) = queue.pop_front() {
        if current_state.x == 3 && current_state.y == 3 {
            return Some(current_state.moves);
        }

        queue.extend(get_neighbours(&current_state));
    }

    None
}

fn find_longest_path() -> usize {
    let mut queue = VecDeque::new();
    let mut longest_path: usize = 0;

    queue.push_back(State {
        x: 0,
        y: 0,
        moves: "".to_string(),
    });

    while let Some(current_state) = queue.pop_front() {
        if current_state.x == 3 && current_state.y == 3 {
            let current_length = current_state.moves.len();
            longest_path = longest_path.max(current_length);
            continue;
        }

        queue.extend(get_neighbours(&current_state));
    }

    longest_path
}
