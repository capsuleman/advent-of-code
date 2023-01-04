use std::cmp::max;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use tqdm::tqdm;

#[derive(Debug)]
enum Wind {
    Left,
    Right,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position {
    x: u32,
    y: u32,
}

const NUMBER_OF_ROCKS: u32 = 2022;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");
    let winds = parse_winds(file_path);

    let mut wind_index: usize = 0;
    let mut rested_rocks: HashSet<Position> = HashSet::new();
    let mut max_y: u32 = 0;

    for rock_count in tqdm(0..NUMBER_OF_ROCKS) {
        let rock_origin = Position { x: 3, y: max_y + 4 };
        let mut rock = get_new_rock(rock_origin, rock_count);

        loop {
            let mut rock_after_wind = match winds[wind_index] {
                Wind::Left => move_rock_left(&rock),
                Wind::Right => move_rock_right(&rock),
            };
            wind_index += 1;
            wind_index %= winds.len();
            if has_collision(&rock_after_wind, &rested_rocks) {
                rock_after_wind = rock;
            }

            let rock_after_fall = move_rock_bottom(&rock_after_wind);
            if has_collision(&rock_after_fall, &rested_rocks) {
                max_y = max(
                    max_y,
                    rock_after_wind
                        .iter()
                        .map(|fragment| fragment.y)
                        .max()
                        .unwrap(),
                );
                rested_rocks.extend(rock_after_wind);
                break;
            }

            rock = rock_after_fall;
        }
    }

    let max_height = rested_rocks.into_iter().map(|rock| rock.y).max().unwrap();
    println!("Result: {max_height}");
}

fn parse_winds(file_path: &String) -> Vec<Wind> {
    let mut file = File::open(file_path).expect("file not found!");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");

    data.chars().into_iter().map(char_to_wind).collect()
}

fn char_to_wind(char: char) -> Wind {
    match char {
        '<' => Some(Wind::Left),
        '>' => Some(Wind::Right),
        _ => None,
    }
    .expect(&format!("Unrecognized character: '{char}'."))
}

fn get_new_rock(new_rock_origin: Position, rock_count: u32) -> Vec<Position> {
    let x = new_rock_origin.x;
    let y = new_rock_origin.y;

    match rock_count % 5 {
        0 => Vec::from([
            Position { x, y },
            Position { x: x + 1, y },
            Position { x: x + 2, y },
            Position { x: x + 3, y },
        ]),
        1 => Vec::from([
            Position { x: x + 1, y },
            Position { x, y: y + 1 },
            Position { x: x + 1, y: y + 1 },
            Position { x: x + 2, y: y + 1 },
            Position { x: x + 1, y: y + 2 },
        ]),
        2 => Vec::from([
            Position { x, y },
            Position { x: x + 1, y },
            Position { x: x + 2, y },
            Position { x: x + 2, y: y + 1 },
            Position { x: x + 2, y: y + 2 },
        ]),
        3 => Vec::from([
            Position { x, y },
            Position { x, y: y + 1 },
            Position { x, y: y + 2 },
            Position { x, y: y + 3 },
        ]),
        4 => Vec::from([
            Position { x, y },
            Position { x: x + 1, y },
            Position { x, y: y + 1 },
            Position { x: x + 1, y: y + 1 },
        ]),
        _ => todo!(),
    }
}

fn has_collision(rock: &Vec<Position>, rested_rocks: &HashSet<Position>) -> bool {
    rock.iter().any(|rock_frag| {
        rock_frag.x == 0 || rock_frag.x == 8 || rock_frag.y == 0 || rested_rocks.contains(rock_frag)
    })
}

fn move_rock_bottom(rock: &Vec<Position>) -> Vec<Position> {
    rock.iter()
        .map(|rock_frag| Position {
            x: rock_frag.x,
            y: rock_frag.y - 1,
        })
        .collect()
}

fn move_rock_left(rock: &Vec<Position>) -> Vec<Position> {
    rock.iter()
        .map(|rock_frag| Position {
            x: rock_frag.x - 1,
            y: rock_frag.y,
        })
        .collect()
}

fn move_rock_right(rock: &Vec<Position>) -> Vec<Position> {
    rock.iter()
        .map(|rock_frag| Position {
            x: rock_frag.x + 1,
            y: rock_frag.y,
        })
        .collect()
}

fn print_rested_rocks(rested_rocks: &HashSet<Position>, rock: &Vec<Position>) {
    let max_y = max(
        rested_rocks.iter().map(|rock| rock.y).max().unwrap_or(0),
        rock.iter().map(|fragment| fragment.y).max().unwrap_or(0),
    );

    for y in (1..max_y + 1).rev() {
        let row_content: String = (1..8)
            .map(|x| {
                let position = Position { x, y };
                if rested_rocks.contains(&position) {
                    '#'
                } else if rock.contains(&position) {
                    '@'
                } else {
                    '.'
                }
            })
            .collect();
        println!("|{row_content}|");
    }
    println!("+-------+");
    println!()
}
