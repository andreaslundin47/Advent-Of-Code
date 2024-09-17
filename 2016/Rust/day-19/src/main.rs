const PUZZLE_INPUT: usize = 3001330;

fn main() {
    first_game();
    second_game();
}

fn first_game() {
    let mut numbers: Vec<usize> = (1..=PUZZLE_INPUT).collect();

    while numbers.len() > 1 {
        let remaining = numbers.len();

        numbers = numbers
            .into_iter()
            .enumerate()
            .filter_map(|(index, num)| (index % 2 == 0).then_some(num))
            .collect();

        if remaining % 2 == 1 {
            if let Some(last_elf) = numbers.pop() {
                numbers.insert(0, last_elf);
            }
        }
    }

    let presenter = numbers[0];
    println!("Part 1. Present Elf = {}", presenter);
}

fn second_game() {
    let mut elves: Vec<usize> = (1..=PUZZLE_INPUT).collect();

    while elves.len() > 1 {
        let len = elves.len() / 2;

        let (cutter_elves, cuttable_elves) = elves.split_at(len);

        let uncut_elves: Vec<usize> = cuttable_elves
            .iter()
            .enumerate()
            .filter_map(|(i, elf)| {
                if elves.len() % 2 == 0 {
                    (i % 3 == 2).then_some(*elf)
                } else {
                    (i % 3 == 1).then_some(*elf)
                }
            })
            .collect();

        let number_of_elves_that_cut = cuttable_elves.len() - uncut_elves.len();
        let (active_cutter_elves, idle_cutter_elves) =
            cutter_elves.split_at(number_of_elves_that_cut);

        elves = [idle_cutter_elves, &uncut_elves, active_cutter_elves].concat();
    }

    let presenter = elves[0];
    println!("Part 2. Present Elf = {}", presenter);
}
