use std::ops::Add;
use std::cmp::max;

use nom::{  IResult, 
            bytes::complete::{tag, is_not}, 
            character::complete::{alphanumeric1, alpha1}, 
            sequence::{delimited, preceded}, 
            multi::separated_list1, 
            Finish
        };

// -----------------------------------------------------------------------------------------

fn main() {
    let raw_input = include_str!("../input.txt");
    let games = parse_games(raw_input);
    part_one(&games);
    part_two(&games);
}

fn part_one(games: &Vec<Game>) {
    let limit = ColorCount(12, 13, 14);

    let id_sum: u32 = games
        .iter()
        .filter(|game| game.is_valid(&limit))
        .map(|game| game.id )
        .sum();
    
    println!("Part 1. Id sum = {}", id_sum);
}

fn part_two(games: &Vec<Game>) {

    let power_sum: u32 = games
        .iter()
        .map(|game| game.minimum_power())
        .sum();
   
    println!("Part 2. Id sum = {}", power_sum);
}
// -----------------------------------------------------------------------------------------

#[derive(Debug, Copy, Clone, Default)]
struct ColorCount(u32, u32, u32);

impl Add for ColorCount {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ColorCount(self.0 + other.0, self.1 + other.1 , self.2 + other.2)
    }
}

impl ColorCount {
    fn power(&self) -> u32 {
        self.0 * self.1 * self.2
    }

    fn elementwise_max(&self, other: ColorCount) -> ColorCount {
        ColorCount( max(self.0, other.0), max(self.1, other.1), max(self.2, other.2) )
    }
}
// Store a Set of shown cubes  -------------------------------------------------------------

#[derive(Debug)]
struct CubeSet {
    set: Vec<ColorCount>,
}

impl CubeSet {
    fn sum(&self) -> ColorCount {
        self.set.iter().fold(ColorCount::default(), |acc, &x| acc + x)
    }

    fn is_valid(&self, limit: &ColorCount) -> bool {
        let sum = self.sum();
        sum.0 <= limit.0 && sum.1 <= limit.1 && sum.2 <= limit.2
    }
}

// Stores a Game  --------------------------------------------------------------------------

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

impl Game {
    fn is_valid(&self, limit: &ColorCount) -> bool {
        self.sets.iter().all(|set| set.is_valid(limit) )
    }

    fn minimum_power(&self) -> u32 {
        let min_sets: Vec<ColorCount> = self.sets.iter().map(|set| set.sum()).collect();
        let min_comb = min_sets.iter().fold(ColorCount::default(), |acc,&x| acc.elementwise_max(x));
        min_comb.power()
    }
}
// Parsing  -------------------------------------------------------------------------------

fn parse_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| parse_game(line).finish().unwrap().1 )
        .collect()
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, id) = delimited(tag("Game "), is_not(":"), tag(":"))(input)?;
    let (input, sets) = separated_list1(tag(";"), parse_set)(input)?;
    let id: u32 = id.parse().expect("Expected a game id");

    Ok( (input, Game { id, sets }) )
}

fn parse_set(input: &str) -> IResult<&str, CubeSet> {
    let (input, set) = separated_list1(tag(","), parse_cube_count)(input)?;
    Ok( (input, CubeSet { set }) )
}

fn parse_cube_count(input: &str) -> IResult<&str, ColorCount> {
    let (input, count) = preceded(tag(" "), alphanumeric1)(input)?;
    let (input, color) = preceded(tag(" "), alpha1)(input)?;

    let count: u32 = count.parse().expect("Expected a number of cubes");

    let cube_count = match color {
        "red" => ColorCount(count, 0, 0),
        "green" => ColorCount(0, count, 0),
        "blue" => ColorCount(0, 0, count),
        _ => panic!("Thsi is not a color!"),
    };

    Ok( (input, cube_count) )
}
// -----------------------------------------------------------------------------------------