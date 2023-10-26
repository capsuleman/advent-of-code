use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut good_word_count: usize = 0;

    for line in buf_reader.lines() {
        let line = line.expect("to read line");

        if check_word(&line) {
            good_word_count += 1;
            println!("{line}");
        };
    }
    println!("{good_word_count}")
}

fn check_word(word: &str) -> bool {
    check_vowels_count(word) && check_double(word) && check_weird_string(word)
}

fn check_vowels_count(word: &str) -> bool {
    let vowels_count = word
        .chars()
        .filter(|&letter| {
            letter == 'a' || letter == 'e' || letter == 'i' || letter == 'o' || letter == 'u'
        })
        .collect::<Vec<char>>()
        .len();

    vowels_count >= 3
}

fn check_double(word: &str) -> bool {
    let mut word_iter = word.chars().into_iter();
    let mut previous_letter = word_iter.next().expect("the first letter");
    for letter in word_iter {
        if letter == previous_letter {
            return true;
        }

        previous_letter = letter;
    }
    false
}

fn check_weird_string(word: &str) -> bool {
    !word.contains("ab") && !word.contains("cd") && !word.contains("pq") && !word.contains("xy")
}
