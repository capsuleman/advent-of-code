use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let mut content = String::new();
    BufReader::new(file)
        .read_to_string(&mut content)
        .expect("Read file");

    let mut floor: i32 = 0;
    for (index, char) in content.as_bytes().into_iter().enumerate() {
        floor = match char {
            b'(' => floor + 1,
            b')' => floor - 1,
            _ => panic!("Unknown char: {}", char),
        };

        if floor < 0 {
            println!("{}", index + 1);
            exit(0);
        }
    }

    panic!("Arrived to the end of input...");
}
