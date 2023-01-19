use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Element {
    instruction: i64,
    index: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let mut encrypted_file = parse_encrypted_file(&file_path);

    for index in 0..encrypted_file.len() {
        execute_instruction(&mut encrypted_file, index);
    }

    println!("Result: {}", find_coordinates(&encrypted_file));
}

fn parse_encrypted_file(file_path: &String) -> Vec<Element> {
    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    buf_reader
        .lines()
        .into_iter()
        .enumerate()
        .map(|(index, line)| Element {
            instruction: line.unwrap().parse::<i64>().unwrap(),
            index,
        })
        .collect()
}

fn execute_instruction(encrypted_file: &mut Vec<Element>, index: usize) {
    let initial_position = encrypted_file
        .iter()
        .position(|element| element.index == index)
        .unwrap();

    let removed_element = encrypted_file.remove(initial_position);

    let next_position = (initial_position as i64 + removed_element.instruction)
        .rem_euclid(encrypted_file.len() as i64) as usize;

    if next_position == 0 && removed_element.instruction < 0 {
        encrypted_file.push(removed_element);
    } else {
        encrypted_file.insert(next_position, removed_element);
    }
}

fn find_coordinates(encrypted_file: &Vec<Element>) -> i64 {
    let zero_position = encrypted_file
        .iter()
        .position(|element| element.instruction == 0)
        .unwrap();

    let first_coordinate = (zero_position + 1000) % encrypted_file.len();
    let second_coordinate = (zero_position + 2000) % encrypted_file.len();
    let third_coordinate = (zero_position + 3000) % encrypted_file.len();

    encrypted_file[first_coordinate].instruction
        + encrypted_file[second_coordinate].instruction
        + encrypted_file[third_coordinate].instruction
}
