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

const MAX_MINUTES: u64 = 24;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let mut handles: Vec<_> = Vec::new();

    for blueprint in parse_blueprints(file_path) {
        handles.push(thread::spawn(move || get_quality_level(&blueprint)));
    }

    let mut quality_level_sum = 0;
    for process in handles {
        quality_level_sum += process.join().unwrap();
    }

    println!("Result: {quality_level_sum}");
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

fn get_quality_level(blueprint: &Blueprint) -> u64 {
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

    let max_geode_produced = find_max_geode_count(&initial_production_state, &blueprint, 0);
    println!(
        "[#{}]\tMax geode produced {max_geode_produced}",
        blueprint.id
    );

    max_geode_produced * blueprint.id
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

fn find_max_geode_count(
    production_state: &ProductionState,
    blueprint: &Blueprint,
    indent: u64,
) -> u64 {
    if production_state.minute == MAX_MINUTES {
        return production_state.geode_count;
    }

    let mut max_geode_counts = Vec::new();

    if let Some(required_minutes) =
        get_required_minutes_until_cost_reached(production_state, &blueprint.ore_robot_cost)
    {
        if production_state.minute + required_minutes < MAX_MINUTES {
            let mut new_production_state = production_state.clone();
            pass_time(&mut new_production_state, required_minutes);
            pay_cost(&mut new_production_state, &blueprint.ore_robot_cost);
            new_production_state.ore_robot_count += 1;

            max_geode_counts.push(find_max_geode_count(
                &new_production_state,
                blueprint,
                indent + 1,
            ));
        }
    }

    if let Some(required_minutes) =
        get_required_minutes_until_cost_reached(production_state, &blueprint.clay_robot_cost)
    {
        if production_state.minute + required_minutes < MAX_MINUTES {
            let mut new_production_state = production_state.clone();
            pass_time(&mut new_production_state, required_minutes);
            pay_cost(&mut new_production_state, &blueprint.clay_robot_cost);
            new_production_state.clay_robot_count += 1;

            max_geode_counts.push(find_max_geode_count(
                &new_production_state,
                blueprint,
                indent + 1,
            ));
        }
    }

    if let Some(required_minutes) =
        get_required_minutes_until_cost_reached(production_state, &blueprint.obsidian_robot_cost)
    {
        if production_state.minute + required_minutes < MAX_MINUTES {
            let mut new_production_state = production_state.clone();
            pass_time(&mut new_production_state, required_minutes);
            pay_cost(&mut new_production_state, &blueprint.obsidian_robot_cost);
            new_production_state.obsidian_robot_count += 1;

            max_geode_counts.push(find_max_geode_count(
                &new_production_state,
                blueprint,
                indent + 1,
            ));
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

            max_geode_counts.push(find_max_geode_count(
                &new_production_state,
                blueprint,
                indent + 1,
            ));
        }
    }

    if max_geode_counts.len() > 0 {
        let possibilities_count = max_geode_counts.len();
        let max_geode_count = max_geode_counts.into_iter().max().unwrap();
        if indent <= 5 {
            println!(
                "[#{}]\t{indent}\tFound among {possibilities_count} possibilities -> {max_geode_count}", blueprint.id
            );
        }

        return max_geode_count;
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
