fn main() {
    let input = include_str!("../input.txt").trim();
    let notes = parse_notes(&input);

    let sum: usize = notes.iter().map(|note| note.mirror_number_one()).sum();
    println!("Part 1. Sum = {sum}");

    let sum: usize = notes.iter().map(|note| note.mirror_number_two()).sum();
    println!("Part 2. Sum = {sum}");
}
// -----------------------------------------------------------------------

fn parse_notes(input: &str) -> Vec<Note> {
    input
        .split("\n\n")
        .map(|block| {
            let grid: Vec<Vec<char>> = block
                .lines()
                .map(|line| line.chars().collect())
                .collect::<Vec<Vec<char>>>();

            let height = grid.len();
            let width = grid.first().unwrap().len();

            Note {
                width,
                height,
                grid,
            }
        })
        .collect()
}
// -----------------------------------------------------------------------

#[derive(Debug)]
struct Note {
    width: usize,
    height: usize,
    grid: Vec<Vec<char>>,
}

impl Note {
    fn mirror_number_one(&self) -> usize {
        // If any find a rows split that is a valid reflection
        let rows: Option<usize> = (1..self.height).find(|&row_index| {
            let (upper, lower) = self.grid.split_at(row_index);
            let reflection = upper
                .iter()
                .rev()
                .zip(lower.iter())
                .all(|(row_a, row_b)| row_a.iter().zip(row_b.iter()).all(|(a, b)| a == b));

            reflection
        });

        // If any find a columns split that is a valid reflection
        let columns: Option<usize> = (1..self.width).find(|&col_index| {
            self.grid.iter().all(|row| {
                let (left, right) = row.split_at(col_index);
                left.iter().rev().zip(right.iter()).all(|(a, b)| a == b)
            })
        });

        if let Some(row_count) = rows {
            return 100 * row_count;
        }

        if let Some(col_count) = columns {
            return col_count;
        }

        panic!("Should have found a valid row or columns by this point");
    }

    fn mirror_number_two(&self) -> usize {
        // If any, find a rows split with exactly one difference
        let rows: Option<usize> = (1..self.height).find(|&row_index| {
            let (upper, lower) = self.grid.split_at(row_index);
            let differences = upper
                .iter()
                .rev()
                .zip(lower.iter())
                .map(|(row_a, row_b)| {
                    row_a
                        .iter()
                        .zip(row_b.iter())
                        .filter(|(a, b)| a != b)
                        .count()
                })
                .sum::<usize>();

            differences == 1
        });

        // If any, find a columns split with exactly one difference
        let columns: Option<usize> = (1..self.width).find(|&col_index| {
            let differences = self
                .grid
                .iter()
                .map(|row| {
                    let (left, right) = row.split_at(col_index);
                    left.iter()
                        .rev()
                        .zip(right.iter())
                        .filter(|(a, b)| a != b)
                        .count()
                })
                .sum::<usize>();

            differences == 1
        });

        if let Some(row_count) = rows {
            return 100 * row_count;
        }

        if let Some(col_count) = columns {
            return col_count;
        }

        panic!("Should have found a valid row or columns by this point");
    }
}
// -----------------------------------------------------------------------
