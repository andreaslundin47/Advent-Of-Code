use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Graph = HashMap<String, Vec<String>>;

fn main() {
    let input = include_str!("../input.txt").trim();
    let graph: Graph = parse(input);

    part_one(&graph);
    part_two(&graph);
}

fn part_one(nodes: &Graph) {
    let mut triangles = HashSet::new();

    for (node_a, neighbours_a) in nodes.iter() {
        for node_b in neighbours_a.iter() {
            if let Some(neighbours_b) = nodes.get(node_b) {
                for node_c in neighbours_b.iter() {
                    if let Some(neighbours_c) = nodes.get(node_c) {
                        if neighbours_c.contains(node_a) {
                            // We have a triangle!
                            let mut trio = vec![node_a, node_b, node_c];
                            trio.sort();

                            triangles.insert(trio);
                        }
                    }
                }
            }
        }
    }

    let t_starters = triangles
        .iter()
        .filter(|tri| tri.iter().any(|node| node.starts_with("t")))
        .count();

    println!("Part 1. {}", t_starters);
}

fn part_two(graph: &Graph) {
    let cliques = find_all_maximal_cliques(graph);

    if let Some(largest) = cliques.iter().sorted_by_key(|cli| cli.len()).last() {
        let password = largest.iter().join(",");
        println!("Part 2. {}", password);
    }
}

fn find_all_maximal_cliques(graph: &Graph) -> HashSet<Vec<String>> {
    let mut all_cliques = HashSet::new();

    let r = HashSet::new();
    let mut p = graph.keys().cloned().collect();
    let mut x = HashSet::new();

    bors_kerbosch(&r, &mut p, &mut x, &mut all_cliques, graph);

    all_cliques
}

fn bors_kerbosch<'a>(
    r: &HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    found_maximal_cliques: &mut HashSet<Vec<String>>,
    graph: &'a Graph,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            found_maximal_cliques.insert(r.iter().cloned().sorted().collect());
        }
        return;
    }

    while !p.is_empty() {
        if let Some(v) = p.iter().next().cloned() {
            if let Some(v_neighbours) = graph.get(&v) {
                let v_neighbours = v_neighbours.iter().cloned().collect::<HashSet<String>>();

                let rr = r.union(&HashSet::from([v.clone()])).cloned().collect();
                let mut pp = p.intersection(&v_neighbours).cloned().collect();
                let mut xx = p.intersection(&v_neighbours).cloned().collect();

                bors_kerbosch(&rr, &mut pp, &mut xx, found_maximal_cliques, graph);
                p.remove(&v);
                x.insert(v);
            }
        }
    }
}

fn parse(input: &str) -> Graph {
    let mut nodes = Graph::new();

    for line in input.lines() {
        let (n1, n2) = line.split_once('-').unwrap();

        nodes
            .entry(n1.to_string())
            .or_insert(vec![])
            .push(n2.to_string());

        nodes
            .entry(n2.to_string())
            .or_insert(vec![])
            .push(n1.to_string());
    }

    nodes
}
