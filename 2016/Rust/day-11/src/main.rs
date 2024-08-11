use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::{
    cmp::Reverse,
    collections::HashSet,
};

fn main() {
    let top_floor = 4;

    let init_state = State {
        floor: 1,
        locations: vec![
            GeneratorChipLocations::new(1, 1),
            GeneratorChipLocations::new(1, 1),
            GeneratorChipLocations::new(1, 1),
            GeneratorChipLocations::new(1, 2),
            GeneratorChipLocations::new(1, 2),
        ],
    };

    let steps = find_minimum_steps_needed(init_state, top_floor);
    println!("Part 1. Steps = {}", steps.unwrap());

    let init_state = State {
        floor: 1,
        locations: vec![
            GeneratorChipLocations::new(1, 1),
            GeneratorChipLocations::new(1, 1),
            GeneratorChipLocations::new(1, 1),
            GeneratorChipLocations::new(1, 1),
            GeneratorChipLocations::new(1, 1),
            GeneratorChipLocations::new(1, 2),
            GeneratorChipLocations::new(1, 2),
        ],
    };

    let steps = find_minimum_steps_needed(init_state, top_floor);
    println!("Part 2. Steps = {}", steps.unwrap());
}

fn find_minimum_steps_needed(init_state: State, top_floor: u8) -> Option<i32> {
    let mut seen = HashSet::new();
    let mut queue = PriorityQueue::new();

    seen.insert(init_state.clone());
    queue.push(init_state, Reverse(0));

    while let Some((state, Reverse(steps))) = queue.pop() {
        if state.all_home(top_floor) {
            return Some(steps);
        }

        for neigh in state.neighbours(top_floor) {
            if !seen.contains(&neigh) {
                seen.insert(neigh.clone());
                queue.push(neigh, Reverse(steps + 1));
            }
        }
    }

    None
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct GeneratorChipLocations {
    generator: u8,
    chip: u8,
}

impl GeneratorChipLocations {
    fn new(generator: u8, chip: u8) -> Self {
        GeneratorChipLocations { generator, chip }
    }
}

#[derive(Debug, Hash, Clone, Eq, PartialEq)]
struct State {
    floor: u8,
    locations: Vec<GeneratorChipLocations>,
}

impl State {
    fn all_home(&self, top_floor: u8) -> bool {
        self.locations
            .iter()
            .all(|pair| pair.generator == top_floor && pair.chip == top_floor)
    }

    fn neighbours(&self, top_floor: u8) -> Vec<State> {
        let mut out = vec![];

        let exists_items_below_current_floor = self
            .locations
            .iter()
            .any(|pair| pair.generator < self.floor || pair.chip < self.floor);

        if self.floor > 1 && exists_items_below_current_floor {
            out.append(&mut self.moves_between_floors(self.floor, self.floor - 1));
        }

        if self.floor < top_floor {
            out.append(&mut self.moves_between_floors(self.floor, self.floor + 1));
        }

        out
    }

    fn moves_between_floors(&self, source_floor: u8, target_floor: u8) -> Vec<State> {
        let mut out = vec![];

        let generators: Vec<u8> = self.locations.iter().map(|pair| pair.generator).collect();
        let chips: Vec<u8> = self.locations.iter().map(|pair| pair.chip).collect();

        let source_generators: Vec<u8> = generators
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| (ch == &source_floor).then_some(i as u8))
            .collect();

        let source_chips: Vec<u8> = chips
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| (ch == &source_floor).then_some(i as u8))
            .collect();

        let target_generators: Vec<u8> = generators
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| (ch == &target_floor).then_some(i as u8))
            .collect();

        let target_chips: Vec<u8> = chips
            .iter()
            .enumerate()
            .filter_map(|(i, ch)| (ch == &target_floor).then_some(i as u8))
            .collect();

        let unshielded_target_chips: Vec<u8> = target_chips
            .into_iter()
            .filter(|ch| !target_generators.contains(ch))
            .collect();

        // Move a single generator
        for ge in source_generators.iter() {
            if unshielded_target_chips.is_empty()
                || (unshielded_target_chips.len() == 1 && unshielded_target_chips.contains(ge))
            {
                let mut new = self.clone();
                new.floor = target_floor;
                new.locations[*ge as usize].generator = target_floor;
                new.locations.sort();
                out.push(new);
            }
        }

        // Move a single chip
        for chip in source_chips.iter() {
            if target_generators.is_empty() || target_generators.contains(chip) {
                let mut new = self.clone();
                new.floor = target_floor;
                new.locations[*chip as usize].chip = target_floor;
                new.locations.sort();
                out.push(new);
            }
        }

        // Move two chips
        for chips in source_chips.iter().combinations(2) {
            if target_generators.is_empty()
                || target_generators.contains(chips[0]) && target_generators.contains(chips[1])
            {
                let mut new = self.clone();
                new.floor = target_floor;
                new.locations[*chips[0] as usize].chip = target_floor;
                new.locations[*chips[1] as usize].chip = target_floor;
                new.locations.sort();
                out.push(new);
            }
        }

        // Move a chip and a generator
        for (generator, chip) in source_generators
            .iter()
            .cartesian_product(source_chips.iter())
        {
            let can_move_matching = *generator == *chip && unshielded_target_chips.is_empty();

            let can_move_non_matching = target_generators.contains(chip)
                && (unshielded_target_chips.is_empty()
                    || unshielded_target_chips.len() == 1
                        && unshielded_target_chips.contains(generator));

            if can_move_matching || can_move_non_matching {
                let mut new = self.clone();
                new.floor = target_floor;
                new.locations[*generator as usize].generator = target_floor;
                new.locations[*chip as usize].chip = target_floor;
                new.locations.sort();
                out.push(new);
            }
        }

        // Move two generators
        for generators in source_generators.iter().combinations(2) {
            if unshielded_target_chips.is_empty()
                || unshielded_target_chips.iter().all(|ch| generators.contains(&ch))
            {
                let mut new = self.clone();
                new.floor = target_floor;
                new.locations[*generators[0] as usize].generator = target_floor;
                new.locations[*generators[1] as usize].generator = target_floor;
                new.locations.sort();
                out.push(new);
            }
        }

        out
    }
}
