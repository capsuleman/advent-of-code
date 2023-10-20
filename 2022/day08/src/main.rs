use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let forest: Vec<Vec<u32>> = buf_reader
        .lines()
        .into_iter()
        .map(|line| {
            line.unwrap()
                .chars()
                .into_iter()
                .map(|char| char.to_digit(10).expect("Can not parse digit"))
                .collect::<Vec<u32>>()
        })
        .collect();

    let forest_length = forest.len();
    let forest_width = forest[0].len();

    let mut scenic_scores = Vec::new();

    for length in 0..forest_length {
        for width in 0..forest_width {
            scenic_scores.push(get_scenic_score(&forest, length, width));
        }
    }

    println!("{}", scenic_scores.into_iter().max().unwrap());
}

fn get_scenic_score(forest: &Vec<Vec<u32>>, length: usize, width: usize) -> usize {
    get_view_from_left(forest, length, width)
        * get_view_from_right(forest, length, width)
        * get_view_from_top(forest, length, width)
        * get_view_from_bottom(forest, length, width)
}

fn get_view_from_left(forest: &Vec<Vec<u32>>, length: usize, width: usize) -> usize {
    for i in (0..width).rev() {
        if forest[length][i] >= forest[length][width] {
            return width - i;
        }
    }
    width
}

fn get_view_from_right(forest: &Vec<Vec<u32>>, length: usize, width: usize) -> usize {
    let forest_width = forest[0].len();
    for i in (width + 1)..forest_width {
        if forest[length][i] >= forest[length][width] {
            return i - width;
        }
    }
    forest_width - width - 1
}

fn get_view_from_top(forest: &Vec<Vec<u32>>, length: usize, width: usize) -> usize {
    for i in (0..length).rev() {
        if forest[i][width] >= forest[length][width] {
            return length - i;
        }
    }
    length
}

fn get_view_from_bottom(forest: &Vec<Vec<u32>>, length: usize, width: usize) -> usize {
    let forest_length = forest.len();
    for i in (length + 1)..forest_length {
        if forest[i][width] >= forest[length][width] {
            return i - length;
        }
    }
    forest_length - length - 1
}
