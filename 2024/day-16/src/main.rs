use std::{
    cmp::{Ordering, Reverse},
    collections::{HashMap, HashSet, VecDeque},
};

use glam::IVec2;
use itertools::Itertools;
use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../input.txt").trim();
    let world = parse(input);

    let start = State {
        pos: world.start,
        dir: IVec2::X,
    };

    let mut scores: HashMap<State, i32> = HashMap::from([(start, 0)]);

    let mut previous_states: HashMap<State, HashSet<State>> = HashMap::new();

    let mut queue = PriorityQueue::new();
    queue.push(start, Reverse(0));

    while let Some((current_state, Reverse(current_score))) = queue.pop() {
        for (neighbour_state, score_increase) in [
            (current_state.forward(), 1),
            (current_state.turn_left(), 1000),
            (current_state.turn_right(), 1000),
        ] {
            if world.walls.contains(&neighbour_state.pos) {
                continue;
            }

            let score_via_current_state = current_score + score_increase;

            if !scores.contains_key(&neighbour_state) {
                // This node has not been seen before
                scores.insert(neighbour_state, score_via_current_state);
                previous_states.insert(neighbour_state, HashSet::from([current_state]));
                queue.push(neighbour_state, Reverse(score_via_current_state));
            } else if let Some(previous_score) = scores.get_mut(&neighbour_state) {
                // Is a previously seen node
                match score_via_current_state.cmp(&previous_score) {
                    Ordering::Greater => (),
                    Ordering::Equal => {
                        // Add to list of predecessors
                        previous_states
                            .entry(neighbour_state)
                            .or_insert(HashSet::new())
                            .insert(current_state);
                    }
                    Ordering::Less => {
                        // Update with the better score
                        *previous_score = score_via_current_state;
                        queue.change_priority(&neighbour_state, Reverse(score_via_current_state));

                        // Replace all old predecessors with this new one
                        previous_states.insert(neighbour_state, HashSet::from([current_state]));
                    }
                }
            }
        }
    }

    let min_score: i32 = *scores
        .iter()
        .filter_map(|(state, score)| (state.pos == world.goal).then_some(score))
        .min()
        .unwrap_or(&-1);

    println!("Part 1. Min path score: {}", min_score);

    let min_goal_states: HashSet<State> = scores
        .iter()
        .filter_map(|(&state, &score)| {
            (score == min_score && state.pos == world.goal).then_some(state)
        })
        .collect();

    let mut visited_states = min_goal_states;
    let mut state_queue = VecDeque::new();

    for goal in visited_states.iter() {
        state_queue.push_back(*goal);
    }

    while let Some(state) = state_queue.pop_front() {
        if let Some(state_predecessors) = previous_states.get(&state) {
            for predecessor in state_predecessors {
                if !visited_states.contains(predecessor) {
                    visited_states.insert(*predecessor);
                    state_queue.push_back(*predecessor);
                }
            }
        }
    }

    let visited_nodes_count = visited_states.into_iter().map(|s| s.pos).unique().count();
    println!("Part 2. Count path nodes: {}", visited_nodes_count);
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    pos: IVec2,
    dir: IVec2,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "State -> pos: ({},{}) dir: ({},{})",
            self.pos.x, self.pos.y, self.dir.x, self.dir.y
        )
    }
}

impl State {
    fn forward(&self) -> Self {
        State {
            pos: self.pos + self.dir,
            dir: self.dir,
        }
    }

    fn turn_left(&self) -> Self {
        State {
            pos: self.pos,
            dir: self.dir.rotate(IVec2::NEG_Y),
        }
    }

    fn turn_right(&self) -> Self {
        State {
            pos: self.pos,
            dir: self.dir.rotate(IVec2::Y),
        }
    }
}

struct World {
    walls: HashSet<IVec2>,
    start: IVec2,
    goal: IVec2,
}

fn parse(input: &str) -> World {
    let mut walls = HashSet::new();
    let mut start = IVec2::new(-1, -1);
    let mut goal = IVec2::new(-1, -1);

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let vec = IVec2::new(x as i32, y as i32);

            match c {
                '#' => {
                    walls.insert(vec);
                    ()
                }
                'S' => start = vec,
                'E' => goal = vec,
                '.' => (),
                _ => panic!("Unexpected char in input!"),
            }
        }
    }

    World { walls, start, goal }
}
