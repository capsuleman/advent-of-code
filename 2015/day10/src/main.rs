use std::{
    char, env,
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");

    let file = File::open(file_path).expect("to find a file");
    let mut sequence = String::new();
    BufReader::new(file)
        .read_to_string(&mut sequence)
        .expect("to read file");

    for _ in 0..50 {
        sequence = look_and_say(sequence);
    }

    println!("{}", sequence.len());
}

fn look_and_say(current: String) -> String {
    let mut next = String::new();
    let mut chars = current.chars();

    let mut counted_char = chars.next().expect("at least one char");
    let mut char_count = 1;

    while let Some(current_char) = chars.next() {
        if current_char == counted_char {
            char_count += 1;
            continue;
        }

        next.push(char::from_digit(char_count, 10).expect("to be <= 3"));
        next.push(counted_char);

        counted_char = current_char;
        char_count = 1;
    }
    next.push(char::from_digit(char_count, 10).expect("to be <= 3"));
    next.push(counted_char);

    next
}
