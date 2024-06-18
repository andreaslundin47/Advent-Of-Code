fn main() {
    let raw_input = include_str!("../input.txt").trim();

    let mut col_a: Vec<i32> = vec![];
    let mut col_b: Vec<i32> = vec![];
    let mut col_c: Vec<i32> = vec![];

    let mut count_valid = 0;

    for line in raw_input.lines() {
        let entry = Entry::from(line);

        col_a.push(entry.a);
        col_b.push(entry.b);
        col_c.push(entry.c);

        if entry.is_triangle() {
            count_valid += 1;
        }
    }

    println!("Part 1. Number of valid triangles = {count_valid}");

    let count_valid = col_a
        .chunks(3)
        .chain(col_b.chunks(3))
        .chain(col_c.chunks(3))
        .map(Entry::from)
        .filter(|e| e.is_triangle())
        .count();

    println!("Part 2. Number of valid triangles = {count_valid}");
}

#[derive(Debug)]
struct Entry {
    a: i32,
    b: i32,
    c: i32,
}

impl Entry {
    fn is_triangle(&self) -> bool {
        self.a + self.b > self.c && self.a + self.c > self.b && self.b + self.c > self.a
    }
}

impl From<&str> for Entry {
    fn from(value: &str) -> Self {
        let nums = value
            .split_whitespace()
            .map(|v| v.parse::<i32>().expect("Wanted an i32!?"))
            .collect::<Vec<i32>>();

        Entry {
            a: nums[0],
            b: nums[1],
            c: nums[2],
        }
    }
}

impl From<&[i32]> for Entry {
    fn from(values: &[i32]) -> Self {
        if values.len() != 3 {
            panic!("Need exactly three values");
        }
        Entry {
            a: values[0],
            b: values[1],
            c: values[2],
        }
    }
}
