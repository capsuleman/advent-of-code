use core::panic;
use itertools::Itertools;
use kdam::tqdm;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
enum Order {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug)]
struct Command {
    order: Order,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

lazy_static! {
    static ref COMMAND_RE: Regex =
        Regex::new(r"^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$").unwrap();
}

const MAX_X: usize = 999;
const MAX_Y: usize = 999;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");
    let commands = parse_commands(file_path);

    let mut total_brightness: usize = 0;

    for (x, y) in tqdm!((0..MAX_X + 1).cartesian_product(0..MAX_Y + 1)) {
        total_brightness += get_position_brightness(x, y, &commands);
    }

    println!("{}", total_brightness);
}

fn get_position_brightness(x: usize, y: usize, commands: &Vec<Command>) -> usize {
    let mut brightness = 0;

    for command in commands.iter() {
        if !is_inside_command(x, y, command) {
            continue;
        };
        brightness = match command.order {
            Order::TurnOn => brightness + 1,
            Order::Toggle => brightness + 2,
            Order::TurnOff => usize::max(1, brightness) - 1,
        }
    }

    brightness
}

fn is_inside_command(x: usize, y: usize, command: &Command) -> bool {
    command.start_x <= x && x <= command.end_x && command.start_y <= y && y <= command.end_y
}

fn parse_command(line: &String) -> Command {
    let captures = COMMAND_RE.captures(&line).unwrap();
    let order = match &captures[1] {
        "turn on" => Order::TurnOn,
        "turn off" => Order::TurnOff,
        "toggle" => Order::Toggle,
        _ => panic!("Unknown order: {}", &captures[1]),
    };
    let start_x = captures[2].parse::<usize>().expect("to convert to usize");
    let start_y = captures[3].parse::<usize>().expect("to convert to usize");
    let end_x = captures[4].parse::<usize>().expect("to convert to usize");
    let end_y = captures[5].parse::<usize>().expect("to convert to usize");

    Command {
        order,
        start_x,
        start_y,
        end_x,
        end_y,
    }
}

fn parse_commands(file_path: &String) -> Vec<Command> {
    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let commands: Vec<Command> = buf_reader
        .lines()
        .map(|line| parse_command(&line.expect("a new line")))
        .collect();

    commands
}
