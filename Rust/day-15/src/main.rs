use itertools::Itertools;

fn main() {
    let ingredients = parse();
    let properties_count = ingredients[0].len();

    let mut v = vec![0; properties_count];
    let best = optimize_score(100, ingredients.len(), &mut v, &ingredients, None);
    println!("Part 1. Total score = {}", best);

    let mut v = vec![0; properties_count];
    let best = optimize_score(100, ingredients.len(), &mut v, &ingredients, Some(500));
    println!("Part 2. Total score = {}", best);
}

fn optimize_score(
    teaspoons_left: usize,
    ingredients_left: usize,
    amounts: &mut [i64],
    properties: &Vec<Vec<i64>>,
    calories_limit: Option<i64>,
) -> i64 {
    if ingredients_left == 0 {
        return 0;
    }

    let mut score: i64 = 0;

    if ingredients_left > 1 {
        for teaspoons in 0..=teaspoons_left {
            let mut amounts = amounts.to_owned();

            for (i, a) in amounts.iter_mut().enumerate() {
                *a += (teaspoons as i64) * properties[ingredients_left - 1][i];
            }

            score = score.max(optimize_score(
                teaspoons_left - teaspoons,
                ingredients_left - 1,
                &mut amounts,
                properties,
                calories_limit,
            ));
        }
    } else {
        for (i, a) in amounts.iter_mut().enumerate() {
            *a += (teaspoons_left as i64) * properties[ingredients_left - 1][i];
        }

        let has_bad_amounts = amounts.iter().dropping_back(1).any(|a| a <= &0);

        if has_bad_amounts {
            score = 0;
        } else {
            score = amounts.iter().dropping_back(1).product();

            let calories_sum = amounts.last().unwrap();

            if let Some(ref limit) = calories_limit {
                if limit != calories_sum {
                    score = 0;
                }
            }
        }
    }

    score
}

fn parse() -> Vec<Vec<i64>> {
    let input = include_str!("../input.txt").trim();
    input
        .lines()
        .map(|line| {
            let (_, props): (_, &str) = line.split(": ").collect_tuple().unwrap();
            props
                .split(", ")
                .map(|p: &str| {
                    let (_, n): (_, &str) = p.split(' ').collect_tuple().unwrap();
                    n.parse::<i64>().unwrap()
                })
                .collect_vec()
        })
        .collect_vec()
}
