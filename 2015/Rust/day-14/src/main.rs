fn main() {
    let reindeers = parse();

    let race_duration: u32 = 2503;

    let winner_dist = reindeers
        .iter()
        .map(|r| r.distance_travelled(race_duration))
        .max()
        .unwrap();

    println!("Part 1. Winner traveled {} kms", winner_dist);

    let mut scores: Vec<usize> = vec![0; reindeers.len()];

    for time in 1..=race_duration {
        let distances: Vec<u32> = reindeers
            .iter()
            .map(|r| r.distance_travelled(time))
            .collect();

        let max = distances.iter().max().unwrap();

        for (i, dist) in distances.iter().enumerate() {
            if dist == max {
                scores[i] += 1;
            }
        }
    }

    let winner_score = scores.iter().max().unwrap();
    println!("Part 2. Winner points = {}", winner_score);
}

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    fly_time: u32,
    rest_time: u32,
}

impl Reindeer {
    fn distance_travelled(&self, time: u32) -> u32 {
        let period = self.fly_time + self.rest_time;
        let full_periods = time / period;
        let remaining = time % period;
        let remaining_fly_time = remaining.min(self.fly_time);

        (full_periods * self.fly_time + remaining_fly_time) * self.speed
    }
}

fn parse() -> Vec<Reindeer> {
    let input = include_str!("../input.txt").trim();

    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let speed = parts[3].parse::<u32>().unwrap();
            let fly_time = parts[6].parse::<u32>().unwrap();
            let rest_time = parts[13].parse::<u32>().unwrap();

            Reindeer {
                speed,
                fly_time,
                rest_time,
            }
        })
        .collect()
}
