use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn clockwise(self: &Direction) -> Direction {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }

    fn counter_clockwise(self: &Direction) -> Direction {
        match self {
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
        }
    }
}

#[derive(Debug)]
struct Path {
    direction: Direction,
    length: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let (board, paths) = parse_input(&file_path);

    let mut position = Position {
        x: board.iter().map(|(position, _)| position.x).min().unwrap(),
        y: 1,
    };

    let paths_len = paths.len();

    for (index, path) in paths.into_iter().enumerate() {
        for _ in 0..path.length {
            if let Some(next_position) = move_board(&position, &path.direction, &board) {
                position = next_position;
            } else {
                break;
            }
        }
        if index == paths_len - 1 {
            println!("Result: {}", get_result(&position, &path.direction));
        }
    }
}

fn parse_input(file_path: &String) -> (HashMap<Position, bool>, Vec<Path>) {
    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);
    let mut line_iterator = buf_reader.lines().enumerate();

    let mut board = HashMap::new();

    while let Some((y, Ok(line))) = line_iterator.next() {
        if line.is_empty() {
            break;
        }

        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                board.insert(Position { x: x + 1, y: y + 1 }, true);
            } else if char == '.' {
                board.insert(Position { x: x + 1, y: y + 1 }, false);
            }
        }
    }

    let (_, path_str) = line_iterator.next().unwrap();
    let paths = parse_path(path_str.unwrap());

    return (board, paths);
}

fn parse_path(path_str: String) -> Vec<Path> {
    let mut paths = Vec::new();
    let mut value_str = String::new();
    let mut direction = Direction::Right;

    for char in path_str.chars() {
        if char == 'R' || char == 'L' {
            let value = value_str.parse::<u64>().unwrap();
            paths.push(Path {
                length: value,
                direction: direction.clone(),
            });

            value_str = String::new();
            direction = match char {
                'R' => direction.clockwise(),
                'L' => direction.counter_clockwise(),
                _ => panic!("Unknown direction"),
            };
        } else {
            value_str.push(char);
        }
    }

    let value = value_str.parse::<u64>().unwrap();
    paths.push(Path {
        length: value,
        direction,
    });

    paths
}

fn move_board(
    current_position: &Position,
    direction: &Direction,
    board: &HashMap<Position, bool>,
) -> Option<Position> {
    let x = current_position.x;
    let y = current_position.y;
    let mut next_position = Position {
        x: match direction {
            Direction::Left => x - 1,
            Direction::Right => x + 1,
            _ => x,
        },
        y: match direction {
            Direction::Up => y - 1,
            Direction::Down => y + 1,
            _ => y,
        },
    };

    if !board.contains_key(&next_position) {
        next_position = find_gap_position(next_position, direction, board);
    };

    if *board.get(&next_position).unwrap() {
        None
    } else {
        Some(next_position)
    }
}

fn find_gap_position(
    position_before_gap: Position,
    direction: &Direction,
    board: &HashMap<Position, bool>,
) -> Position {
    let x = position_before_gap.x;
    let y = position_before_gap.y;

    if direction == &Direction::Left {
        let last_x = board
            .iter()
            .filter(|(position, _)| position.y == y)
            .map(|(position, _)| position.x)
            .max()
            .unwrap();
        Position { x: last_x, y }
    } else if direction == &Direction::Right {
        let first_x = board
            .iter()
            .filter(|(position, _)| position.y == y)
            .map(|(position, _)| position.x)
            .min()
            .unwrap();
        Position { x: first_x, y }
    } else if direction == &Direction::Up {
        let last_y = board
            .iter()
            .filter(|(position, _)| position.x == x)
            .map(|(position, _)| position.y)
            .max()
            .unwrap();
        Position { x, y: last_y }
    } else {
        let first_y = board
            .iter()
            .filter(|(position, _)| position.x == x)
            .map(|(position, _)| position.y)
            .min()
            .unwrap();
        Position { x, y: first_y }
    }
}

fn get_result(position: &Position, direction: &Direction) -> usize {
    let facing = match direction {
        Direction::Right => 0,
        Direction::Down => 1,
        Direction::Left => 2,
        Direction::Up => 3,
    };

    position.y * 1000 + position.x * 4 + facing
}
