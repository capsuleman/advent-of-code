use std::{
    env,
    fs::File,
    io::{BufReader, Read},
    process::exit,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("an input path given.");

    let file = File::open(file_path).expect("to find a file");
    let mut secret_key = String::new();
    BufReader::new(file)
        .read_to_string(&mut secret_key)
        .expect("to read file");

    for number_to_stick in 0..usize::MAX {
        let combination = format!("{secret_key}{number_to_stick}");
        let hash_string = format!("{:?}", md5::compute(combination));

        for (index, char) in hash_string.chars().enumerate() {
            if index >= 5 {
                println!("{number_to_stick}");
                exit(0);
            }
            if char != '0' {
                break;
            }
        }
    }
}
