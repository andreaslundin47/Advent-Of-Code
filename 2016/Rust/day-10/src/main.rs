mod parser;
use std::collections::HashMap;
type Chip = usize;

fn main() {
    let input = include_str!("../input.txt").trim();
    let instructions: Vec<Instr> = parser::parse_instructions(input);

    let mut bots = create_bots(&instructions);
    let mut outputs: HashMap<usize, Vec<Chip>> = HashMap::new();

    while let Some(giving_bot) = bots.values_mut().find(|bot| bot.chips.len() > 1) {
        let (lower_chip, higher_chip) = giving_bot.get_chips();

        if lower_chip == 17 && higher_chip == 61 {
            println!("Part 1. Bot number = {}", giving_bot.id);
        }

        let lower_recipient = giving_bot.lower;
        let higher_recipient = giving_bot.higher;

        match lower_recipient {
            Recipient::Bot(bot) => {
                if let Some(bot) = bots.get_mut(&bot) {
                    bot.add_chip(lower_chip);
                }
            }
            Recipient::Output(out) => {
                outputs.entry(out).or_default().push(lower_chip);
            }
        }

        match higher_recipient {
            Recipient::Bot(bot) => {
                if let Some(bot) = bots.get_mut(&bot) {
                    bot.add_chip(higher_chip);
                }
            }
            Recipient::Output(out) => {
                outputs.entry(out).or_default().push(higher_chip);
            }
        }
    }

    let product: usize = (0..3)
        .map(|index| {
            if let Some(chips) = outputs.get(&index) {
                if chips.len() != 1 {
                    panic!("Output {} does have {} chips, not one", index, chips.len());
                }
                chips[0]
            } else {
                panic!("No output {}!", index);
            }
        })
        .product();

    println!("Part 2. Product = {}", product);
}

#[derive(Debug, Clone, Copy)]
enum Instr {
    Init {
        bot: usize,
        value: Chip,
    },
    Rule {
        bot: usize,
        lower: Recipient,
        higher: Recipient,
    },
}

#[derive(Debug, Clone, Copy)]
enum Recipient {
    Bot(usize),
    Output(usize),
}

#[derive(Debug)]
struct Bot {
    id: usize,
    lower: Recipient,
    higher: Recipient,
    chips: Vec<Chip>,
}

impl Bot {
    fn new(id: usize, lower: Recipient, higher: Recipient) -> Self {
        Bot {
            id,
            lower,
            higher,
            chips: vec![],
        }
    }

    fn add_chip(&mut self, chip: Chip) {
        self.chips.push(chip);
    }

    fn get_chips(&mut self) -> (Chip, Chip) {
        if self.chips.len() != 2 {
            panic!("The giving bot should have exactly 2 chips at this time!");
        }
        self.chips.sort();
        let output = (self.chips[0], self.chips[1]);
        self.chips.clear();

        output
    }
}

fn create_bots(instructions: &Vec<Instr>) -> HashMap<usize, Bot> {
    let mut bots = HashMap::new();

    for instr in instructions.iter() {
        if let Instr::Rule { bot, lower, higher } = *instr {
            let new_bot = Bot::new(bot, lower, higher);
            bots.insert(bot, new_bot);
        }
    }

    for instr in instructions.iter() {
        if let Instr::Init { bot, value } = *instr {
            if let Some(bot) = bots.get_mut(&bot) {
                bot.add_chip(value);
            }
        }
    }

    bots
}
