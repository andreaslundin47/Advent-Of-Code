use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt").trim();
    let data: CityData = parse(input);

    let min = optimize(&data, false);
    println!("Part 1. Min distance = {min}");

    let max = optimize(&data, true);
    println!("Part 2. Max distance = {max}");
}

fn optimize(data: &CityData, get_max: bool) -> usize {
    let distances = data
        .cities
        .iter()
        .permutations(data.cities.len())
        .map(|permutation| {
            permutation
                .windows(2)
                .map(|city_pair| {
                    data.distance(city_pair[0], city_pair[1])
                })
                .sum()
        });

    match get_max {
        true => distances.max(),
        false => distances.min(),
    }
    .unwrap()
}

struct CityData<'a> {
    distances: HashMap<(&'a str, &'a str), usize>,
    cities: Vec<&'a str>,
}

impl CityData<'_> {
    fn distance(&self, city1: &str, city2: &str) -> usize {
        if let Some(distance) = self.distances.get(&(city1, city2)) {
            return *distance;
        }

        if let Some(distance) = self.distances.get(&(city2, city1)) {
            return *distance;
        }

        panic!("Tried to look up a city pair that is not in the map!");
    }
}

fn parse(input: &str) -> CityData {
    let mut cities = Vec::new();
    let mut distances = HashMap::new();

    for line in input.lines() {
        let (places, distance) = line.split(" = ").collect_tuple().unwrap();
        let (left, right) = places.split(" to ").collect_tuple().unwrap();

        cities.push(left);
        cities.push(right);

        let distance: usize = distance.parse().unwrap();
        distances.insert((left, right), distance);
    }

    let cities = cities.into_iter().unique().collect_vec();

    CityData { distances, cities }
}