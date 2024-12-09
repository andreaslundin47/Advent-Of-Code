use std::collections::BTreeMap;

fn main() {
    let input = include_str!("../input.txt").trim();
    part_one(input);
    part_two(input);
}

#[derive(Debug, Clone, PartialEq)]
enum MemoryBlock {
    Free,
    Used(usize),
}

impl MemoryBlock {
    fn is_used(&self) -> bool {
        !matches!(self, Self::Free)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Memory {
    start: usize,
    length: usize,
}

fn part_one(input: &str) {
    let memory: Vec<MemoryBlock> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(i, len)| {
            if i % 2 == 0 {
                vec![MemoryBlock::Used(i / 2); len]
            } else {
                vec![MemoryBlock::Free; len]
            }
        })
        .collect();

    let compressed_size = memory.iter().filter(|block| block.is_used()).count();

    let mut used_memory_reversed_iter = memory
        .iter()
        .filter_map(|block| match block {
            MemoryBlock::Used(id) => Some(id),
            MemoryBlock::Free => None,
        })
        .rev();

    let checksum = std::iter::zip(0..compressed_size, memory.iter())
        .map(|(idx, block)| match block {
            MemoryBlock::Used(id) => idx * id,
            MemoryBlock::Free => {
                if let Some(id) = used_memory_reversed_iter.next() {
                    idx * id
                } else {
                    panic!("Ran out of blocks too early")
                }
            }
        })
        .sum::<usize>();

    println!("Part 1. Checksum: {}", checksum);
}

fn part_two(input: &str) {
    let raw_memory_layout: Vec<usize> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut file_id = 0;
    let mut start = 0;

    let mut used_memory_blocks: BTreeMap<usize, Memory> = BTreeMap::new();
    let mut free_memory_blocks: BTreeMap<usize, usize> = BTreeMap::new();

    for (idx, length) in raw_memory_layout.into_iter().enumerate() {
        let is_used_memory_block = idx % 2 == 0;

        if is_used_memory_block {
            used_memory_blocks.insert(file_id, Memory { start, length });
            file_id += 1;
            start += length;
        } else {
            free_memory_blocks.insert(start, length);
            start += length;
        }
    }

    for memory_block in used_memory_blocks.values_mut().rev() {
        let viable_free_memory_location: Option<(usize, usize)> = free_memory_blocks
            .iter()
            .find(|(&start, &length)| start < memory_block.start && length >= memory_block.length)
            .map(|(&start, &length)| (start, length));

        if let Some((free_start, free_length)) = viable_free_memory_location {
            free_memory_blocks.remove(&free_start);
            memory_block.start = free_start;

            if memory_block.length < free_length {
                free_memory_blocks.insert(
                    free_start + memory_block.length,
                    free_length - memory_block.length,
                );
            }
        }
    }

    let checksum: usize = used_memory_blocks
        .iter()
        .map(|(block_id, block)| {
            (0..block.length)
                .map(|offset| (block.start + offset) * block_id)
                .sum::<usize>()
        })
        .sum();

    println!("Part 2. Checksum: {}", checksum);
}
