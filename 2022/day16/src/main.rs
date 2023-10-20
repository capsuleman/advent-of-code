use itertools::iproduct;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;
use tqdm::tqdm;

#[derive(Debug)]
struct Valve {
    flow_rate: u32,
    neighbor_valves: HashSet<String>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Possibility {
    opened_valves: Vec<String>,
    current_valves: Vec<String>,
}

lazy_static! {
    static ref PARSER_REGEX: Regex =
        Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
            .unwrap();
}

const MAX_STEP: u32 = 26;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");
    let valves = parse_file(file_path);

    let mut possibilities: HashMap<Possibility, u32> = HashMap::from([(
        Possibility {
            opened_valves: Vec::new(),
            current_valves: Vec::from([String::from("AA"), String::from("AA")]),
        },
        0,
    )]);

    for step_index in 0..MAX_STEP {
        println!("{step_index}\t{:} possibilities", possibilities.len());

        let max_total_flow = *possibilities.values().max().unwrap();
        println!("{}", max_total_flow);

        let mut next_possibilities: HashMap<Possibility, u32> = HashMap::new();
        for (possibility, total_flow) in tqdm(possibilities.into_iter()) {
            let remaining_flow_upper_value =
                get_remaining_flow_upper_value(&valves, &possibility, step_index);

            if max_total_flow > total_flow + remaining_flow_upper_value {
                continue;
            }

            for (next_possibility, new_total_flow) in
                get_next_possibilities(&valves, possibility, total_flow).into_iter()
            {
                let max_total_flow = *next_possibilities.get(&next_possibility).unwrap_or(&0);
                if new_total_flow >= max_total_flow {
                    next_possibilities.insert(next_possibility, new_total_flow);
                }
            }
        }
        possibilities = next_possibilities;
    }

    let max_total_flow = *possibilities.values().max().unwrap();
    println!("Result: {max_total_flow}");
}

fn parse_file(file_path: &String) -> HashMap<String, Valve> {
    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let mut valves: HashMap<String, Valve> = HashMap::new();

    while let Some(Ok(line)) = line_iterator.next() {
        let captures = PARSER_REGEX
            .captures(&line)
            .expect("Error parsing input line.");

        let valve_name = String::from(&captures[1]);
        let flow_rate = captures[2].parse::<u32>().unwrap();
        let mut neighbor_valves: HashSet<String> = HashSet::new();
        captures[3].split(", ").for_each(|neighbor_valve| {
            neighbor_valves.insert(String::from(neighbor_valve));
        });

        valves.insert(
            valve_name,
            Valve {
                flow_rate,
                neighbor_valves,
            },
        );
    }

    valves
}

fn get_next_possibilities(
    valves: &HashMap<String, Valve>,
    possibility: Possibility,
    total_flow: u32,
) -> HashMap<Possibility, u32> {
    let mut next_possibilities: HashMap<Possibility, u32> = HashMap::new();

    let current_flow: u32 = possibility
        .opened_valves
        .iter()
        .map(|open_valve| valves[open_valve].flow_rate)
        .sum();

    let is_current_first_valve_open = possibility
        .opened_valves
        .contains(&possibility.current_valves[0]);
    let current_first_valve_flow = valves[&possibility.current_valves[0]].flow_rate;

    let should_try_open_current_first_valve =
        !is_current_first_valve_open && current_first_valve_flow > 0;

    let is_current_second_valve_open = possibility
        .opened_valves
        .contains(&possibility.current_valves[1]);
    let current_second_valve_flow = valves[&possibility.current_valves[1]].flow_rate;

    let should_try_open_current_second_valve =
        !is_current_second_valve_open && current_second_valve_flow > 0;

    if should_try_open_current_first_valve && should_try_open_current_second_valve {
        let mut opened_valves = possibility.opened_valves.clone();
        opened_valves.extend(possibility.current_valves.clone().into_iter().unique());
        opened_valves.sort();

        next_possibilities.insert(
            Possibility {
                opened_valves,
                current_valves: possibility.current_valves.clone(),
            },
            total_flow + current_flow,
        );
    }

    if should_try_open_current_first_valve {
        for next_possibility in
            get_next_possibilities_one_opening(valves, &possibility, true).into_iter()
        {
            next_possibilities.insert(next_possibility, total_flow + current_flow);
        }
    };

    if should_try_open_current_second_valve {
        for next_possibility in
            get_next_possibilities_one_opening(valves, &possibility, false).into_iter()
        {
            next_possibilities.insert(next_possibility, total_flow + current_flow);
        }
    };

    let first_valve_neighbors = Vec::from_iter(
        valves
            .get(&possibility.current_valves[0])
            .unwrap()
            .neighbor_valves
            .clone(),
    );
    let second_valve_neighbors = Vec::from_iter(
        valves
            .get(&possibility.current_valves[1])
            .unwrap()
            .neighbor_valves
            .clone(),
    );

    for (first_neighbor, second_neighbor) in
        iproduct!(first_valve_neighbors, second_valve_neighbors)
    {
        let mut next_current_valves = Vec::from([first_neighbor, second_neighbor]);
        next_current_valves.sort();
        next_possibilities.insert(
            Possibility {
                current_valves: next_current_valves,
                opened_valves: possibility.opened_valves.clone(),
            },
            total_flow + current_flow,
        );
    }

    next_possibilities
}

fn get_next_possibilities_one_opening(
    valves: &HashMap<String, Valve>,
    possibility: &Possibility,
    is_first_valve_opening: bool,
) -> HashSet<Possibility> {
    let (opening_valve_index, moving_valve_index): (usize, usize) = match is_first_valve_opening {
        true => (0, 1),
        false => (1, 0),
    };

    let mut next_possibilities_one_opening = HashSet::new();

    let mut opened_valves = possibility.opened_valves.clone();
    opened_valves.push(possibility.current_valves[opening_valve_index].clone());
    opened_valves.sort();

    for neighbor_valves in valves
        .get(&possibility.current_valves[moving_valve_index])
        .unwrap()
        .neighbor_valves
        .clone()
    {
        let mut current_valves = Vec::from([
            possibility.current_valves[opening_valve_index].clone(),
            neighbor_valves,
        ]);
        current_valves.sort();
        next_possibilities_one_opening.insert(Possibility {
            opened_valves: opened_valves.clone(),
            current_valves,
        });
    }

    next_possibilities_one_opening
}

fn get_remaining_flow_upper_value(
    valves: &HashMap<String, Valve>,
    possibility: &Possibility,
    step_count: u32,
) -> u32 {
    let closed_valves_flow_rate: u32 = valves
        .iter()
        .filter(|(valve_name, _)| !possibility.opened_valves.contains(valve_name))
        .map(|(_, valve)| valve.flow_rate)
        .sum();

    closed_valves_flow_rate * (MAX_STEP - step_count)
}
