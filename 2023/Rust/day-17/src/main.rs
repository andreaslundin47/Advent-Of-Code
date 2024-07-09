use std::collections::{HashMap, HashSet};
use glam::IVec2;
use priority_queue::PriorityQueue;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Next {
    Horizontal,
    Vertical,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct SearchState {
    pos: IVec2,
    next: Next,
}

fn main() {
    //let input = include_str!("../sample1-input.txt").trim();
    let input = include_str!("../input.txt").trim();

    let heat_drops = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars().enumerate().map(move |(x, c)| {
                let pos = IVec2::new(x as i32, y as i32);
                let heat_drop = c.to_digit(10).expect("Should be a digit");
                (pos, heat_drop as i32)
            })
        })
        .collect::<HashMap<IVec2, i32>>();

    let locations = heat_drops.keys().cloned().collect::<HashSet<IVec2>>();

    let max_x = locations.iter().map(|loc| loc.x).max().unwrap();
    let max_y = locations.iter().map(|loc| loc.y).max().unwrap();

    let start = IVec2::new(0, 0);
    let goal = IVec2::new(max_x, max_y);

    let start_state_1 = SearchState {
        pos: start,
        next: Next::Horizontal,
    };

    let start_state_2 = SearchState {
        pos: start,
        next: Next::Vertical,
    };

    let mut prio_queue = PriorityQueue::new();
    prio_queue.push(start_state_1, 0);
    prio_queue.push(start_state_2, 0);

    let mut distances: HashMap<SearchState, i32> = HashMap::new();
    distances.insert(start_state_1, 0);
    distances.insert(start_state_2, 0);

    let mut previous: HashMap<SearchState, SearchState> = HashMap::new();

    while let Some((current_state, _)) = prio_queue.pop() {

        let valid_neighbours = neighbours(&current_state, &locations);

        for neighbour in valid_neighbours {
            let delta_drop: i32 = delta_heat_drop(&current_state.pos, &neighbour.pos, &heat_drops);
            let dist_to_current = *distances
                .get(&current_state)
                .expect("Should have a distance");

            let proposed_distance = dist_to_current + delta_drop;

            if let Some(distance) = distances.get(&neighbour) {
                if proposed_distance < *distance {
                    distances.insert(neighbour, delta_drop + dist_to_current);
                    prio_queue.change_priority(&neighbour, -proposed_distance);
                }
            } else {
                distances.insert(neighbour, proposed_distance);
                prio_queue.push(neighbour, -proposed_distance);
            }

            previous.insert(neighbour, current_state);
        }
    }

    let a = distances.get(&SearchState { pos: goal, next: Next::Horizontal}).unwrap();
    let b = distances.get(&SearchState { pos: goal, next: Next::Vertical}).unwrap();
    let m = std::cmp::min(a, b);
    println!("{:?}    {:?} => {}", a, b, m);
}


fn delta_heat_drop(current_pos: &IVec2, next_pos: &IVec2, heat_drops: &HashMap<IVec2, i32>) -> i32 {
    let diff = *next_pos - *current_pos;
    let steps = diff.x.abs() + diff.y.abs();
    let delta = diff / steps;

    let mut sum = 0;

    for factor in 1..=steps {
        let a = *current_pos + factor * delta;
        sum += heat_drops.get(&a).unwrap();
    }

    sum
}

fn neighbours(current_state: &SearchState, locations: &HashSet<IVec2>) -> HashSet<SearchState> {
    let mut candidates = HashSet::new();

    match current_state.next {
        Next::Horizontal => {
            for x in -3..=3 {
                if x != 0 {
                    candidates.insert(SearchState {
                        pos: IVec2::new(current_state.pos.x + x, current_state.pos.y),
                        next: Next::Vertical,
                    });
                }
            }
        }
        Next::Vertical => {
            for y in -3..=3 {
                if y != 0 {
                    candidates.insert(SearchState {
                        pos: IVec2::new(current_state.pos.x, current_state.pos.y + y),
                        next: Next::Horizontal,
                    });
                }
            }
        }
    }

    candidates
        .iter()
        .filter(|c| locations.contains(&c.pos))
        .cloned()
        .collect()
}
