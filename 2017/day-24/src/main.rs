use itertools::Itertools;
use std::{
    cmp::max,
    collections::HashSet,
};

fn main() {
    let input = include_str!("../input.txt").trim();

    let components: Vec<Component> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once('/').unwrap();
            Component {
                connector_1: a.parse::<usize>().unwrap(), 
                connector_2: b.parse::<usize>().unwrap(),
            }
        })
        .collect_vec();

    let graph = Graph { components };

    let max_strength = graph.max_strength();
    println!("Part 1. bridge strength = {max_strength}");

    let longest_strength = graph.longest_strength();
    println!("Part 2. bridge strength = {longest_strength}");
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Component {
    connector_1: usize,
    connector_2: usize,
}

impl Component {
    fn next_connector(&self, connector: usize) -> Option<usize> {
        if connector == self.connector_1 {
            Some(self.connector_2)
        }
        else if connector == self.connector_2 {
            Some(self.connector_1)
        }
        else {
            None
        }
    }

    fn strength(&self) -> usize {
        self.connector_1 + self.connector_2
    }
}

struct Graph {
    components: Vec<Component>,
}

impl Graph {
    fn max_strength(&self) -> usize {
        let mut used = HashSet::new();
        self.dfs(0, 0, &mut used)
    }

    fn longest_strength(&self) -> usize {
        let mut used = HashSet::new();
        let (_, strength) = self.dfs2(0, 0, &mut used);
        strength
    }

    fn dfs<'a>(&'a self, exposed_connector: usize, current_strength: usize, used: &mut HashSet<&'a Component>) -> usize {
        let mut best_strength = current_strength;

        for comp in self.components.iter() {
            if used.contains(&comp) {
                continue;
            }

            if let Some(next_connector) = comp.next_connector(exposed_connector) {
                let next_strength = current_strength + comp.strength();
                used.insert(comp);
                let strength = self.dfs(next_connector, next_strength, used);
                used.remove(&comp);
                best_strength = max(best_strength, strength);
            }
        }

        best_strength
    }


    fn dfs2<'a>(&'a self, exposed_connector: usize, current_strength: usize, used: &mut HashSet<&'a Component>) -> (usize, usize) {
        let mut best_strength = current_strength;
        let mut best_length = used.len();

        for comp in self.components.iter() {
            if used.contains(&comp) {
                continue;
            }

            if let Some(next_connector) = comp.next_connector(exposed_connector) {
                let next_strength = current_strength + comp.strength();

                used.insert(comp);
                let (length, strength) = self.dfs2(next_connector, next_strength, used);
                used.remove(&comp);

                if length > best_length {
                    best_length = length;
                    best_strength = strength;
                }
                else if length == best_length {
                    best_strength = max(best_strength, strength);
                }

            }
        }

        (best_length, best_strength)
    }
}
