use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut reconsideration_count: usize = 0;

    for line in buf_reader.lines() {
        let line_value = line?;
        let (start1, end1, start2, end2) = parse_line(&line_value);

        if check_conflicts(start1, end1, start2, end2) {
            reconsideration_count += 1;
        }
    }

    println!("{}", reconsideration_count);
    Ok(())
}

fn parse_line(line_value: &str) -> (u32, u32, u32, u32) {
    let ranges: Vec<&str> = line_value.split(",").collect();
    let range1 = ranges[0];
    let range2 = ranges[1];
    let (start1, end1) = parse_range(range1);
    let (start2, end2) = parse_range(range2);
    (start1, end1, start2, end2)
}

fn parse_range(range: &str) -> (u32, u32) {
    let boundaries: Vec<&str> = range.split("-").collect();
    let start = boundaries[0].parse::<u32>().unwrap();
    let end = boundaries[1].parse::<u32>().unwrap();
    (start, end)
}

fn check_conflicts(start1: u32, end1: u32, start2: u32, end2: u32) -> bool {
    let max_start = std::cmp::max(start1, start2);
    let min_end = std::cmp::min(end1, end2);

    max_start <= min_end
}
