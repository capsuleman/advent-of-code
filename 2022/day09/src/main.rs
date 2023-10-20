use std::cmp::max;
use std::cmp::min;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const ROPE_LENGTH: usize = 10;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let mut rope = [Position { x: 0, y: 0 }; ROPE_LENGTH];

    let mut tail_positions_set: HashSet<Position> = HashSet::new();
    tail_positions_set.insert(rope[ROPE_LENGTH - 1].clone());

    while let Some(Ok(line)) = line_iterator.next() {
        let parameters = line.split(' ').collect::<Vec<&str>>();
        let direction = parameters[0];
        let direction = match direction {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
        .expect("Could not parse direction");
        let steps = parameters[1]
            .parse::<u64>()
            .expect("Could not parse number of step");

        for _ in 0..steps {
            move_rope(&mut rope, &direction);
            tail_positions_set.insert(rope[ROPE_LENGTH - 1].clone());
        }
        // println!("{direction:?} {steps}");
        // pretty_rope(&rope);
    }
    println!("Result: {}", tail_positions_set.len());
}

fn move_rope(rope: &mut [Position; ROPE_LENGTH], direction: &Direction) {
    let mut new_head_position = move_head_rope(&rope[0], direction);
    rope[0] = new_head_position;

    for i in 1..ROPE_LENGTH {
        let tail_position = rope[i];
        let new_tail_position = move_tail_rope(&new_head_position, tail_position.clone());
        rope[i] = new_tail_position;
        new_head_position = new_tail_position;
    }
}

fn move_head_rope(head_position: &Position, direction: &Direction) -> Position {
    let head_x = head_position.x;
    let head_y = head_position.y;

    match direction {
        Direction::Up => Position {
            x: head_x,
            y: head_y + 1,
        },
        Direction::Down => Position {
            x: head_x,
            y: head_y - 1,
        },
        Direction::Left => Position {
            x: head_x - 1,
            y: head_y,
        },
        Direction::Right => Position {
            x: head_x + 1,
            y: head_y,
        },
    }
}

fn move_tail_rope(new_head_position: &Position, tail_position: Position) -> Position {
    let distance = get_tchebychev_distance(new_head_position, &tail_position);

    if distance <= 1 {
        return tail_position;
    }

    Position {
        x: match new_head_position.x - tail_position.x {
            d if d > 0 => tail_position.x + 1,
            d if d < 0 => tail_position.x - 1,
            _ => tail_position.x,
        },
        y: match new_head_position.y - tail_position.y {
            d if d > 0 => tail_position.y + 1,
            d if d < 0 => tail_position.y - 1,
            _ => tail_position.y,
        },
    }
}

fn get_tchebychev_distance(position_a: &Position, position_b: &Position) -> u64 {
    let distance_x = position_b.x.abs_diff(position_a.x);
    let distance_y = position_b.y.abs_diff(position_a.y);

    if distance_x > distance_y {
        distance_x
    } else {
        distance_y
    }
}

fn pretty_rope(rope: &[Position; ROPE_LENGTH]) {
    let (min_x, max_x, min_y, max_y) =
        rope.iter()
            .fold((-10, 10, -10, 10), |(min_x, max_x, min_y, max_y), knot| {
                (
                    min(min_x, knot.x),
                    max(max_x, knot.x),
                    min(min_y, knot.y),
                    max(max_y, knot.y),
                )
            });

    let mut grid: Vec<Vec<char>> = (min_y..max_y + 1)
        .into_iter()
        .map(|y| {
            (min_x..max_x + 1)
                .into_iter()
                .map(|x| if x == 0 && y == 0 { 's' } else { '.' })
                .collect()
        })
        .collect();

    for (index, knot) in rope.iter().enumerate().rev() {
        let x: usize = usize::try_from(knot.x - min_x).unwrap();
        let y: usize = usize::try_from(knot.y - min_y).unwrap();

        grid[y][x] = match index {
            0 => 'H',
            _ => char::from_digit(index as u32, 10).unwrap(),
        };
    }

    for line in grid.into_iter().rev() {
        println!("{}", line.iter().collect::<String>());
    }
    println!();
}
