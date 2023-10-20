use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    line_number: usize,
    column_number: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let line_iterator = buf_reader.lines().into_iter();

    let mut start_position: Position = Position {
        line_number: 0,
        column_number: 0,
    };

    let altitudes: Vec<Vec<u32>> = line_iterator
        .enumerate()
        .map(|(line_number, line)| {
            line.expect("Issue reading line")
                .chars()
                .into_iter()
                .enumerate()
                .map(|(column_number, letter)| {
                    if letter == 'E' {
                        start_position = Position {
                            line_number,
                            column_number,
                        };
                    }
                    letter_to_altitude(letter)
                })
                .collect()
        })
        .collect();

    let step_count = find_minimal_step(&altitudes, start_position);
    println!("Result: {step_count}");
}

fn letter_to_altitude(letter: char) -> u32 {
    if letter == 'S' {
        return 0;
    }
    if letter == 'E' {
        return 25;
    }
    letter as u32 - 97
}

fn find_minimal_step(altitudes: &Vec<Vec<u32>>, start_position: Position) -> u32 {
    let mut current_positions: HashSet<Position> = HashSet::from([start_position]);
    let mut previous_positions: HashSet<Position> = HashSet::new();

    let max_line_number = altitudes.len();
    let max_column_number = altitudes[0].len();
    let mut step_count: u32 = 0;

    while !current_positions
        .iter()
        .any(|position| altitudes[position.line_number][position.column_number] == 0)
    {
        let mut new_current_positions: HashSet<Position> = HashSet::new();

        for current_position in current_positions.into_iter() {
            let current_position_altitude =
                altitudes[current_position.line_number][current_position.column_number];

            for neighbor in
                get_neighbors(&current_position, max_line_number, max_column_number).into_iter()
            {
                if previous_positions.contains(&neighbor) {
                    continue;
                }

                if altitudes[neighbor.line_number][neighbor.column_number]
                    >= current_position_altitude - 1
                {
                    new_current_positions.insert(neighbor);
                }
            }
            previous_positions.insert(current_position);
        }

        current_positions = new_current_positions;
        step_count += 1;
    }
    step_count
}

fn get_neighbors(
    position: &Position,
    max_line_number: usize,
    max_column_number: usize,
) -> Vec<Position> {
    let mut neighbors: Vec<Position> = Vec::new();

    let line_number = position.line_number;
    let column_number = position.column_number;

    if line_number > 0 {
        neighbors.push(Position {
            line_number: line_number - 1,
            column_number,
        });
    };

    if line_number < max_line_number - 1 {
        neighbors.push(Position {
            line_number: line_number + 1,
            column_number,
        });
    };

    if column_number > 0 {
        neighbors.push(Position {
            line_number,
            column_number: column_number - 1,
        });
    };

    if column_number < max_column_number - 1 {
        neighbors.push(Position {
            line_number,
            column_number: column_number + 1,
        });
    };

    neighbors
}
