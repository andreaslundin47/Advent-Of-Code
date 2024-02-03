use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt").trim();

    let enhance_rules: HashMap<String, String> = input
        .lines()
        .flat_map(|line| {
            let (source, target) = line.split_once(" => ").expect("Valid input line");

            Square::new(source)
                .variants()
                .iter()
                .map(|variant| (variant.to_string(), target.to_string()))
                .collect::<Vec<_>>()
        })
        .collect();

    let initial = ".#./..#/###".to_string();

    let mut image = initial;

    for _ in 0..5 {
        image = enhance(&image, &enhance_rules);
    }

    let count = image.chars().filter(|c| c == &'#').count();
    println!("Part 1. Number of active cells: {count}");

    for _ in 0..13 {
        image = enhance(&image, &enhance_rules);
    }

    let count = image.chars().filter(|c| c == &'#').count();
    println!("Part 2. Number of active cells: {count}");
}

fn enhance(s: &str, rules: &HashMap<String, String>) -> String {
    let blocks = Square::new(s).subdivide();

    let translated_blocks: Vec<Vec<Square>> = blocks
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|block| {
                    let new_block = rules.get(&block).expect("Block translation");
                    Square::new(new_block)
                })
                .collect()
        })
        .collect();

    let b = translated_blocks
        .iter()
        .map(|line| join_squares_horizontal(line))
        .collect::<Vec<String>>()
        .join("/");

    b
}

fn join_squares_horizontal(squares: &Vec<Square>) -> String {
    let mut rows: Vec<String> = Vec::new();
    let dim = squares[0].dim;
    for i in 0..dim {
        let a = squares
            .iter()
            .map(|sq| sq.grid[i].iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("");
        rows.push(a);
    }
    rows.join("/")
}

#[derive(Debug, Clone)]
struct Square {
    grid: Vec<Vec<char>>,
    dim: usize,
}

impl Square {
    fn new(s: &str) -> Self {
        let grid: Vec<Vec<char>> = s.split("/").map(|row| row.chars().collect()).collect();
        let dim = grid.len();
        Square { grid, dim }
    }

    fn mirror(&self) -> Square {
        let grid = self.grid.iter().rev().cloned().collect();
        Square {
            grid,
            dim: self.dim,
        }
    }

    fn rotate(&self) -> Square {
        let mut iters: Vec<_> = self.grid.iter().rev().map(|row| row.iter()).collect();

        let grid: Vec<Vec<char>> = std::iter::from_fn(move || {
            let mut new_row = vec![];

            for it in iters.iter_mut() {
                if let Some(elem) = it.next() {
                    new_row.push(*elem);
                } else {
                    return None;
                }
            }
            Some(new_row)
        })
        .collect();

        let dim = grid.len();
        Square { grid, dim }
    }

    fn to_string(&self) -> String {
        let rows: Vec<String> = self.grid.iter().map(|row| row.iter().collect()).collect();
        rows.join("/")
    }

    fn variants(&self) -> Vec<Square> {
        let mut variants = vec![];
        let mut sq = self.clone();
        for _ in 0..4 {
            variants.push(sq.clone());
            sq = sq.rotate();
        }
        sq = sq.mirror();
        for _ in 0..4 {
            variants.push(sq.clone());
            sq = sq.rotate();
        }

        variants
    }

    fn subdivide(&self) -> Vec<Vec<String>> {
        let block_size: usize = if self.dim % 2 == 0 { 2 } else { 3 };
        let mut subdivisions = Vec::new();

        for i in (0..self.grid.len()).step_by(block_size) {
            let mut sub_row = Vec::new();

            for j in (0..self.grid.len()).step_by(block_size) {
                let mut sub_block = Vec::new();

                for row in &self.grid[i..i + block_size] {
                    sub_block.push(row[j..j + block_size].to_vec());
                }

                let sub_square = Square {
                    grid: sub_block,
                    dim: block_size,
                };

                sub_row.push(sub_square.to_string());
            }

            subdivisions.push(sub_row);
        }

        subdivisions
    }
}
