use itertools::Itertools;

const OFFSETS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    let input = include_str!("../input.txt").trim();
    let start: Grid = parse(input);

    let mut grid = start.clone();
    for _ in 0..100 {
        grid.step();
    }
    println!("Part 1. Lights on = {}", grid.lights_count());

    let mut grid = start;
    for _ in 0..100 {
        grid.step();
        grid.light_corners();
    }
    println!("Part 2. Lights on = {}", grid.lights_count());
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Grid {
    fn _print(&self) {
        println!("----------------------------------");
        for row in &self.grid {
            println!(
                "{}",
                row.iter().map(|c| if *c { '#' } else { '.' }).join("")
            );
        }
    }

    fn is_valid(&self, pos: (i32, i32)) -> bool {
        let (x, y) = pos;
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }

    fn get(&self, pos: (i32, i32)) -> bool {
        let (x, y) = pos;
        self.grid[y as usize][x as usize]
    }

    fn step(&mut self) {
        let mut next_grid = Vec::with_capacity(self.height);

        for y in 0..self.height {
            let mut next_row = Vec::with_capacity(self.width);

            for x in 0..self.width {
                let living_neighbours = OFFSETS
                    .iter()
                    .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
                    .filter(|p| self.is_valid(*p))
                    .filter(|p| self.get(*p))
                    .count();

                let is_alive = self.grid[y][x];

                let next_value = matches!(
                    (is_alive, living_neighbours),
                    (true, 2) | (true, 3) | (false, 3)
                );
                next_row.push(next_value);
            }

            next_grid.push(next_row);
        }

        self.grid = next_grid;
    }

    fn light_corners(&mut self) {
        self.grid[0][0] = true;
        self.grid[0][self.width - 1] = true;
        self.grid[self.height - 1][0] = true;
        self.grid[self.height - 1][self.width - 1] = true;
    }

    fn lights_count(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|v| **v).count())
            .sum()
    }
}

fn parse(input: &str) -> Grid {
    let grid: Vec<Vec<bool>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => true,
                    '.' => false,
                    _ => panic!("Bad input!"),
                })
                .collect_vec()
        })
        .collect_vec();

    let width = grid[0].len();
    let height = grid.len();

    Grid {
        grid,
        width,
        height,
    }
}
