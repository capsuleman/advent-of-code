use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file_path = "real.txt";

    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut accumulator = 0;

    for line in buf_reader.lines() {
        let line_value = line.expect("Issue reading a line");

        accumulator += snafu_to_decimal(line_value);
    }

    println!("{accumulator} -> {}", decimal_to_snafu(accumulator));
}

fn snafu_part_to_digit(snafu_part: char) -> i64 {
    let digit = match snafu_part {
        '0' => Some(0),
        '1' => Some(1),
        '2' => Some(2),
        '-' => Some(-1),
        '=' => Some(-2),
        _ => None,
    };
    digit.expect("Unknown SNAFU")
}

fn snafu_to_decimal(snafu: String) -> i64 {
    let mut decimal = 0;
    let mut exponent = 1;

    for snafu_part in snafu.chars().rev() {
        decimal += exponent * snafu_part_to_digit(snafu_part);
        exponent *= 5;
    }

    decimal
}

fn digit_to_snafu_part(digit: i64) -> char {
    let snafu_part = match digit {
        -2 => Some('='),
        -1 => Some('-'),
        0 => Some('0'),
        1 => Some('1'),
        2 => Some('2'),
        _ => None,
    };
    snafu_part.expect("Unknown digit")
}

fn decimal_to_snafu(decimal: i64) -> String {
    let mut remain = decimal;
    let mut stafu: Vec<char> = Vec::new();

    while remain != 0 {
        let digit = (remain + 2) % 5 - 2;
        stafu.push(digit_to_snafu_part(digit));
        remain -= digit;
        remain /= 5;
    }

    stafu.reverse();
    stafu.into_iter().collect()
}
