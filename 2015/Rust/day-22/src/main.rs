use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;

fn main() {
    let raw_input = include_str!("../input.txt").trim();

    let (boss_hp, boss_damage) = raw_input
        .lines()
        .map(|line| line.split_once(": ").unwrap().1.parse::<u32>().unwrap())
        .collect_tuple()
        .unwrap();

    let init_state = State {
        player_turn: true,
        player_hp: 50,
        player_mana: 500,
        player_armour: 0,
        boss_hp,
        boss_damage,
        shield_timer: 0,
        poison_timer: 0,
        recharge_timer: 0,
        mana_spent: 0,
    };

    let mut prio_queue = PriorityQueue::new();

    prio_queue.push(init_state, Reverse(0));

    while let Some((mut state, _)) = prio_queue.pop() {
        if state.boss_hp <= 0 {
            println!("Part 1. Mana spent: {}", state.mana_spent);
            break;
        }

        for new_state in state.next_states() {
            let priority = Reverse(new_state.mana_spent);
            prio_queue.push(new_state, priority);
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
struct State {
    player_turn: bool,
    player_hp: u32,
    player_mana: u32,
    player_armour: u32,
    boss_hp: u32,
    boss_damage: u32,
    shield_timer: u32,
    poison_timer: u32,
    recharge_timer: u32,
    mana_spent: u32,
}

impl State {
    fn next_states(&mut self) -> Vec<State> {
        self.apply_effects();

        if self.boss_hp <= 0 {
            return vec![*self];
        }

        if false == self.player_turn {
            let damage = self.boss_damage.saturating_sub(self.player_armour).max(1);
            self.player_hp = self.player_hp.saturating_sub(damage);

            if self.player_hp <= 0 {
                return vec![]
            } else {
                self.player_turn = true;
                return vec![*self];
            }
        }

        self.player_turn = false;
        let mut new_states = vec![];

        // Consider each move

        // Magic Missile
        if self.player_mana >= 53 {
            new_states.push(
                State {
                    player_mana: self.player_mana - 53,
                    boss_hp: self.boss_hp.saturating_sub(4),
                    mana_spent: self.mana_spent + 53,
                    ..*self
                }
            );
        }

        // Drain
        if self.player_mana >= 73 {
            new_states.push(
                State {
                    player_mana: self.player_mana - 73,
                    player_hp: self.player_hp + 2,
                    boss_hp: self.boss_hp.saturating_sub(2),
                    mana_spent: self.mana_spent + 73,
                    ..*self
                }
            );
        }

        // Shield
        if self.shield_timer == 0 && self.player_mana >= 113 {
            new_states.push(
                State {
                    player_mana: self.player_mana - 113,
                    player_armour: 7,
                    shield_timer: 6,
                    mana_spent: self.mana_spent + 113,
                    ..*self
                }
            );
        }

        // Poison
        if self.poison_timer == 0 && self.player_mana >= 173 {
            new_states.push(
                State {
                    player_mana: self.player_mana - 173,
                    poison_timer: 6,
                    mana_spent: self.mana_spent + 173,
                    ..*self
                }
            );
        }

        // Recharge
        if self.recharge_timer == 0 && self.player_mana >= 229 {
            new_states.push(
                State {
                    player_mana: self.player_mana - 229,
                    recharge_timer: 5,
                    mana_spent: self.mana_spent + 229,
                    ..*self
                }
            );
        }

        new_states
    }


    fn apply_effects(&mut self) {
        self.player_hp = self.player_hp.saturating_sub(1); // Part 2 Only!!!

        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        } else {
            self.player_armour = 0;
        }

        if self.poison_timer > 0 {
            self.boss_hp = self.boss_hp.saturating_sub(3);
            self.poison_timer -= 1;
        }

        if self.recharge_timer > 0 {
            self.player_mana += 101;
            self.recharge_timer -= 1;
        }
    }
}