use regex::Regex;
use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use tqdm::tqdm;

const TUNING_FREQUENCY: u128 = 4000000;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: u32,
    end: u32,
}

impl Interval {
    pub fn intersect(&self, other: &Self) -> bool {
        let max_start = std::cmp::max(self.start, other.start);
        let min_end = std::cmp::min(self.end, other.end);

        max_start <= min_end
    }

    pub fn exclusive_union(&self, other: &Self) -> Vec<Interval> {
        if !self.intersect(other) {
            return Vec::from([other.clone()]);
        }

        let mut exclusive_union_vec: Vec<Interval> = Vec::new();
        if other.end >= self.start && self.start > other.start {
            exclusive_union_vec.push(Interval {
                start: other.start,
                end: self.start - 1,
            })
        }
        if other.start <= self.end && self.end < other.end {
            exclusive_union_vec.push(Interval {
                start: self.end + 1,
                end: other.end,
            })
        }

        exclusive_union_vec
    }

    pub fn len(&self) -> u32 {
        self.end - self.start + 1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");
    let max_coordinates: u32 = args
        .get(2)
        .expect("No max coordinates given.")
        .parse::<u32>()
        .unwrap();

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let parser_regex = Regex::new(
        r"^Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)$",
    )
    .unwrap();

    let mut impossible_beacon_intervals_list: Vec<Vec<Interval>> =
        (0..max_coordinates).map(|_| Vec::new()).collect();

    while let Some(Ok(line)) = line_iterator.next() {
        println!("{line}");
        let regex_captures = parser_regex
            .captures(&line)
            .expect("Issue capturing input.");

        let sensor_position = Position {
            x: regex_captures[1].parse::<i32>().unwrap(),
            y: regex_captures[2].parse::<i32>().unwrap(),
        };
        let beacon_position = Position {
            x: regex_captures[3].parse::<i32>().unwrap(),
            y: regex_captures[4].parse::<i32>().unwrap(),
        };

        for (depth_search, impossible_beacon_intervals) in
            tqdm(impossible_beacon_intervals_list.iter_mut().enumerate())
        {
            let distance_sensor_beacon = get_manhattan_distance(&sensor_position, &beacon_position);
            let distance_sensor_depth = (depth_search as i32).abs_diff(sensor_position.y);

            if distance_sensor_depth > distance_sensor_beacon {
                continue;
            }

            let diff_distances = (distance_sensor_beacon - distance_sensor_depth) as i32;

            let start = u32::try_from(sensor_position.x - diff_distances).unwrap_or(0);
            let end = min(max_coordinates, (sensor_position.x + diff_distances) as u32);

            if start > max_coordinates {
                continue;
            }

            let mut intervals_to_insert = Vec::from([Interval { start, end }]);

            for impossible_beacon_interval in impossible_beacon_intervals.iter() {
                let mut new_intervals_to_insert: Vec<Interval> = Vec::new();
                for interval_to_insert in intervals_to_insert.into_iter() {
                    let exclusive_union =
                        impossible_beacon_interval.exclusive_union(&interval_to_insert);

                    new_intervals_to_insert.extend(exclusive_union);
                }
                intervals_to_insert = new_intervals_to_insert;
            }

            impossible_beacon_intervals.extend(intervals_to_insert);
        }
    }

    impossible_beacon_intervals_list
        .iter()
        .enumerate()
        .for_each(|(y, impossible_beacon_intervals)| {
            let impossible_beacon_count = impossible_beacon_intervals
                .iter()
                .map(|interval| interval.len())
                .sum::<u32>();

            if impossible_beacon_count < max_coordinates + 1 {
                for x in 0..max_coordinates + 1 {
                    let contains_x = impossible_beacon_intervals
                        .iter()
                        .any(|interval| interval.start <= x && x <= interval.end);
                    if !contains_x {
                        println!(
                            "Missing beacon at {x}, {y} -> {}",
                            x as u128 * TUNING_FREQUENCY + y as u128
                        );
                    }
                }
            }
        });
}

fn get_manhattan_distance(position_a: &Position, position_b: &Position) -> u32 {
    position_b.x.abs_diff(position_a.x) + position_b.y.abs_diff(position_a.y)
}
