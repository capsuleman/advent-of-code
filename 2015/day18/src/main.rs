use std::{
    collections::HashSet,
    env,
    fmt::{self, Display},
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Grid {
    positions_on: HashSet<Position>,
    size: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for x in 0..self.size {
            let line = (0..self.size)
                .map(|y| {
                    if self.positions_on.contains(&Position { x, y }) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect::<String>();
            if let Err(error) = write!(f, "{}\n", line) {
                return Err(error);
            };
        }

        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");

    let mut grid = parse_grid(&file_path);

    for _ in 0..100 {
        grid = next_step(grid);
    }
    println!("{}", grid);
    println!("{}", grid.positions_on.len());
}

fn parse_grid(file_path: &String) -> Grid {
    let mut grid = HashSet::new();
    let mut last_x = 0;

    let file = File::open(file_path).expect("File not found!");
    let mut line_iter = BufReader::new(file).lines().enumerate();

    while let Some((x, Ok(line))) = line_iter.next() {
        for (y, char) in line.chars().enumerate() {
            if char == '#' {
                grid.insert(Position { x, y });
            }
        }
        last_x = x;
    }

    Grid {
        positions_on: grid,
        size: last_x + 1,
    }
}

fn next_step(grid: Grid) -> Grid {
    let mut next_positions_on = HashSet::new();

    for x in 0..grid.size {
        for y in 0..grid.size {
            let current_position = Position { x, y };

            if is_light_on_next_grid(&current_position, &grid) {
                next_positions_on.insert(current_position);
            }
        }
    }

    Grid {
        positions_on: next_positions_on,
        size: grid.size,
    }
}

fn is_light_on_next_grid(position: &Position, grid: &Grid) -> bool {
    let neighbors = get_neighbors(&position);

    let turn_on_count = neighbors
        .into_iter()
        .filter(|neighbor| grid.positions_on.contains(neighbor))
        .count();

    if grid.positions_on.contains(position) {
        turn_on_count == 2 || turn_on_count == 3
    } else {
        turn_on_count == 3
    }
}

fn get_neighbors(position: &Position) -> HashSet<Position> {
    let mut neighbors = HashSet::from([
        Position {
            x: position.x + 1,
            y: position.y,
        },
        Position {
            x: position.x,
            y: position.y + 1,
        },
        Position {
            x: position.x + 1,
            y: position.y + 1,
        },
    ]);

    if position.x > 0 && position.y > 0 {
        neighbors.insert(Position {
            x: position.x - 1,
            y: position.y - 1,
        });
    }

    if position.x > 0 {
        neighbors.extend([
            Position {
                x: position.x - 1,
                y: position.y,
            },
            Position {
                x: position.x - 1,
                y: position.y + 1,
            },
        ]);
    }

    if position.y > 0 {
        neighbors.extend([
            Position {
                x: position.x,
                y: position.y - 1,
            },
            Position {
                x: position.x + 1,
                y: position.y - 1,
            },
        ]);
    }

    neighbors
}
