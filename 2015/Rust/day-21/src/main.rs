use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt").trim();
    let (hp, damage, armor) = input
        .lines()
        .map(|line| {
            let (_, value) = line.split_once(": ").unwrap();
            value.parse::<usize>().unwrap()
        })
        .collect_tuple()
        .unwrap();

    let boss = Fighter {
        hp,
        damage,
        armor,
        cost: 0,
    };

    let weapons = [
        Item::new(8, 4, 0),
        Item::new(10, 5, 0),
        Item::new(25, 6, 0),
        Item::new(40, 7, 0),
        Item::new(74, 8, 0),
    ];

    let armor = [
        Item::new(0, 0, 0),
        Item::new(13, 0, 1),
        Item::new(31, 0, 2),
        Item::new(53, 0, 3),
        Item::new(75, 0, 4),
        Item::new(102, 0, 5),
    ];

    let rings = [
        Item::new(0, 0, 0),
        Item::new(0, 0, 0),
        Item::new(25, 1, 0),
        Item::new(50, 2, 0),
        Item::new(100, 3, 0),
        Item::new(20, 0, 1),
        Item::new(40, 0, 2),
        Item::new(80, 0, 3),
    ];

    let (min_cost, max_cost) = optimize_costs(&boss, &weapons, &armor, &rings);

    println!("Part 1. Lowest cost win = {}", min_cost);
    println!("Part 2. Highest cost loss = {}", max_cost);
}

fn optimize_costs(
    boss: &Fighter,
    weapons: &[Item],
    armor: &[Item],
    rings: &[Item],
) -> (usize, usize) {
    let mut min_cost = usize::MAX;
    let mut max_cost = 0;

    let ring_combos = rings.iter().combinations(2);

    let combinations_iter = ring_combos
        .cartesian_product(weapons)
        .cartesian_product(armor);

    for ((rings, weapon), armour) in combinations_iter {
        let mut player = Fighter::new(100);
        player.equip(weapon);
        player.equip(armour);
        player.equip(rings[0]);
        player.equip(rings[1]);

        if player.beats(boss) {
            min_cost = min_cost.min(player.cost);
        } else {
            max_cost = max_cost.max(player.cost);
        }
    }

    (min_cost, max_cost)
}
struct Fighter {
    hp: usize,
    damage: usize,
    armor: usize,
    cost: usize,
}

impl Fighter {
    fn new(hp: usize) -> Fighter {
        Fighter {
            hp,
            damage: 0,
            armor: 0,
            cost: 0,
        }
    }

    fn equip(&mut self, item: &Item) {
        self.damage += item.damage;
        self.armor += item.armor;
        self.cost += item.cost;
    }

    fn beats(&self, enemy: &Fighter) -> bool {
        let net_damage = (self.damage.saturating_sub(enemy.armor)).max(1);
        let enemy_net_damage = (enemy.damage.saturating_sub(self.armor)).max(1);

        let turns_to_beat = enemy.hp / net_damage
            + match enemy.hp % net_damage == 0 {
                true => 0,
                false => 1,
            };

        let enemy_turns_to_beat = self.hp / enemy_net_damage
            + match self.hp % enemy_net_damage == 0 {
                true => 0,
                false => 1,
            };

        turns_to_beat <= enemy_turns_to_beat
    }
}

#[derive(Clone)]
struct Item {
    cost: usize,
    damage: usize,
    armor: usize,
}

impl Item {
    fn new(cost: usize, damage: usize, armor: usize) -> Item {
        Item {
            cost,
            damage,
            armor,
        }
    }
}
