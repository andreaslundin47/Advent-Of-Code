use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let connections = parse(input);

    let paths = count_paths(&connections, "you", "out");
    println!("Part 1. {paths}");

    // Graph has no cycles, so either dac -> fft has zero paths, or fft -> dac has zero paths.
    // We take the order with a non-zero number of paths.

    let (first_stop, second_stop) = match count_paths(&connections, "dac", "fft") {
        0 => ("fft", "dac"),
        _ => ("dac", "fft"),
    };

    let paths = count_paths(&connections, "svr", first_stop)
        * count_paths(&connections, first_stop, second_stop)
        * count_paths(&connections, second_stop, "out");
    println!("Part 2. {paths}");
}

fn count_paths(connections: &HashMap<&str, Vec<&str>>, start: &str, goal: &str) -> usize {
    let mut memo = HashMap::new();
    return count_helper(connections, start, goal, &mut memo);

    fn count_helper<'a>(
        connections: &HashMap<&'a str, Vec<&'a str>>,
        current: &'a str,
        goal: &'a str,
        memo: &mut HashMap<&'a str, usize>,
    ) -> usize {
        if let Some(&count) = memo.get(&current) {
            return count;
        }

        if current == goal {
            return 1;
        }

        if let Some(children) = connections.get(&current) {
            let paths = children
                .iter()
                .map(|ch| count_helper(connections, ch, goal, memo))
                .sum::<usize>();

            memo.insert(current, paths);
            paths
        } else {
            memo.insert(current, 0);
            0
        }
    }
}

fn parse(s: &str) -> HashMap<&str, Vec<&str>> {
    s.trim()
        .lines()
        .map(|line| {
            let (parent, children) = line.split_once(':').unwrap();
            let children = children.split_whitespace().collect();

            (parent, children)
        })
        .collect()
}
