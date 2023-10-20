use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const TOTAL_SPACE: usize = 70000000;
const REQUIRED_SPACE: usize = 30000000;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let cd_regex = Regex::new(r"^\$ cd ([a-z]+)$").unwrap();
    let node_regex = Regex::new(r"^(\d+) (.+)$").unwrap();
    let dir_regex = Regex::new(r"^dir (.+)$").unwrap();

    let mut current_path = String::from("");

    let mut directory_size: HashMap<String, usize> = HashMap::new();

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    while let Some(Ok(line)) = line_iterator.next() {
        if line == "$ cd /" {
            current_path.push('/');
            directory_size.insert(String::from("/"), 0);
            continue;
        }

        if line == "$ cd .." {
            current_path.pop();
            while let Some(char) = current_path.pop() {
                if char == '/' {
                    break;
                }
            }
            current_path.push('/');
            continue;
        }

        if cd_regex.is_match(&line) {
            let cd_captures = cd_regex.captures(&line).expect("Issue capturing cd regexp");
            let directory_name = String::from(&cd_captures[1]);

            current_path.push_str(&directory_name);
            current_path.push('/');
            continue;
        }

        if dir_regex.is_match(&line) {
            let dir_captures = dir_regex
                .captures(&line)
                .expect("Issue capturing dir regexp");
            let directory_name = String::from(&dir_captures[1]);

            let directory_path = format!("{current_path}{directory_name}/");
            directory_size.insert(directory_path, 0);
            continue;
        }

        if node_regex.is_match(&line) {
            let node_captures = node_regex
                .captures(&line)
                .expect("Issue capturing node regexp");

            let size = node_captures[1]
                .parse::<usize>()
                .expect("Can not parse node size");

            let mut new_directory_path = HashMap::new();
            for (directory_path, directory_size) in directory_size {
                let mut new_size = directory_size;
                if current_path.starts_with(&directory_path) {
                    new_size += size;
                }
                new_directory_path.insert(directory_path, new_size);
            }
            directory_size = new_directory_path;
            continue;
        }
    }

    let used_size = *directory_size.get("/").expect("Root not found");
    let free_space = TOTAL_SPACE - used_size;
    let perfect_space_to_free = REQUIRED_SPACE - free_space;

    let optimized_space_to_free = directory_size
        .values()
        .into_iter()
        .filter(|&&size| size > perfect_space_to_free)
        .min()
        .unwrap();

    println!("{directory_size:#?}");

    println!("Response: {optimized_space_to_free:#?}");
}
