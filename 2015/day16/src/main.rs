use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

lazy_static! {
    static ref INPUT_RE: Regex =
        Regex::new(r"^Sue \d+: ([A-Za-z]+): (\d+), ([A-Za-z]+): (\d+), ([A-Za-z]+): (\d+)$")
            .unwrap();
    static ref TICKER_TAPE: HashMap<String, u32> = HashMap::from([
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1),
    ]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");

    let file = File::open(file_path).expect("File not found!");
    let mut line_iter = BufReader::new(file).lines();

    while let Some(Ok(line)) = line_iter.next() {
        if is_aunt_valid(&line) {
            println!("{line}");
        }
    }
}

fn is_aunt_valid(line: &String) -> bool {
    let captures = INPUT_RE.captures(&line).expect("to parse line");

    let name_1 = &captures[1];
    let name_2 = &captures[3];
    let name_3 = &captures[5];

    let value_1 = captures[2].parse::<u32>().expect("a number");
    let value_2 = captures[4].parse::<u32>().expect("a number");
    let value_3 = captures[6].parse::<u32>().expect("a number");

    check_criteria(name_1, value_1)
        && check_criteria(name_2, value_2)
        && check_criteria(name_3, value_3)
}

fn check_criteria(name: &str, value: u32) -> bool {
    match name {
        "cats" => TICKER_TAPE[name] < value,
        "trees" => TICKER_TAPE[name] < value,
        "pomeranians" => TICKER_TAPE[name] > value,
        "goldfish" => TICKER_TAPE[name] > value,
        _ => TICKER_TAPE[name] == value,
    }
}
