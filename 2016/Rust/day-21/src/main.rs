mod parser;

fn main() {
    let input = include_str!("../input.txt").trim();
    let (_, operations) = parser::parse_operations(input).expect("A valid parse");

    let scrambled = scramble("abcdefgh", &operations);
    println!("Part 1. Scrambled string: {scrambled}");

    let unscrambled = reverse_scramble("fbgdceah", &operations);
    println!("Part 2. Unscrambled string: {unscrambled}");
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    SwapPositions { index_one: usize, index_two: usize },
    SwapLetters { letter_one: char, letter_two: char },
    RotateLeft { steps: usize },
    RotateRight { steps: usize },
    RotateOnLetter { letter: char },
    ReverseIndexSpan { index_one: usize, index_two: usize },
    MoveBetweenPositions { index_one: usize, index_two: usize },
}

fn swap_positions(mut current: Vec<char>, index_one: usize, index_two: usize) -> Vec<char> {
    let first = current[index_one];
    let second = current[index_two];
    current[index_one] = second;
    current[index_two] = first;
    current
}

fn swap_letters(current: Vec<char>, letter_one: char, letter_two: char) -> Vec<char> {
    let index_one = current.iter().position(|c| c == &letter_one).unwrap();
    let index_two = current.iter().position(|c| c == &letter_two).unwrap();
    swap_positions(current, index_one, index_two)
}

fn rotate_left(mut current: Vec<char>, steps: usize) -> Vec<char> {
    let steps = steps % current.len();
    let (a, b) = current.split_at(steps);
    current = [b, a].concat();
    current
}

fn rotate_right(current: Vec<char>, steps: usize) -> Vec<char> {
    let steps = (2 * current.len() - steps) % current.len();
    rotate_left(current, steps)
}

fn rotate_one_letter(mut current: Vec<char>, letter: char) -> Vec<char> {
    let index = current.iter().position(|c| c == &letter);
    if let Some(index) = index {
        let steps = if index >= 4 { index + 2 } else { index + 1 };

        current = rotate_right(current, steps);
    }
    current
}

fn reverse_rotate_one_letter(mut current: Vec<char>, letter: char) -> Vec<char> {
    let final_index = current.iter().position(|c| c == &letter).unwrap();
    for start_index in 0..current.len() {
        let steps = if start_index >= 4 {
            start_index + 2
        } else {
            start_index + 1
        };

        if (start_index + steps) % current.len() == final_index {
            current = rotate_left(current, steps);
            break;
        }
    }

    current
}

fn reverse_span(current: Vec<char>, index_one: usize, index_two: usize) -> Vec<char> {
    let (before, remaining) = current.split_at(index_one);
    let (middle, after) = remaining.split_at(index_two - index_one + 1);
    let reversed: Vec<char> = middle.iter().cloned().rev().collect();
    [before, &reversed, after].concat()
}

fn move_between_positions(mut current: Vec<char>, index_one: usize, index_two: usize) -> Vec<char> {
    let letter = current.remove(index_one);
    current.insert(index_two, letter);
    current
}

fn scramble(original: &str, operations: &[Operation]) -> String {
    let mut current: Vec<char> = original.chars().collect();

    for op in operations {
        current = match *op {
            Operation::SwapPositions {
                index_one,
                index_two,
            } => swap_positions(current, index_one, index_two),
            Operation::SwapLetters {
                letter_one,
                letter_two,
            } => swap_letters(current, letter_one, letter_two),
            Operation::RotateLeft { steps } => rotate_left(current, steps),
            Operation::RotateRight { steps } => rotate_right(current, steps),
            Operation::RotateOnLetter { letter } => rotate_one_letter(current, letter),
            Operation::ReverseIndexSpan {
                index_one,
                index_two,
            } => reverse_span(current, index_one, index_two),
            Operation::MoveBetweenPositions {
                index_one,
                index_two,
            } => move_between_positions(current, index_one, index_two),
        };
    }

    current.into_iter().collect()
}

fn reverse_scramble(original: &str, ops: &[Operation]) -> String {
    let mut current: Vec<char> = original.chars().collect();

    for op in ops.iter().rev() {
        current = match *op {
            Operation::SwapPositions {
                index_one,
                index_two,
            } => swap_positions(current, index_one, index_two),
            Operation::SwapLetters {
                letter_one,
                letter_two,
            } => swap_letters(current, letter_one, letter_two),
            Operation::RotateLeft { steps } => rotate_right(current, steps),
            Operation::RotateRight { steps } => rotate_left(current, steps),
            Operation::RotateOnLetter { letter } => reverse_rotate_one_letter(current, letter),
            Operation::ReverseIndexSpan {
                index_one,
                index_two,
            } => reverse_span(current, index_one, index_two),
            Operation::MoveBetweenPositions {
                index_one,
                index_two,
            } => move_between_positions(current, index_two, index_one),
        };
    }

    current.into_iter().collect()
}
