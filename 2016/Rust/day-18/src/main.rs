use tile_row::{Tile, TileRow};

fn main() {
    let input = include_str!("../input.txt").trim();
    let init_row: Vec<Tile> = TileRow::parse(input);

    println!(
        "Part 1. # of safe tiles = {}",
        init_row.count_safes_in_n_rows(40)
    );
    println!(
        "Part 2. # of safe tiles = {}",
        init_row.count_safes_in_n_rows(400_000)
    );
}

mod tile_row {
    use itertools::Itertools;
    use std::iter;

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Tile {
        Safe,
        Trap,
    }

    pub trait TileRow {
        fn parse(input: &str) -> Self;
        fn count_safes_in_n_rows(&self, n_rows: usize) -> usize;
    }

    trait TileRowHelpers {
        fn evolve(&self) -> Self;
        fn count_safes(&self) -> usize;
    }

    impl TileRowHelpers for Vec<Tile> {
        fn count_safes(&self) -> usize {
            self.iter().filter(|tile| tile == &&Tile::Safe).count()
        }

        fn evolve(&self) -> Self {
            use Tile::*;

            let padding_left = iter::once(&Safe);
            let padding_right = iter::once(&Safe);

            let row_iter = padding_left.chain(self.iter()).chain(padding_right);

            row_iter
                .tuple_windows()
                .map(|tripple| match tripple {
                    (Trap, Trap, Safe) => Trap,
                    (Safe, Trap, Trap) => Trap,
                    (Trap, Safe, Safe) => Trap,
                    (Safe, Safe, Trap) => Trap,
                    _ => Safe,
                })
                .collect()
        }
    }

    impl TileRow for Vec<Tile> {
        fn count_safes_in_n_rows(&self, n_rows: usize) -> usize {
            (0..n_rows)
                .scan(self.clone(), |current_row, _| {
                    let count = current_row.count_safes();
                    *current_row = current_row.evolve();
                    Some(count)
                })
                .sum()
        }

        fn parse(input: &str) -> Self {
            input
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| match c {
                            '.' => Tile::Safe,
                            '^' => Tile::Trap,
                            _ => panic!("Bad input!"),
                        })
                        .collect()
                })
                .next()
                .expect("At least one line")
        }
    }
}
