use std::{ops::Range, cmp::{max, min}};
use itertools::Itertools;

// ------------------------------------------------------------------------
fn main() {
    let raw_input = include_str!("../input.txt").trim();
    let (seeds, mappings) = parse_input(&raw_input);

    solve_part_one(&seeds, &mappings);
    solve_part_two(&seeds, &mappings);
}
// ------------------------------------------------------------------------
fn solve_part_one(seeds: &Vec<u64>, mappings: &Vec<GardenMap>) {
    let locations = seeds
        .iter()
        .map(|&seed| {
            mappings.iter().fold(seed, |src, map| map.dst_from_src(src) )
        })
        .collect::<Vec<u64>>();

    let lowest_location = locations.iter().min().expect("A valid minimum");

   println!("Part 1. Lowest location number = {lowest_location}");
}
// ------------------------------------------------------------------------
fn solve_part_two(seeds: &Vec<u64>, mappings: &Vec<GardenMap>) {

    let seed_ranges: Vec<Range<u64>> = seeds
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0];
            let length = chunk[1];
            start..(start+length)
        })
        .collect();


    let lowest_location: u64 = seed_ranges
        .iter()
        .map(|seed_range| {

            let mut current_ranges = vec![seed_range.clone()];
            let mut next_ranges = vec![];

            for map in mappings {
                for range in current_ranges.iter() {
                    for new_range in map.dst_ranges_from_src_range(&range) {
                        next_ranges.push(new_range);
                    }
                }
                std::mem::swap(&mut current_ranges, &mut next_ranges);
                next_ranges.clear();
            }

            current_ranges.iter().map(|range| range.start).min().expect("Should have a minimum")
        })
        .min()
        .unwrap();


   println!("Part 2. Lowest location number = {lowest_location}");
}
// ------------------------------------------------------------------------

#[derive(Debug)]
struct RangeEntry {
    dst_start: u64,
    src_start: u64,
    src_end: u64,
}

impl RangeEntry {
    fn contains(&self, src: u64) -> bool {
        self.src_start <= src && src < self.src_end
    }
}

// ------------------------------------------------------------------------
#[derive(Debug)]
struct GardenMap {
    ranges: Vec<RangeEntry>,
}

impl GardenMap {
    fn dst_from_src(&self, src: u64) -> u64 {
        for range in self.ranges.iter() {
            if range.contains(src) {
                let dst = range.dst_start + (src - range.src_start);
                return dst;
            }
        }
        src
    }

    fn dst_ranges_from_src_range(&self, src: &Range<u64>) -> Vec<Range<u64>> {

        let mut dst_ranges = vec![];

        // Add parts of src overlapping with the ranges
        for range in &self.ranges {
            if src.end > range.src_start &&  src.start < range.src_end {
                let src_start = max(src.start, range.src_start);
                let src_end = min(src.end, range.src_end);
                let dst_start = src_start + range.dst_start - range.src_start;
                let dst_end = src_end + range.dst_start - range.src_start;
                let dst = dst_start .. dst_end;
                dst_ranges.push(dst);
            }
        }

        // If src is outside the ranges, add these parts too
        let left = self.ranges.first().unwrap();
        if src.start < left.src_start {
            let dst = src.start .. min(src.end, left.src_start);
            dst_ranges.push(dst);
        }

        let right = self.ranges.last().unwrap();
        if right.src_end < src.end {
            let dst = max(src.start, right.src_end) .. src.end;
            dst_ranges.push(dst);
        }
        
        if dst_ranges.is_empty() {
            // There was a gap! Map to self!
            dst_ranges.push(src.clone());
        }
        dst_ranges
    }
}

// ------------------------------------------------------------------------

fn parse_map(ranges: &str) -> GardenMap {
    let mut ranges: Vec<RangeEntry> = ranges
        .lines()
        .skip(1)
        .map(|entry| {
            let (dst_start, src_start, length): (u64, u64, u64) = entry
                .split_whitespace().map(|v| v.parse().unwrap()).collect_tuple().unwrap();
            let src_end = src_start + length;
            RangeEntry { dst_start, src_start, src_end }
        })
        .collect();

    ranges.sort_by(|a, b| a.src_start.cmp(&b.src_start));

    GardenMap { ranges }
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<GardenMap>) {

    let mut input_blocks = input.split("\n\n");

    let seeds: Vec<u64> = input_blocks
            .next().expect("Seeds string")
            .strip_prefix("seeds: ").unwrap()
            .split_whitespace()
            .filter_map(|num| num.parse::<u64>().ok())
            .collect();

    let mappings = input_blocks
            .map(|block| parse_map(block))
            .collect::<Vec<GardenMap>>();

    (seeds, mappings)
}
// ------------------------------------------------------------------------