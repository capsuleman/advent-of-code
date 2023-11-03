use std::collections::HashSet;

#[derive(PartialEq, Eq)]
enum Spell {
    InstantSpell(Instant),
    EffectSpell(Effect),
}

#[derive(PartialEq, Eq)]
enum Instant {
    MagicMissile,
    Drain,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Effect {
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, PartialEq)]
enum EndGame {
    PlayerWin,
    MonsterWin,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct EffectsCount {
    shield_count: u8,
    poison_count: u8,
    recharge_count: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Game {
    player_mana: u32,
    player_hp: u32,

    monster_hp: u32,
    monster_damage: u32,

    effects: EffectsCount,
    mana_spent: u32,
}

const MAGIC_MISSILE_COST: u32 = 53;
const MAGIC_MISSILE_DAMAGE: u32 = 4;

const DRAIN_COST: u32 = 73;
const DRAIN_TRANSFER: u32 = 2;

const SHIELD_COST: u32 = 113;
const SHIELD_DURATION: u8 = 6;
const SHIELD_ARMOR_GAIN: u32 = 7;

const POISON_COST: u32 = 173;
const POISON_DURATION: u8 = 6;
const POISON_DAMAGE: u32 = 3;

const RECHARGE_COST: u32 = 229;
const RECHARGE_DURATION: u8 = 5;
const RECHARGE_MANA_GAIN: u32 = 101;

const PLAYER_HP: u32 = 50;
const PLAYER_MANA: u32 = 500;
const MONSTER_HP: u32 = 51;
const MONSTER_DAMAGE: u32 = 9;

fn main() {
    let mut games = HashSet::from([Game {
        player_hp: PLAYER_HP,
        player_mana: PLAYER_MANA,
        monster_hp: MONSTER_HP,
        monster_damage: MONSTER_DAMAGE,
        effects: EffectsCount {
            shield_count: 0,
            poison_count: 0,
            recharge_count: 0,
        },
        mana_spent: 0,
    }]);
    let mut min_mana_spent_turn;
    let mut min_mana_spent = u32::MAX;

    while games.len() > 0 {
        (games, min_mana_spent_turn) = get_all_next_turns(games);
        min_mana_spent = u32::min(min_mana_spent, min_mana_spent_turn);
    }

    println!("{}", min_mana_spent);
}

fn get_all_next_turns(games: HashSet<Game>) -> (HashSet<Game>, u32) {
    let mut next_games = HashSet::new();
    let mut min_mana_spent = u32::MAX;

    for game in games {
        for instant in [Instant::Drain, Instant::MagicMissile] {
            let mut next_game = game.clone();
            if let Err(error) = play(&mut next_game, Spell::InstantSpell(instant)) {
                if error == EndGame::PlayerWin {
                    min_mana_spent = u32::min(min_mana_spent, next_game.mana_spent);
                }
                continue;
            };
            next_games.insert(next_game);
        }

        for effect in [Effect::Shield, Effect::Poison, Effect::Recharge] {
            let effect_count = match effect {
                Effect::Shield => game.effects.shield_count,
                Effect::Poison => game.effects.poison_count,
                Effect::Recharge => game.effects.recharge_count,
            };
            if effect_count > 1 {
                continue;
            }

            let mut next_game = game.clone();
            if let Err(error) = play(&mut next_game, Spell::EffectSpell(effect)) {
                if error == EndGame::PlayerWin {
                    min_mana_spent = u32::min(min_mana_spent, next_game.mana_spent);
                }
                continue;
            };
            next_games.insert(next_game);
        }
    }

    (next_games, min_mana_spent)
}

fn tick_spells(effects: &mut EffectsCount) {
    effects.shield_count = effects.shield_count.checked_sub(1).unwrap_or(0);
    effects.poison_count = effects.poison_count.checked_sub(1).unwrap_or(0);
    effects.recharge_count = effects.recharge_count.checked_sub(1).unwrap_or(0);
}

fn play(game: &mut Game, spell: Spell) -> Result<(), EndGame> {
    if let Err(error) = play_player_turn(game, spell) {
        return Err(error);
    };
    if let Err(error) = play_monster_turn(game) {
        return Err(error);
    };

    Ok(())
}

fn play_player_turn(game: &mut Game, spell: Spell) -> Result<(), EndGame> {
    // println!("-- Player turn --");
    if game.player_hp <= 1 {
        return Err(EndGame::MonsterWin);
    }
    game.player_hp -= 1;

    if let Err(error) = play_default_actions(game) {
        return Err(error);
    };

    if spell == Spell::InstantSpell(Instant::MagicMissile) {
        // println!("Player casts Magic Missile.");
        if game.player_mana < MAGIC_MISSILE_COST {
            return Err(EndGame::MonsterWin);
        }
        game.player_mana -= MAGIC_MISSILE_COST;
        game.mana_spent += MAGIC_MISSILE_COST;

        if game.monster_hp <= MAGIC_MISSILE_DAMAGE {
            return Err(EndGame::PlayerWin);
        }
        game.monster_hp -= MAGIC_MISSILE_DAMAGE;
    }

    if spell == Spell::InstantSpell(Instant::Drain) {
        // println!("Player casts Drain.");
        if game.player_mana < DRAIN_COST {
            return Err(EndGame::MonsterWin);
        }
        game.player_mana -= DRAIN_COST;
        game.mana_spent += DRAIN_COST;

        if game.monster_hp <= DRAIN_TRANSFER {
            return Err(EndGame::PlayerWin);
        }
        game.monster_hp -= DRAIN_TRANSFER;
        game.player_hp += DRAIN_TRANSFER;
    }

    if spell == Spell::EffectSpell(Effect::Recharge) {
        // println!("Player casts Recharge.");
        if game.player_mana < RECHARGE_COST {
            return Err(EndGame::MonsterWin);
        }
        game.player_mana -= RECHARGE_COST;
        game.mana_spent += RECHARGE_COST;
        game.effects.recharge_count = RECHARGE_DURATION;
    }

    if spell == Spell::EffectSpell(Effect::Shield) {
        // println!("Player casts Shield.");
        if game.player_mana < SHIELD_COST {
            return Err(EndGame::MonsterWin);
        }
        game.player_mana -= SHIELD_COST;
        game.mana_spent += SHIELD_COST;
        game.effects.shield_count = SHIELD_DURATION;
    }

    if spell == Spell::EffectSpell(Effect::Poison) {
        // println!("Player casts Poison.");
        if game.player_mana < POISON_COST {
            return Err(EndGame::MonsterWin);
        }
        game.player_mana -= POISON_COST;
        game.mana_spent += POISON_COST;
        game.effects.poison_count = POISON_DURATION;
    }

    // println!();
    Ok(())
}

fn play_monster_turn(game: &mut Game) -> Result<(), EndGame> {
    // println!("-- Boss turn --");
    if let Err(error) = play_default_actions(game) {
        return Err(error);
    };

    let monster_damage = if game.effects.shield_count > 0 {
        // println!("Shield's timer is now {}.", game.effects.shield_count - 1);
        get_damage(game.monster_damage, SHIELD_ARMOR_GAIN)
    } else {
        get_damage(game.monster_damage, 0)
    };

    // println!("Boss attacks for {} damage!", monster_damage);

    if game.player_hp <= monster_damage {
        return Err(EndGame::MonsterWin);
    }
    game.player_hp -= monster_damage;

    // println!();
    Ok(())
}

fn play_default_actions(game: &mut Game) -> Result<(), EndGame> {
    // println!(
    //     "- Player has {} hit points, {} mana",
    //     game.player_hp, game.player_mana
    // );
    // println!("- Boss has {} hit points", game.monster_hp);

    if game.effects.shield_count > 0 {
        // println!("Shield's timer is now {}.", game.effects.shield_count - 1);
    }

    if game.effects.poison_count > 0 {
        // println!(
        //     "Poison deals {} damage; its timer is now {}.",
        //     POISON_DAMAGE,
        //     game.effects.poison_count - 1
        // );
        if game.monster_hp <= POISON_DAMAGE {
            return Err(EndGame::PlayerWin);
        }
        game.monster_hp -= POISON_DAMAGE;
    }

    if game.effects.recharge_count > 0 {
        // println!(
        //     "Recharge provides 101 mana; its timer is now {}.",
        //     game.effects.recharge_count - 1
        // );
        game.player_mana += RECHARGE_MANA_GAIN;
    }

    tick_spells(&mut game.effects);
    Ok(())
}

fn get_damage(attacker_damage: u32, defender_armor: u32) -> u32 {
    if defender_armor >= attacker_damage {
        1
    } else {
        attacker_damage - defender_armor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_double_overlap() {
        let mut initial_game = Game {
            player_hp: 10,
            player_mana: 250,
            monster_hp: 14,
            monster_damage: 8,
            effects: EffectsCount {
                shield_count: 0,
                poison_count: 0,
                recharge_count: 0,
            },
            mana_spent: 0,
        };

        assert_eq!(
            play(&mut initial_game, Spell::EffectSpell(Effect::Recharge)),
            Ok(())
        );
        assert_eq!(
            play(&mut initial_game, Spell::EffectSpell(Effect::Shield)),
            Ok(())
        );
        assert_eq!(
            play(&mut initial_game, Spell::InstantSpell(Instant::Drain)),
            Ok(())
        );
        assert_eq!(
            play(&mut initial_game, Spell::EffectSpell(Effect::Poison)),
            Ok(())
        );
        assert_eq!(
            play(
                &mut initial_game,
                Spell::InstantSpell(Instant::MagicMissile),
            ),
            Err(EndGame::PlayerWin)
        );
    }
}
