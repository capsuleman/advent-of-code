use lazy_static::lazy_static;
use regex::Regex;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

const DURATION: u32 = 2503;

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(
        r"^([A-Za-z]+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$"
    )
    .unwrap();
}

struct Reindeer {
    fly_speed: u32,
    fly_duration: u32,
    rest_duration: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");

    let reindeers = parse_reindeers(&file_path);
    let mut best_distance = 0;

    for reindeer in reindeers {
        let reindeer_distance = get_reindeer_distance(&reindeer, DURATION);
        best_distance = u32::max(best_distance, reindeer_distance);
    }

    println!("{best_distance}");
}

fn parse_reindeers(file_path: &str) -> Vec<Reindeer> {
    let file = File::open(file_path).expect("File not found!");
    let mut line_iter = BufReader::new(file).lines();

    let mut reindeers = Vec::new();
    while let Some(Ok(line)) = line_iter.next() {
        let reindeer = parse_reindeer(&line);
        reindeers.push(reindeer);
    }

    reindeers
}

fn parse_reindeer(line: &str) -> Reindeer {
    let captures = INPUT_RE.captures(&line).expect("to parse line");
    let fly_speed = captures[2].parse::<u32>().expect("a number");
    let fly_duration = captures[3].parse::<u32>().expect("a number");
    let rest_duration = captures[4].parse::<u32>().expect("a number");

    Reindeer {
        fly_speed,
        fly_duration,
        rest_duration,
    }
}

fn get_reindeer_distance(reindeer: &Reindeer, duration: u32) -> u32 {
    let cycle_duration = reindeer.fly_duration + reindeer.rest_duration;
    let cycle_count = duration / cycle_duration;
    let complete_cycle_distance = cycle_count * reindeer.fly_speed * reindeer.fly_duration;
    let remain_distance =
        reindeer.fly_speed * u32::min(duration % cycle_duration, reindeer.fly_duration);

    complete_cycle_distance + remain_distance
}
