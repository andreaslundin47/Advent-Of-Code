fn main() {
    let input = include_str!("../input.txt").trim();
    let blocked_ranges: Vec<IPRange> = parse(input);

    let full_range = IPRange {
        start: 0,
        end: 2i64.pow(32) - 1,
    };

    let valid_ranges: Vec<IPRange> =
        blocked_ranges
            .into_iter()
            .fold(vec![full_range], |current_valid_ranges, blocked_range| {
                current_valid_ranges
                    .into_iter()
                    .flat_map(|vr| vr.block_split(blocked_range))
                    .collect()
            });

    let smallest_valid_ip = valid_ranges[0].start;
    println!("Part 1. Smallest IP address: {smallest_valid_ip}");

    let valid_ip_count: usize = valid_ranges.iter().map(|vr| vr.size()).sum();
    println!("Part 2. Number of valid IP addresses: {valid_ip_count}");
}

#[derive(Debug, Copy, Clone)]
struct IPRange {
    start: i64,
    end: i64,
}

impl IPRange {
    fn size(&self) -> usize {
        (self.end - self.start + 1) as usize
    }

    fn block_split(&self, blocked: IPRange) -> Vec<IPRange> {
        let candidates = [
            IPRange {
                start: self.start,
                end: (blocked.start - 1).min(self.end),
            },
            IPRange {
                start: self.start.max(blocked.end + 1),
                end: self.end,
            },
        ];

        candidates
            .into_iter()
            .filter(|r| r.start <= r.end)
            .collect()
    }
}

fn parse(input: &str) -> Vec<IPRange> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            let start = parts[0].parse::<i64>().unwrap();
            let end = parts[1].parse::<i64>().unwrap();

            IPRange { start, end }
        })
        .collect()
}
