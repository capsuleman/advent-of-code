use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(
        r"^([A-Za-z]+) would (gain|lose) (\d+) happiness units by sitting next to ([A-Za-z]+).$"
    )
    .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");
    let (relations, names) = parse_input(file_path);

    let name_count = names.len();
    let mut max_love_count = i32::MIN;
    for permutation in names.into_iter().permutations(name_count).unique() {
        max_love_count = i32::max(
            max_love_count,
            get_love_count_from_permutation(permutation, &relations),
        );
    }
    println!("{max_love_count}");
}

fn parse_input(file_path: &String) -> (HashMap<(String, String), i32>, HashSet<String>) {
    let mut relations = HashMap::new();
    let me = String::from("me");
    let mut names = HashSet::from([me.clone()]);

    let file = File::open(file_path).expect("File not found!");
    let mut line_iter = BufReader::new(file).lines();

    while let Some(Ok(line)) = line_iter.next() {
        let captures = INPUT_RE.captures(&line).expect("to parse line");
        let name_1 = String::from(&captures[1]);
        let is_positive = match &captures[2] {
            "gain" => true,
            "lose" => false,
            _ => panic!("Should be 'gain' or 'lose' but is {}", &captures[2]),
        };
        let amount = captures[3].parse::<i32>().expect("a number");
        let amount = match is_positive {
            true => amount,
            false => -amount,
        };
        let name_2 = String::from(&captures[4]);

        names.insert(name_1.clone());
        names.insert(name_2.clone());
        relations.insert((name_1, name_2), amount);
    }

    for name in names.iter() {
        relations.insert((me.clone(), name.clone()), 0);
        relations.insert((name.clone(), me.clone()), 0);
    }

    (relations, names)
}

fn get_love_count_from_permutation(
    permutation: Vec<String>,
    relations: &HashMap<(String, String), i32>,
) -> i32 {
    let mut love_count = 0;
    for (name_1, name_2) in permutation.into_iter().circular_tuple_windows() {
        love_count += relations
            .get(&(name_1.clone(), name_2.clone()))
            .expect("to get love value");
        love_count += relations.get(&(name_2, name_1)).expect("to get love value");
    }
    love_count
}
