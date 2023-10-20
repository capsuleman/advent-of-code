use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut total_priority = 0;

    let mut line_iterator = buf_reader.lines().into_iter();

    while let (Some(first_line), Some(second_line), Some(third_line)) = (
        line_iterator.next(),
        line_iterator.next(),
        line_iterator.next(),
    ) {
        let first_elf = first_line?;
        let second_elf = second_line?;
        let third_elf = third_line?;

        let duplicate = find_duplicate(&first_elf, &second_elf, &third_elf);
        total_priority += get_letter_score(duplicate);
    }

    println!("{}", total_priority);
    Ok(())
}

fn find_duplicate(first_part: &str, second_part: &str, third_part: &str) -> char {
    for element in first_part.chars().into_iter() {
        if second_part.contains(element) && third_part.contains(element) {
            return element;
        }
    }

    '0'
}

fn get_letter_score(letter: char) -> usize {
    ALPHABET
        .chars()
        .into_iter()
        .position(|r| r == letter)
        .unwrap()
        + 1
}
