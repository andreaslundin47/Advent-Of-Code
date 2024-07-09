use std::collections::{HashMap, HashSet};

fn main() {
    let raw_input = include_str!("../input.txt");
    solve_parts_one_and_two(&raw_input);
}
// --------------------------------------------------------------

fn solve_parts_one_and_two(input: &str) {
    let grid_map: GridMap = parse_map(&input);

    // Part 1. Walk around the loop, find where it is, and its length

    let mut current_pos: Vec2 = grid_map.start;

    let mut next_pos: Vec2 = grid_map
        .pipe_segments
        .get(&current_pos)
        .expect("Start should have adjacents")
        .exits
        .0;

    let mut loop_positions = HashSet::from([current_pos]);

    println!("{:?} - {:?}", current_pos, next_pos);

    while !loop_positions.contains(&next_pos) {
        loop_positions.insert(next_pos);

        let next_segment: &Segment = grid_map
            .pipe_segments
            .get(&next_pos)
            .expect("Adjacent segment should exist in map");

        let (exit_a, exit_b) = next_segment.exits;

        let next_next_pos: Vec2 = if current_pos == exit_a {
            exit_b
        } else {
            exit_a
        };

        current_pos = next_pos;
        next_pos = next_next_pos;
    }

    let loop_length = loop_positions.len();
    let longest_distance = loop_length / 2;

    println!("Part 1. Distance = {longest_distance}");

    // Scan map line by line, from left to right. Keep track of pipe crossings to
    // know when we are inside or outside. Count the inside positions.

    let mut interior_count = 0;

    for y in 0..grid_map.height {
        let mut is_inside = false;
        let mut up_down: i8 = 0;

        for x in 0..grid_map.width {
            let is_on_pipe_loop = loop_positions.contains(&Vec2::from(x as i32, y as i32));

            if is_on_pipe_loop {
                let sym = grid_map.map[y][x];

                match sym {
                    '|' => is_inside = !is_inside,
                    'F' | 'J' => up_down += 1,
                    'L' | '7' => up_down -= 1,
                    _ => (),
                }

                if up_down.abs() == 2 {
                    is_inside = !is_inside;
                    up_down = 0;
                }
            } else if is_inside {
                interior_count += 1;
            }
        }
        assert!(!is_inside, "Should be outside at end of line");
    }

    println!("Part 2. Interior = {interior_count}");
}
// --------------------------------------------------------------

fn parse_map(input: &str) -> GridMap {
    let mut start = Vec2::from(0, 0);
    let mut map = vec![];
    let mut pipe_segments = HashMap::<Vec2, Segment>::new();

    for (y, row) in input.lines().enumerate() {
        let mut row_map = vec![];

        for (x, symbol) in row.chars().enumerate() {
            row_map.push(symbol);

            if symbol == '.' {
                continue;
            } else if symbol == 'S' {
                start = Vec2::from(x as i32, y as i32);
                continue;
            } else {
                let x = x as i32;
                let y = y as i32;
                let segment = Segment::from(x, y, symbol);
                pipe_segments.insert(Vec2::from(x, y), segment);
            }
        }

        map.push(row_map);
    }

    let start_segment = determine_start_segment(&start, &pipe_segments);

    map[start.y as usize][start.x as usize] = start_segment.symbol;
    pipe_segments.insert(start, start_segment);

    let height = map.len();
    let width = map.first().expect("Map should have at least one row").len();

    GridMap {
        start,
        width,
        height,
        map,
        pipe_segments,
    }
}
// --------------------------------------------------------------

fn determine_start_segment(start: &Vec2, map: &HashMap<Vec2, Segment>) -> Segment {
    let x = start.x as i32;
    let y = start.y as i32;

    let north_vec = Vec2::from(x, y - 1);
    let south_vec = Vec2::from(x, y + 1);
    let west_vec = Vec2::from(x - 1, y);
    let east_vec = Vec2::from(x + 1, y);

    let north = if let Some(segment) = map.get(&north_vec) {
        ['|', 'F', '7'].contains(&segment.symbol)
    } else {
        false
    };

    let south = if let Some(segment) = map.get(&south_vec) {
        ['|', 'J', 'L'].contains(&segment.symbol)
    } else {
        false
    };

    let west = if let Some(segment) = map.get(&west_vec) {
        ['-', 'F', 'L'].contains(&segment.symbol)
    } else {
        false
    };

    let east = if let Some(segment) = map.get(&east_vec) {
        ['-', '7', 'J'].contains(&segment.symbol)
    } else {
        false
    };

    match (north, south, west, east) {
        (true, true, false, false) => Segment {
            symbol: '|',
            exits: (north_vec, south_vec),
        },
        (true, false, true, false) => Segment {
            symbol: 'J',
            exits: (north_vec, west_vec),
        },
        (true, false, false, true) => Segment {
            symbol: 'L',
            exits: (north_vec, east_vec),
        },
        (false, true, false, true) => Segment {
            symbol: 'F',
            exits: (south_vec, east_vec),
        },
        (false, true, true, false) => Segment {
            symbol: '7',
            exits: (south_vec, west_vec),
        },
        (false, false, true, true) => Segment {
            symbol: '-',
            exits: (west_vec, east_vec),
        },
        _ => panic!("Not a valid starting position"),
    }
}
// --------------------------------------------------------------

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn from(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }
}
// --------------------------------------------------------------

#[derive(Debug, Eq, PartialEq, Hash)]
struct Segment {
    symbol: char,
    exits: (Vec2, Vec2),
}

impl Segment {
    fn from(x: i32, y: i32, symbol: char) -> Segment {
        let left = Vec2::from(x - 1, y);
        let right = Vec2::from(x + 1, y);
        let up = Vec2::from(x, y - 1);
        let down = Vec2::from(x, y + 1);

        let exits = match symbol {
            '|' => (up, down),
            '-' => (left, right),
            'L' => (up, right),
            'J' => (left, up),
            '7' => (left, down),
            'F' => (right, down),
            _ => panic!("The map symbol {} should not be handled here!", symbol),
        };

        Segment { symbol, exits }
    }
}
// --------------------------------------------------------------

struct GridMap {
    start: Vec2,
    width: usize,
    height: usize,
    map: Vec<Vec<char>>,
    pipe_segments: HashMap<Vec2, Segment>,
}
// --------------------------------------------------------------
