use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let grid = parse(input);

    let xmases = count_xmas(&grid);
    println!("Part 1. XMASes: {}", xmases);

    let cross_mases = count_cross_mas(&grid);
    println!("Part 2. Cross MASes: {}", cross_mases);
}

fn count_xmas(grid: &Grid) -> usize {
    let direction_offsets = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
    ];

    let mut count = 0;

    for (x, y) in (0..grid.width).cartesian_product(0..grid.height) {
        for (dx, dy) in direction_offsets.iter() {
            let word: String = (0..4)
                .filter_map(|i| grid.get(x + i * dx, y + i * dy))
                .collect();

            if word == "XMAS" {
                count += 1;
            }
        }
    }

    count
}

fn count_cross_mas(grid: &Grid) -> usize {
    let mut count = 0;

    for (x, y) in (1..grid.width - 1).cartesian_product(1..grid.height - 1) {
        if let Some('A') = grid.get(x, y) {
            let dec_diagonal: String = [(-1, -1), (1, 1)]
                .iter()
                .filter_map(|(dx, dy)| grid.get(x + dx, y + dy))
                .collect();

            let inc_diagonal: String = [(-1, 1), (1, -1)]
                .iter()
                .filter_map(|(dx, dy)| grid.get(x + dx, y + dy))
                .collect();

            if ["SM", "MS"].contains(&dec_diagonal.as_str())
                && ["SM", "MS"].contains(&inc_diagonal.as_str())
            {
                count += 1;
            }
        }
    }

    count
}

struct Grid {
    width: i32,
    height: i32,
    letters: Vec<Vec<char>>,
}

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        Some(*(self.letters.get(y as usize)?.get(x as usize)?))
    }
}

fn parse(input: &str) -> Grid {
    let letters: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = letters.len() as i32;
    let width = letters[0].len() as i32;

    Grid {
        width,
        height,
        letters,
    }
}
