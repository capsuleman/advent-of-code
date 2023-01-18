use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::thread;

#[derive(Debug)]
struct Cost {
    ore: u64,
    clay: u64,
    obsidian: u64,
}

#[derive(Debug)]
struct Blueprint {
    id: u64,
    ore_robot_cost: Cost,
    clay_robot_cost: Cost,
    obsidian_robot_cost: Cost,
    geode_robot_cost: Cost,
}

#[derive(Debug, Clone, Copy)]
struct ProductionState {
    minute: u64,
    ore_robot_count: u64,
    clay_robot_count: u64,
    obsidian_robot_count: u64,
    geode_robot_count: u64,
    ore_count: u64,
    clay_count: u64,
    obsidian_count: u64,
    geode_count: u64,
}

const MAX_MINUTES: u64 = 32;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let mut handles: Vec<_> = Vec::new();

    for blueprint in parse_blueprints(file_path) {
        handles.push(thread::spawn(move || find_max_geode_count(&blueprint)));
    }

    let mut max_geode_product = 1;
    for process in handles {
        max_geode_product *= process.join().unwrap();
    }

    println!("Result: {max_geode_product}");
}

fn parse_blueprints(file_path: &String) -> Vec<Blueprint> {
    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);
    let mut line_iterator = buf_reader.lines().into_iter();

    let blueprint_regex = Regex::new(r"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$").unwrap();

    let mut blueprints: Vec<Blueprint> = Vec::new();

    while let Some(Ok(line)) = line_iterator.next() {
        let blueprint_capture = blueprint_regex
            .captures(&line)
            .expect("Issue parsing blueprint.");

        blueprints.push(Blueprint {
            id: blueprint_capture[1].parse::<u64>().unwrap(),
            ore_robot_cost: Cost {
                ore: blueprint_capture[2].parse::<u64>().unwrap(),
                clay: 0,
                obsidian: 0,
            },
            clay_robot_cost: Cost {
                ore: blueprint_capture[3].parse::<u64>().unwrap(),
                clay: 0,
                obsidian: 0,
            },
            obsidian_robot_cost: Cost {
                ore: blueprint_capture[4].parse::<u64>().unwrap(),
                clay: blueprint_capture[5].parse::<u64>().unwrap(),
                obsidian: 0,
            },
            geode_robot_cost: Cost {
                ore: blueprint_capture[6].parse::<u64>().unwrap(),
                clay: 0,
                obsidian: blueprint_capture[7].parse::<u64>().unwrap(),
            },
        });
    }

    blueprints
}

fn find_max_geode_count(blueprint: &Blueprint) -> u64 {
    let initial_production_state = ProductionState {
        minute: 0,
        ore_robot_count: 1,
        clay_robot_count: 0,
        obsidian_robot_count: 0,
        geode_robot_count: 0,
        ore_count: 0,
        clay_count: 0,
        obsidian_count: 0,
        geode_count: 0,
    };

    let max_geode_produced = find_max_geode_count_aux(&initial_production_state, &blueprint, 0, 0);
    println!(
        "[#{}]\tMax geode produced {max_geode_produced}",
        blueprint.id
    );

    max_geode_produced
}

fn pass_time(production_state: &mut ProductionState, minute_count: u64) {
    production_state.minute += minute_count;
    production_state.ore_count += production_state.ore_robot_count * minute_count;
    production_state.clay_count += production_state.clay_robot_count * minute_count;
    production_state.obsidian_count += production_state.obsidian_robot_count * minute_count;
    production_state.geode_count += production_state.geode_robot_count * minute_count;
}

fn pay_cost(production_state: &mut ProductionState, cost: &Cost) {
    production_state.ore_count -= cost.ore;
    production_state.clay_count -= cost.clay;
    production_state.obsidian_count -= cost.obsidian;
}

fn find_max_geode_count_aux(
    production_state: &ProductionState,
    blueprint: &Blueprint,
    current_max_geode_count: u64,
    indent: u64,
) -> u64 {
    if production_state.minute == MAX_MINUTES {
        return production_state.geode_count;
    }

    if get_max_geode_count_greater_value(production_state) < current_max_geode_count {
        return 0;
    }

    let mut new_robot_found = false;
    let mut temp_max_geode_count = current_max_geode_count;

    if let Some(required_minutes) =
        get_required_minutes_until_cost_reached(production_state, &blueprint.ore_robot_cost)
    {
        let is_robot_required = production_state.ore_robot_count < blueprint.ore_robot_cost.ore
            || production_state.ore_robot_count < blueprint.clay_robot_cost.ore
            || production_state.ore_robot_count < blueprint.obsidian_robot_cost.ore
            || production_state.ore_robot_count < blueprint.geode_robot_cost.ore;

        if production_state.minute + required_minutes < MAX_MINUTES && is_robot_required {
            let mut new_production_state = production_state.clone();
            pass_time(&mut new_production_state, required_minutes);
            pay_cost(&mut new_production_state, &blueprint.ore_robot_cost);
            new_production_state.ore_robot_count += 1;

            new_robot_found = true;
            temp_max_geode_count = std::cmp::max(
                temp_max_geode_count,
                find_max_geode_count_aux(
                    &new_production_state,
                    blueprint,
                    temp_max_geode_count,
                    indent + 1,
                ),
            );
        }
    }

    if let Some(required_minutes) =
        get_required_minutes_until_cost_reached(production_state, &blueprint.clay_robot_cost)
    {
        let is_robot_required = production_state.clay_robot_count < blueprint.ore_robot_cost.clay
            || production_state.clay_robot_count < blueprint.clay_robot_cost.clay
            || production_state.clay_robot_count < blueprint.obsidian_robot_cost.clay
            || production_state.clay_robot_count < blueprint.geode_robot_cost.clay;

        if production_state.minute + required_minutes < MAX_MINUTES && is_robot_required {
            let mut new_production_state = production_state.clone();
            pass_time(&mut new_production_state, required_minutes);
            pay_cost(&mut new_production_state, &blueprint.clay_robot_cost);
            new_production_state.clay_robot_count += 1;

            new_robot_found = true;
            temp_max_geode_count = std::cmp::max(
                temp_max_geode_count,
                find_max_geode_count_aux(
                    &new_production_state,
                    blueprint,
                    temp_max_geode_count,
                    indent + 1,
                ),
            );
        }
    }

    if let Some(required_minutes) =
        get_required_minutes_until_cost_reached(production_state, &blueprint.obsidian_robot_cost)
    {
        let is_robot_required = production_state.obsidian_robot_count
            < blueprint.ore_robot_cost.obsidian
            || production_state.obsidian_robot_count < blueprint.clay_robot_cost.obsidian
            || production_state.obsidian_robot_count < blueprint.obsidian_robot_cost.obsidian
            || production_state.obsidian_robot_count < blueprint.geode_robot_cost.obsidian;

        if production_state.minute + required_minutes < MAX_MINUTES && is_robot_required {
            let mut new_production_state = production_state.clone();
            pass_time(&mut new_production_state, required_minutes);
            pay_cost(&mut new_production_state, &blueprint.obsidian_robot_cost);
            new_production_state.obsidian_robot_count += 1;

            new_robot_found = true;
            temp_max_geode_count = std::cmp::max(
                temp_max_geode_count,
                find_max_geode_count_aux(
                    &new_production_state,
                    blueprint,
                    temp_max_geode_count,
                    indent + 1,
                ),
            );
        }
    }

    if let Some(required_minutes) =
        get_required_minutes_until_cost_reached(production_state, &blueprint.geode_robot_cost)
    {
        if production_state.minute + required_minutes < MAX_MINUTES {
            let mut new_production_state = production_state.clone();
            pass_time(&mut new_production_state, required_minutes);
            pay_cost(&mut new_production_state, &blueprint.geode_robot_cost);
            new_production_state.geode_robot_count += 1;

            new_robot_found = true;
            temp_max_geode_count = std::cmp::max(
                temp_max_geode_count,
                find_max_geode_count_aux(
                    &new_production_state,
                    blueprint,
                    temp_max_geode_count,
                    indent + 1,
                ),
            );
        }
    }

    if new_robot_found {
        return temp_max_geode_count;
    }

    let mut new_production_state = production_state.clone();

    let remaining_minutes = MAX_MINUTES - new_production_state.minute;
    pass_time(&mut new_production_state, remaining_minutes);
    new_production_state.geode_count
}

fn get_required_minutes_until_cost_reached(
    production_state: &ProductionState,
    cost: &Cost,
) -> Option<u64> {
    if (cost.ore > 0 && production_state.ore_robot_count == 0)
        || (cost.clay > 0 && production_state.clay_robot_count == 0)
        || (cost.obsidian > 0 && production_state.obsidian_robot_count == 0)
    {
        return None;
    }

    let required_minutes_for_ore = if production_state.ore_count >= cost.ore {
        1
    } else {
        ceiled_division(
            cost.ore - production_state.ore_count,
            production_state.ore_robot_count,
        ) + 1
    };
    let required_minutes_for_clay = if production_state.clay_count >= cost.clay {
        1
    } else {
        ceiled_division(
            cost.clay - production_state.clay_count,
            production_state.clay_robot_count,
        ) + 1
    };
    let required_minutes_for_obsidian = if production_state.obsidian_count >= cost.obsidian {
        1
    } else {
        ceiled_division(
            cost.obsidian - production_state.obsidian_count,
            production_state.obsidian_robot_count,
        ) + 1
    };

    let required_minutes = Vec::from([
        required_minutes_for_ore,
        required_minutes_for_clay,
        required_minutes_for_obsidian,
    ])
    .into_iter()
    .max()
    .unwrap();

    Some(required_minutes)
}

fn ceiled_division(x: u64, y: u64) -> u64 {
    if x == 0 && y == 0 {
        return 0;
    }

    (x + y - 1) / y
}

fn get_max_geode_count_greater_value(production_state: &ProductionState) -> u64 {
    let remaining_minutes = MAX_MINUTES - production_state.minute;
    let geode_robot_count = production_state.geode_robot_count;
    let geode_count = production_state.geode_count;

    geode_count // Initial number of geode
        + geode_robot_count * remaining_minutes // Current robot production
        + remaining_minutes * (remaining_minutes + 1) / 2 // Production if each remaining minute produces a robot
}
