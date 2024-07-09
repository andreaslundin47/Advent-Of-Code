const TARGET: usize = 36_000_000;

fn main() {
    for i in 1.. {
        let presents = presents_for_house_one(i);
        if presents >= TARGET {
            println!("Part 1. {} presents at house {}", presents, i);
            break;
        }
    }

    for i in 1.. {
        let presents = presents_for_house_two(i);
        if presents >= TARGET {
            println!("Part 2. {} presents at house {}", presents, i);
            break;
        }
    }
}

fn presents_for_house_one(number: usize) -> usize {
    let root = (number as f32).sqrt().floor() as usize;

    let mut sum = 0;

    for elf_number in 1..=root {
        if number % elf_number == 0 {
            let complementary_elf_number = number / elf_number;

            sum += elf_number;

            if complementary_elf_number != elf_number {
                sum += complementary_elf_number;
            }
        }
    }

    10 * sum
}

fn presents_for_house_two(number: usize) -> usize {
    let root = (number as f32).sqrt().floor() as usize;

    let mut sum = 0;

    for elf_number in 1..=root {
        if number % elf_number == 0 {
            let complementary_elf_number = number / elf_number;

            if elf_number * 50 >= number {
                sum += elf_number;
            }

            if complementary_elf_number == elf_number {
                continue;
            } 
            
            if complementary_elf_number * 50 >= number {
                sum += complementary_elf_number;
            }
        }
    }

    11 * sum
}