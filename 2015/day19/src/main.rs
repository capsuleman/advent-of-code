use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");

    let (transitions, molecule) = parse_input(&file_path);

    let mut new_molecules = HashSet::new();

    for (target, results) in transitions {
        for (start_index, _) in molecule.match_indices(&target) {
            for result in results.iter() {
                let mut new_molecule = molecule.clone();
                new_molecule.replace_range(start_index..(start_index + target.len()), result);
                new_molecules.insert(new_molecule);
            }
        }
    }

    println!("{}", new_molecules.len());
}

fn parse_input(file_path: &String) -> (HashMap<String, HashSet<String>>, String) {
    let mut transitions = HashMap::new();

    let file = File::open(file_path).expect("File not found!");
    let mut line_iter = BufReader::new(file).lines();

    while let Some(Ok(line)) = line_iter.next() {
        if line.is_empty() {
            break;
        }

        let split_line = line.split(" => ").collect::<Vec<&str>>();
        transitions
            .entry(split_line[0].to_string())
            .and_modify(|results: &mut HashSet<String>| {
                results.insert(split_line[1].to_string());
            })
            .or_insert(HashSet::from([split_line[1].to_string()]));
    }

    let molecule = line_iter.next().expect("last line is molecule.").unwrap();

    (transitions, molecule)
}
