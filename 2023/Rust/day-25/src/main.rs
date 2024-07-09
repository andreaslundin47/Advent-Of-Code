use itertools::Itertools;
use rand::Rng;
use std::collections::HashMap;

fn main() {
    let adjacencies = include_str!("../input.txt")
        .trim()
        .lines()
        .map(|line| {
            let (start, dests) = line.split(": ").collect_tuple().unwrap();
            let dests = dests.split_whitespace().collect::<Vec<&str>>();
            (start, dests)
        })
        .collect::<HashMap<&str, Vec<&str>>>();

    let prod = kargers_algorithm(&adjacencies);
    println!("Part 1. Product = {prod}");
}

fn kargers_algorithm(adjacencies: &HashMap<&str, Vec<&str>>) -> usize {
    let mut rng = rand::thread_rng();

    let all_vertices: HashMap<&str, Vec<&str>> = adjacencies
        .values()
        .flatten()
        .chain(adjacencies.keys())
        .map(|vertex| (*vertex, vec![*vertex]))
        .collect();

    let all_edges: Vec<Vec<&str>> = adjacencies
        .iter()
        .flat_map(|(key, values)| values.iter().map(|val| vec![*key, *val]))
        .collect();

    loop {
        // Reset initial graph
        let mut vertices = all_vertices.clone();
        let mut edges = all_edges.clone();

        while vertices.len() > 2 {
            // Pick a random edge to remove
            let edge_index = rng.gen_range(0..edges.len());
            let edge_to_remove = edges[edge_index].clone();

            // Vertices to merge
            let (vertex_1, vertex_2) = (edge_to_remove[0], edge_to_remove[1]);

            // Move vertices from the second to the first, and remove the second
            let vertices_to_move = vertices
                .remove(&vertex_2)
                .expect("Source vertex should exist");

            vertices
                .get_mut(&vertex_1)
                .unwrap()
                .extend(vertices_to_move);

            edges.remove(edge_index);

            // Redirect any edges from the second vertex to the first, expanded vertex
            // Then filter out any loops back to self
            edges = edges
                .into_iter()
                .map(|edge| {
                    if edge[0] == vertex_2 {
                        vec![vertex_1, edge[1]]
                    } else if edge[1] == vertex_2 {
                        vec![edge[0], vertex_1]
                    } else {
                        edge
                    }
                })
                .filter(|edge| edge[0] != edge[1])
                .collect::<Vec<Vec<&str>>>();
        }

        if edges.len() == 3 {
            let prod = vertices.values().map(|v| v.len()).product::<usize>();
            return prod;
        }
    }
}
