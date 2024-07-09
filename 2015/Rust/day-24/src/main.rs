use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let packages: Vec<usize> = input
        .lines()
        .map(|line| line.parse::<usize>().unwrap())
        .collect();

    let e = entanglement(&packages, 3).expect("An answer!");
    println!("Part 1. {}", e);

    let e = entanglement(&packages, 4).expect("An answer!");
    println!("Part 1. {}", e);
}

fn entanglement(packages: &Vec<usize>, splits: usize) -> Option<usize> {
    if splits == 1 {
        return Some(packages.iter().product());
    }

    let group_sum: usize = packages.iter().sum::<usize>() / splits;

    let min_group_size: usize = 1 + packages
        .iter()
        .rev()
        .scan(0, |acc, &p| {
            *acc += p;
            Some(*acc)
        })
        .position(|s| s >= group_sum)
        .unwrap_or(0);

    let max_group_size: usize = packages.len() / splits;

    let mut entanglements = vec![];

    for group_size in min_group_size..max_group_size {
        for combination in packages.iter().combinations(group_size) {
            let combination_sum: usize = combination.iter().copied().sum::<usize>();

            if combination_sum != group_sum {
                continue;
            }

            let remains: Vec<usize> = packages
                .clone()
                .into_iter()
                .filter(|p| !combination.contains(&p))
                .collect();

            if entanglement(&remains, splits - 1).is_some() {
                let entanglement: usize = combination.into_iter().product();
                entanglements.push(entanglement);
            }
        }

        if !entanglements.is_empty() {
            return Some(entanglements.into_iter().min().unwrap());
        }
    }

    None
}
