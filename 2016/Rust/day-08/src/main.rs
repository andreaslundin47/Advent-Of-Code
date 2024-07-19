fn main() {
    let input = include_str!("../input.txt").trim();
    let updates: Vec<Update> = parse(input).expect("A valid parse");

    let mut screen = Screen::new();

    for update in &updates {
        screen = match *update {
            Update::DrawRect { dx, dy } => screen.rect(dx, dy),
            Update::RotateRow { y, shift } => screen.row(y, shift),
            Update::RotateCol { x, shift } => screen.col(x, shift),
        }
    }

    println!("Part 1. Lit pixels = {}", screen.count_lit());
    println!("Part 2. Screen output:");
    screen.draw();
}

#[derive(Debug, Clone)]
struct Screen {
    width: usize,
    height: usize,
    pixels: Vec<Vec<bool>>,
}

impl Screen {
    fn new() -> Screen {
        Screen {
            width: 50,
            height: 6,
            pixels: vec![vec![false; 50]; 6],
        }
    }

    fn count_lit(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|p| **p).count())
            .sum()
    }

    fn draw(&self) {
        let mut output = String::new();
        for row in &self.pixels {
            for &pixel in row {
                output.push_str(if pixel { "##" } else { "  " });
            }
            output.push('\n');
        }
        println!("{}", output);
    }

    fn rect(&self, dx: usize, dy: usize) -> Screen {
        let mut screen = self.clone();
        for y in 0..dy {
            for x in 0..dx {
                screen.pixels[y][x] = true;
            }
        }
        screen
    }

    fn row(&self, y: usize, shift: usize) -> Screen {
        let mut screen = self.clone();
        for x in 0..self.width {
            screen.pixels[y][(x + shift) % self.width] = self.pixels[y][x];
        }
        screen
    }

    fn col(&self, x: usize, shift: usize) -> Screen {
        let mut screen = self.clone();
        for y in 0..self.height {
            screen.pixels[(y + shift) % self.height][x] = self.pixels[y][x];
        }
        screen
    }
}

#[derive(Debug)]
enum Update {
    DrawRect { dx: usize, dy: usize },
    RotateRow { y: usize, shift: usize },
    RotateCol { x: usize, shift: usize },
}

fn parse(input: &str) -> Option<Vec<Update>> {
    input
        .lines()
        .map(|line| {
            if line.contains("rect") {
                let (cols, rows) = line.strip_prefix("rect ")?.split_once('x')?;
                Some(Update::DrawRect {
                    dx: cols.parse::<usize>().ok()?,
                    dy: rows.parse::<usize>().ok()?,
                })
            } else if line.contains("row") {
                let (row, shift) = line.strip_prefix("rotate row y=")?.split_once(" by ")?;
                Some(Update::RotateRow {
                    y: row.parse::<usize>().ok()?,
                    shift: shift.parse::<usize>().ok()?,
                })
            } else if line.contains("column") {
                let (col, shift) = line.strip_prefix("rotate column x=")?.split_once(" by ")?;
                Some(Update::RotateCol {
                    x: col.parse::<usize>().ok()?,
                    shift: shift.parse::<usize>().ok()?,
                })
            } else {
                None
            }
        })
        .collect()
}
