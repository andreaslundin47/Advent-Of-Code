use std::collections::HashMap;

fn main() {
    let sue_known_properties = parse();

    let sample_properties = HashMap::from([
            (String::from("children"), 3),
            (String::from("cats"), 7),
            (String::from("samoyeds"), 2),
            (String::from("pomeranians"), 3),
            (String::from("akitas"), 0),
            (String::from("vizslas"), 0),
            (String::from("goldfish"), 5),
            (String::from("trees"), 3),
            (String::from("cars"), 2),
            (String::from("perfumes"), 1)
    ]);

    for (index, properties) in sue_known_properties.iter() {
        if validate_one(properties, &sample_properties) {
            println!("Part 1. Sue number {index} and {properties:?}");
            break;
        }
    }

    for (index, properties) in sue_known_properties.iter() {
        if validate_two(properties, &sample_properties) {
            println!("Part 2. Sue number {index} and {properties:?}");
            break;
        }
    }
}


fn validate_one(sue_properties: &Vec<(String, usize)>, sample_properties: &HashMap<String,usize>) -> bool {
    for (property, value) in sue_properties.iter() {
        let sample_value = sample_properties[property];
        if &sample_value != value {
            return false;
        }
    }
    true
}

fn validate_two(sue_properties: &Vec<(String, usize)>, sample_properties: &HashMap<String,usize>) -> bool {
    for (property, value) in sue_properties.iter() {
        let sample_value = sample_properties[property];

        if matches!(property.as_str(), "cats" | "trees") {
            if value <= &sample_value {
                return false;
            }
        }

        else if matches!(property.as_str(), "pomeranians" | "goldfish") { 
            if value >= &sample_value {
                return false;
            }
        }

        else if value != &sample_value {
            return false;
        }
    }
    true
}



fn parse() -> HashMap<usize, Vec<(String, usize)>> {
    let input = include_str!("../input.txt").trim();

    input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (_, properties)  = line.split_once(": ").unwrap();

            ( i+1,
                properties
                    .split(", ")
                    .map(|key_value| {
                        let (k, v) = key_value.split_once(": ").unwrap();
                        (k.to_string(), v.parse::<usize>().unwrap())
                    })
                    .collect()
            )
        })
        .collect()
}