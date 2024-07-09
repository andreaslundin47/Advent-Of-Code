use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt").trim();
    let (replacements, molecule) = parse(input);

    let products = generate_neighbours(&molecule, &replacements);
    println!("Part 1. Products: {}", products.len());

    let steps = min_steps_to_molecule(&molecule);
    println!("Part 2. Steps = {steps}");
}

fn generate_neighbours(molecule: &str, replacements: &[Replacement]) -> Vec<String> {
    let mut products: HashSet<String> = HashSet::new();

    for Replacement { input, output } in replacements.iter() {
        for (i, _) in molecule.match_indices(input) {
            let left = molecule[0..i].to_owned();
            let right = molecule[i + input.len()..].to_owned();

            let new_molecule = format!("{left}{output}{right}");
            products.insert(new_molecule);
        }
    }

    products.into_iter().collect()
}

fn min_steps_to_molecule(molecule: &str) -> usize {
    let count_elements = molecule.chars().filter(|c| c.is_ascii_uppercase()).count();
    let count_ar = molecule.matches("Ar").count();
    let count_y = molecule.matches('Y').count();
    let count_rn = molecule.matches("Rn").count();

    count_elements - count_ar - count_rn - 2 * count_y - 1
}

#[derive(Debug)]
struct Replacement {
    input: String,
    output: String,
}

fn parse(input: &str) -> (Vec<Replacement>, String) {
    let (list, molecule) = input.split_once("\n\n").unwrap();

    let reactions = list
        .lines()
        .map(|line| {
            let (source, product) = line.split_once(" => ").unwrap();

            Replacement {
                input: source.to_string(),
                output: product.to_string(),
            }
        })
        .collect();

    (reactions, molecule.to_owned())
}
