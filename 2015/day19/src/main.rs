use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");

    let (reverse_transitions, target_molecule) = parse_input(&file_path);

    println!(
        "{}",
        get_shortest_count(target_molecule, &reverse_transitions)
    );
}

fn parse_input(file_path: &String) -> (HashMap<String, String>, String) {
    let mut reverse_transitions = HashMap::new();

    let file = File::open(file_path).expect("File not found!");
    let mut line_iter = BufReader::new(file).lines();

    while let Some(Ok(line)) = line_iter.next() {
        if line.is_empty() {
            break;
        }

        let split_line = line.split(" => ").collect::<Vec<&str>>();
        reverse_transitions.insert(split_line[1].to_string(), split_line[0].to_string());
    }

    let target_molecule = line_iter.next().expect("last line is molecule.").unwrap();

    (reverse_transitions, target_molecule)
}

fn get_next_molecules(
    molecules: HashSet<String>,
    reverse_transitions: &HashMap<String, String>,
) -> HashSet<String> {
    let mut new_molecules = HashSet::new();

    for molecule in molecules {
        for (target, result) in reverse_transitions {
            for (start_index, _) in molecule.match_indices(target) {
                let mut new_molecule = molecule.clone();
                new_molecule.replace_range(start_index..(start_index + target.len()), result);
                new_molecules.insert(new_molecule);
            }
        }
    }
    new_molecules
}

fn get_shortest_count(
    target_molecule: String,
    reverse_transitions: &HashMap<String, String>,
) -> u32 {
    let mut count = 0;
    let mut molecules = HashSet::from([target_molecule]);

    while !molecules.contains("e") {
        molecules = get_next_molecules(molecules, reverse_transitions);
        molecules = get_reduced_molecules(molecules);
        count += 1;
        println!("{:?}", molecules);
    }

    count
}

fn get_reduced_molecules(mut molecules: HashSet<String>) -> HashSet<String> {
    let mut reduced_molecules = HashSet::new();

    for _ in 0..usize::min(20, molecules.len()) {
        let molecule = molecules
            .iter()
            .min_by_key(|molecule| molecule.len())
            .expect("minimum length molecule.")
            .clone();

        molecules.remove(&molecule);
        reduced_molecules.insert(molecule);
    }

    reduced_molecules
}
