use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");

    let packets = parse_packets(&file_path);

    let total_weight: u128 = packets.iter().sum();
    let target_weight = total_weight / 3;

    println!("{packets:?} {target_weight}");

    let combinations_by_size = get_combination_by_size(target_weight, packets.clone());

    for first_package_size in 1..packets.len() {
        if !combinations_by_size.contains_key(&first_package_size) {
            continue;
        }

        let mut min_quantum_entanglement = u128::MAX;

        for first_package_packets in combinations_by_size[&first_package_size].clone() {
            let complement_packets: Vec<_> = first_package_packets
                .clone()
                .into_iter()
                .filter(|packet| !first_package_packets.contains(packet))
                .collect();

            if !could_be_split_equally(&complement_packets) {
                continue;
            };

            min_quantum_entanglement = u128::min(
                min_quantum_entanglement,
                get_quantum_entanglement(&first_package_packets),
            );
        }

        println!("{}", min_quantum_entanglement);
    }
}

fn parse_packets(file_path: &String) -> Vec<u128> {
    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.parse::<u128>().unwrap()
        })
        .collect()
}

fn get_combination_by_size(
    target_weight: u128,
    weights_combination: Vec<u128>,
) -> HashMap<usize, HashSet<Vec<u128>>> {
    let mut combinations_by_size = HashMap::new();
    for combination in get_combinations(target_weight, 0, weights_combination) {
        combinations_by_size
            .entry(combination.len())
            .or_insert_with(HashSet::new)
            .insert(combination);
    }
    combinations_by_size
}

fn get_combinations(
    target_weight: u128,
    current_weight: u128,
    mut weights_combination: Vec<u128>,
) -> Vec<Vec<u128>> {
    if target_weight == current_weight {
        return vec![vec![]];
    }

    let mut new_weights_combinations = vec![];

    while let Some(weight_combination) = weights_combination.pop() {
        if current_weight + weight_combination > target_weight {
            continue;
        }

        let sub_weights_combination = weights_combination.clone();

        let sub_solutions = get_combinations(
            target_weight,
            current_weight + weight_combination,
            sub_weights_combination,
        );

        for mut sub_solution in sub_solutions {
            sub_solution.push(weight_combination);
            new_weights_combinations.push(sub_solution);
        }
    }

    new_weights_combinations
}

fn could_be_split_equally(packets: &Vec<u128>) -> bool {
    let packet_size: u128 = packets.iter().sum();
    has_combinations(packet_size / 2, 0, packets.clone())
}

fn has_combinations(
    target_weight: u128,
    current_weight: u128,
    mut weights_combination: Vec<u128>,
) -> bool {
    if target_weight == current_weight {
        return true;
    }

    while let Some(weight_combination) = weights_combination.pop() {
        if current_weight + weight_combination > target_weight {
            continue;
        }

        let sub_weights_combination = weights_combination.clone();

        if has_combinations(
            target_weight,
            current_weight + weight_combination,
            sub_weights_combination,
        ) {
            return true;
        };
    }

    false
}

fn get_quantum_entanglement(packets: &Vec<u128>) -> u128 {
    packets.iter().fold(1, |acc, packet| acc * packet)
}
