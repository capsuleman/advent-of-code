use std::{
    env,
    fs::File,
    io::{BufReader, Read},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let mut content = String::new();
    BufReader::new(file)
        .read_to_string(&mut content)
        .expect("Read file");

    let floor = content
        .as_bytes()
        .into_iter()
        .fold(0, |floor, char| match char {
            b'(' => floor + 1,
            b')' => floor - 1,
            _ => panic!("Unknown char: {}", char),
        });

    println!("{floor}");
}
