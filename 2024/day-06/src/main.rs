use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt").trim();

    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
    let mut map_positions: HashSet<(i32, i32)> = HashSet::new();

    let mut guard_position = (0, 0);
    let mut guard_direction = (0, -1);

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            map_positions.insert((x as i32, y as i32));
            if c == '^' {
                guard_position = (x as i32, y as i32);
            } else if c == '#' {
                obstacles.insert((x as i32, y as i32));
            }
        }
    }

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(guard_position);

    let mut valid_blocks = 0;

    loop {
        let (x, y) = guard_position;
        let (dx, dy) = guard_direction;
        let forward = (x + dx, y + dy);

        let old_position = guard_position;
        let old_direction = guard_direction;

        if obstacles.contains(&forward) {
            guard_direction = turn(guard_direction);
        } else {
            guard_position = forward;
        }

        if map_positions.contains(&guard_position) {
            if !visited.contains(&guard_position) {
                if has_cycle(
                    guard_position,
                    old_direction,
                    old_position,
                    &map_positions,
                    &obstacles,
                ) {
                    valid_blocks += 1;
                }
            }

            visited.insert(guard_position);
        } else {
            break;
        }
    }

    println!("Part 1. Visited: {}", visited.len());
    println!("Part 2. Blocks: {}", valid_blocks);
}

fn turn(direction: (i32, i32)) -> (i32, i32) {
    match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => panic!("Not a valid direction!"),
    }
}

fn has_cycle(
    new_obstacle: (i32, i32),
    guard_dir: (i32, i32),
    guard_pos: (i32, i32),
    map_positions: &HashSet<(i32, i32)>,
    obstacles: &HashSet<(i32, i32)>,
) -> bool {
    let mut current_dir = turn(guard_dir);
    let mut current_pos = guard_pos;

    let mut visited: HashSet<((i32, i32), (i32, i32))> = HashSet::new();

    loop {
        let (x, y) = current_pos;
        let (dx, dy) = current_dir;
        let forward = (x + dx, y + dy);

        if forward == new_obstacle || obstacles.contains(&forward) {
            current_dir = turn(current_dir);
        } else {
            current_pos = forward;
        }

        if visited.contains(&(current_pos, current_dir)) {
            return true;
        }

        visited.insert((current_pos, current_dir));

        if !map_positions.contains(&current_pos) {
            return false;
        }
    }
}
