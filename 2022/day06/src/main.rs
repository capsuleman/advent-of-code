use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let mut file = File::open(file_path).expect("file not found!");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    for i in 14..(data.len() - 1) {
        if check_duplicate(&data[i - 14..i]) {
            println!("{} {}", i, at(&data, i));
            return Ok(());
        }
    }

    Ok(())
}

fn check_duplicate(string: &str) -> bool {
    for i in 0..string.len() {
        let current_letter = at(string, i);
        for j in i + 1..string.len() {
            if current_letter == at(string, j) {
                return false;
            }
        }
    }
    true
}

fn at(string: &str, index: usize) -> char {
    string.chars().nth(index).unwrap()
}
