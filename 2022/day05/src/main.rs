use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut is_parsing_crates: bool = true;
    let mut raw_crates_lines: Vec<String> = Vec::new();
    let mut crate_stacks: Vec<Vec<char>> = Vec::new();
    let move_regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();

    for line in buf_reader.lines() {
        let line_value = line?;

        if is_parsing_crates {
            if !line_value.is_empty() {
                raw_crates_lines.push(line_value);
            } else {
                let crates_count = (raw_crates_lines[0].len() + 1) / 4;
                for _crate_index in 0..crates_count {
                    let crate_stack: Vec<char> = Vec::new();
                    crate_stacks.push(crate_stack);
                }

                raw_crates_lines.reverse();
                for crate_line in raw_crates_lines[1..].iter() {
                    for (index, crate_stack) in crate_stacks.iter_mut().enumerate() {
                        let crate_name = crate_line.chars().nth(4 * index + 1).unwrap();
                        if crate_name != ' ' {
                            crate_stack.push(crate_name);
                        }
                    }
                }

                is_parsing_crates = false;
            }
        } else {
            let captures = move_regex.captures(&line_value).unwrap();
            let move_count = captures[1].parse::<usize>().unwrap();
            let crate_origin_index = captures[2].parse::<usize>().unwrap() - 1;
            let crate_destination_index = captures[3].parse::<usize>().unwrap() - 1;

            println!("{}", line_value);
            println!(
                "{} -> {} x {}",
                crate_origin_index, crate_destination_index, move_count
            );

            let mut temporary_stack: Vec<char> = Vec::new();
            for _index in 0..move_count {
                temporary_stack.push(crate_stacks[crate_origin_index].pop().unwrap());
            }

            for _index in 0..move_count {
                crate_stacks[crate_destination_index].push(temporary_stack.pop().unwrap());
            }
        }
    }

    for crate_stack in crate_stacks.iter() {
        print!("{}", crate_stack[crate_stack.len() - 1]);
    }
    println!();

    Ok(())
}
