use std::cmp::{max, min};
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: u32,
    y: u32,
}

const SAND_SOURCE: Position = Position { x: 500, y: 0 };

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let mut walls = HashSet::new();
    while let Some(Ok(line)) = line_iterator.next() {
        let corners: Vec<&str> = line.split(" -> ").collect();
        corners
            .iter()
            .zip(corners.iter().skip(1))
            .for_each(|(start_corner, end_corner)| {
                let start_position = parse_position(start_corner);
                let end_position = parse_position(end_corner);

                walls.extend(get_intermediate_positions(&start_position, &end_position));
            });
    }

    let deepest_wall = walls.iter().map(|wall| wall.y).max().unwrap();

    let mut sands: HashSet<Position> = HashSet::new();

    loop {
        let new_sand_position = get_new_sand_position(&walls, &sands, deepest_wall);

        if new_sand_position.is_none() {
            break;
        }

        sands.insert(new_sand_position.unwrap());
    }

    sands.insert(SAND_SOURCE);

    println!("{}", sands.len());
}

fn parse_position(corner_str: &str) -> Position {
    let coordinates: Vec<u32> = corner_str
        .split(",")
        .map(|coordinate| coordinate.parse::<u32>().unwrap())
        .collect();

    Position {
        x: coordinates[0],
        y: coordinates[1],
    }
}

fn get_intermediate_positions(start_position: &Position, end_position: &Position) -> Vec<Position> {
    if start_position.y == end_position.y {
        return (min(start_position.x, end_position.x)..max(start_position.x, end_position.x) + 1)
            .map(|x| Position {
                x,
                y: start_position.y,
            })
            .collect();
    }

    if start_position.x == end_position.x {
        return (min(start_position.y, end_position.y)..max(start_position.y, end_position.y) + 1)
            .map(|y| Position {
                x: start_position.x,
                y,
            })
            .collect();
    }

    panic!("Start and end position are not aligned.")
}

fn get_new_sand_position(
    sands: &HashSet<Position>,
    walls: &HashSet<Position>,
    deepest_wall: u32,
) -> Option<Position> {
    let mut sand_position = SAND_SOURCE.clone();

    loop {
        let next_sand_position = move_sand_one_step(&sand_position, sands, walls);

        if next_sand_position == SAND_SOURCE {
            return None;
        }

        if next_sand_position.y + 1 == deepest_wall + 2 {
            return Some(next_sand_position);
        }

        if next_sand_position == sand_position {
            return Some(sand_position);
        }

        sand_position = next_sand_position;
    }
}

fn move_sand_one_step(
    sand: &Position,
    sands: &HashSet<Position>,
    walls: &HashSet<Position>,
) -> Position {
    let bottom_position = Position {
        x: sand.x,
        y: sand.y + 1,
    };

    if !sands.contains(&bottom_position) && !walls.contains(&bottom_position) {
        return bottom_position;
    }

    let bottom_left_position = Position {
        x: sand.x - 1,
        y: sand.y + 1,
    };

    if !sands.contains(&bottom_left_position) && !walls.contains(&bottom_left_position) {
        return bottom_left_position;
    }

    let bottom_right_position = Position {
        x: sand.x + 1,
        y: sand.y + 1,
    };

    if !sands.contains(&bottom_right_position) && !walls.contains(&bottom_right_position) {
        return bottom_right_position;
    }

    sand.clone()
}
