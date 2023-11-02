use std::iter::Sum;

#[derive(Debug, Clone, Copy)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32,
}

impl Sum for Item {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, item| Item {
            cost: acc.cost + item.cost,
            damage: acc.damage + item.damage,
            armor: acc.armor + item.armor,
        })
        .unwrap()
    }
}

const WEAPONS: [Item; 5] = [
    Item {
        cost: 8,
        damage: 4,
        armor: 0,
    },
    Item {
        cost: 10,
        damage: 5,
        armor: 0,
    },
    Item {
        cost: 25,
        damage: 6,
        armor: 0,
    },
    Item {
        cost: 40,
        damage: 7,
        armor: 0,
    },
    Item {
        cost: 74,
        damage: 8,
        armor: 0,
    },
];

const ARMORS: [Item; 5] = [
    Item {
        cost: 13,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 31,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 53,
        damage: 0,
        armor: 3,
    },
    Item {
        cost: 75,
        damage: 0,
        armor: 4,
    },
    Item {
        cost: 102,
        damage: 0,
        armor: 5,
    },
];

const RINGS: [Item; 6] = [
    Item {
        cost: 25,
        damage: 1,
        armor: 0,
    },
    Item {
        cost: 50,
        damage: 2,
        armor: 0,
    },
    Item {
        cost: 100,
        damage: 3,
        armor: 0,
    },
    Item {
        cost: 20,
        damage: 0,
        armor: 1,
    },
    Item {
        cost: 40,
        damage: 0,
        armor: 2,
    },
    Item {
        cost: 80,
        damage: 0,
        armor: 3,
    },
];

const ENEMY_HP: u32 = 103;
const ENEMY_DAMAGE: u32 = 9;
const ENEMY_ARMOR: u32 = 2;

const PLAYER_HP: u32 = 100;

fn main() {
    let mut max_cost = 0;

    for set in generate_sets() {
        if !is_winning_set(&set) {
            max_cost = u32::max(max_cost, set.cost);
        }
    }

    println!("{}", max_cost);
}

fn generate_sets() -> Vec<Item> {
    let mut sets = vec![];

    for weapon_index in 0..WEAPONS.len() {
        for armor_index in -1..ARMORS.len() as i32 {
            for ring_1_index in -2..RINGS.len() as i32 {
                for ring_2_index in (ring_1_index + 1)..RINGS.len() as i32 {
                    let mut items = vec![WEAPONS[weapon_index]];

                    if armor_index >= 0 {
                        items.push(ARMORS[armor_index as usize]);
                    }

                    if ring_1_index >= 0 {
                        items.push(RINGS[ring_1_index as usize]);
                    }

                    if ring_2_index >= 0 {
                        items.push(RINGS[ring_2_index as usize]);
                    }

                    sets.push(items.into_iter().sum::<Item>());
                }
            }
        }
    }
    sets
}

fn is_winning_set(set: &Item) -> bool {
    let turn_to_kill_player = get_turn_count_to_kill(ENEMY_DAMAGE, PLAYER_HP, set.armor);
    let turn_to_kill_enemy = get_turn_count_to_kill(set.damage, ENEMY_HP, ENEMY_ARMOR);

    turn_to_kill_player >= turn_to_kill_enemy
}

fn get_turn_count_to_kill(attacker_damage: u32, defender_hp: u32, defender_armor: u32) -> u32 {
    let turn_damage = get_damage(attacker_damage, defender_armor);
    defender_hp / turn_damage + if defender_hp % turn_damage != 0 { 1 } else { 0 }
}

fn get_damage(attacker_damage: u32, defender_armor: u32) -> u32 {
    if defender_armor >= attacker_damage {
        1
    } else {
        attacker_damage - defender_armor
    }
}
