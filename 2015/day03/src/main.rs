use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{BufReader, Read},
};

#[derive(PartialEq, Eq, Clone, Hash)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let mut content = String::new();
    BufReader::new(file)
        .read_to_string(&mut content)
        .expect("to read file");

    let mut current_position = Position { x: 0, y: 0 };
    let mut positions = HashSet::new();
    positions.insert(current_position.clone());

    for order in content.as_bytes().into_iter() {
        move_position(&mut current_position, order);
        positions.insert(current_position.clone());
    }

    println!("{}", positions.len());
}

fn move_position(position: &mut Position, order: &u8) {
    match order {
        b'<' => position.x -= 1,
        b'>' => position.x += 1,
        b'v' => position.y -= 1,
        b'^' => position.y += 1,
        _ => panic!("Unknown order: {}", order),
    };
}
