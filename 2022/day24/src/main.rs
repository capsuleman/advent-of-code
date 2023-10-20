use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Copy, Clone)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    line_number: usize,
    column_number: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd)]
enum Journey {
    GO,
    COMEBACK,
    GOBACK,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct PositionWithJourney {
    position: Position,
    journey: Journey,
}

#[derive(Debug)]
struct Blizzard {
    position: Position,
    direction: Direction,
}

struct InitialInput {
    blizzards: Vec<Blizzard>,
    line_number_max: usize,
    column_number_max: usize,
}

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let initial_input = parse_initial_blizzard(file_path);

    let mut blizzards = initial_input.blizzards;
    let line_number_max = initial_input.line_number_max;
    let column_number_max = initial_input.column_number_max;

    println!("{line_number_max} / {column_number_max}");

    let mut count: usize = 0;
    let mut positions_with_journey = vec![PositionWithJourney {
        position: Position {
            line_number: 0,
            column_number: 1,
        },
        journey: Journey::GO,
    }];

    loop {
        blizzards = get_next_blizzards(&blizzards, line_number_max, column_number_max);

        positions_with_journey =
            get_all_neighbors(&positions_with_journey, line_number_max, column_number_max)
                .into_iter()
                .filter(|position_with_journey| {
                    !is_position_taken(&blizzards, &position_with_journey.position)
                })
                .collect();

        count += 1;
        println!("{count}\t{}", positions_with_journey.len());

        if positions_with_journey.iter().any(|position_with_journey| {
            position_with_journey.journey == Journey::GOBACK
                && is_end_position(
                    &position_with_journey.position,
                    line_number_max,
                    column_number_max,
                )
        }) {
            break;
        }
    }

    println!("{count}");

    Ok(())
}

fn parse_initial_blizzard(filename: &str) -> InitialInput {
    let file = File::open(filename).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut blizzards: Vec<Blizzard> = Vec::new();
    let mut line_number_max = 0;
    let mut column_number_max = 0;

    for (line_number, line) in buf_reader.lines().enumerate() {
        let line_value = line.unwrap();
        line_number_max = line_number;

        for (column_number, char_value) in line_value.chars().enumerate() {
            column_number_max = column_number;
            let direction: Option<Direction> = match char_value {
                '>' => Some(Direction::RIGHT),
                '<' => Some(Direction::LEFT),
                '^' => Some(Direction::UP),
                'v' => Some(Direction::DOWN),
                _ => None,
            };

            if direction.is_none() {
                continue;
            }

            let blizzard = Blizzard {
                position: Position {
                    line_number,
                    column_number,
                },
                direction: direction.unwrap(),
            };

            blizzards.push(blizzard);
        }
    }

    InitialInput {
        blizzards,
        line_number_max,
        column_number_max,
    }
}

fn get_next_blizzards(
    blizzards: &Vec<Blizzard>,
    line_number_max: usize,
    column_number_max: usize,
) -> Vec<Blizzard> {
    blizzards
        .iter()
        .map(|blizzard| {
            let line_number = blizzard.position.line_number;
            let column_number = blizzard.position.column_number;

            let position = match blizzard.direction {
                Direction::UP => Position {
                    line_number: 1
                        + (line_number_max - 1 + line_number - 1 - 1) % (line_number_max - 1),
                    column_number,
                },
                Direction::DOWN => Position {
                    line_number: 1
                        + (line_number_max - 1 + line_number - 1 + 1) % (line_number_max - 1),
                    column_number,
                },
                Direction::LEFT => Position {
                    line_number,
                    column_number: 1
                        + (column_number_max - 1 + column_number - 1 - 1) % (column_number_max - 1),
                },
                Direction::RIGHT => Position {
                    line_number,
                    column_number: 1
                        + (column_number_max - 1 + column_number - 1 + 1) % (column_number_max - 1),
                },
            };

            Blizzard {
                position,
                direction: blizzard.direction,
            }
        })
        .collect()
}

fn is_position_taken(blizzards: &Vec<Blizzard>, position: &Position) -> bool {
    blizzards.iter().any(|blizzard| {
        blizzard.position.line_number == position.line_number
            && blizzard.position.column_number == position.column_number
    })
}

fn is_start_position(position: &Position) -> bool {
    position.line_number == 0 && position.column_number == 1
}

fn is_end_position(position: &Position, line_number_max: usize, column_number_max: usize) -> bool {
    position.line_number == line_number_max && position.column_number == column_number_max - 1
}

fn get_neighbors_position(
    position: Position,
    line_number_max: usize,
    column_number_max: usize,
) -> Vec<Position> {
    if is_start_position(&position) {
        return vec![
            Position {
                line_number: 0,
                column_number: 1,
            },
            Position {
                line_number: 1,
                column_number: 1,
            },
        ];
    }

    let line_number = position.line_number;
    let column_number = position.column_number;

    vec![
        Position {
            line_number,
            column_number,
        },
        Position {
            line_number: line_number + 1,
            column_number,
        },
        Position {
            line_number,
            column_number: column_number + 1,
        },
        Position {
            line_number: line_number - 1,
            column_number,
        },
        Position {
            line_number,
            column_number: column_number - 1,
        },
    ]
    .into_iter()
    .filter(|position| {
        // Start + End
        if is_start_position(position)
            || is_end_position(position, line_number_max, column_number_max)
        {
            return true;
        }

        1 <= position.line_number
            && position.line_number < line_number_max
            && 1 <= position.column_number
            && position.column_number < column_number_max
    })
    .collect()
}

fn get_neighbors(
    position_with_journey: PositionWithJourney,
    line_number_max: usize,
    column_number_max: usize,
) -> Vec<PositionWithJourney> {
    let journey = position_with_journey.journey;

    get_neighbors_position(
        position_with_journey.position,
        line_number_max,
        column_number_max,
    )
    .into_iter()
    .map(|position| {
        if is_start_position(&position) && journey == Journey::COMEBACK {
            return PositionWithJourney {
                position,
                journey: Journey::GOBACK,
            };
        }

        if is_end_position(&position, line_number_max, column_number_max) && journey == Journey::GO
        {
            return PositionWithJourney {
                position,
                journey: Journey::COMEBACK,
            };
        }

        PositionWithJourney { position, journey }
    })
    .collect()
}

fn get_all_neighbors(
    positions_with_journey: &Vec<PositionWithJourney>,
    line_number_max: usize,
    column_number_max: usize,
) -> Vec<PositionWithJourney> {
    positions_with_journey
        .into_iter()
        .flat_map(|&position_with_journey| {
            get_neighbors(position_with_journey, line_number_max, column_number_max)
        })
        .unique()
        .collect()
}
