use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Operator {
    Add,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug)]
enum Monkey {
    Value(i64),
    Operation(String, String, Operator),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let monkeys = parse_monkeys(&file_path);

    let humn_lignee = get_humn_lignee(&String::from("root"), &monkeys);

    if let Monkey::Operation(monkey1, monkey2, _) = monkeys.get(&String::from("root")).unwrap() {
        let result = if humn_lignee.contains(monkey1) {
            compute_expected(
                compute_monkey(monkey2, &monkeys),
                monkey1,
                &monkeys,
                &humn_lignee,
            )
        } else {
            compute_expected(
                compute_monkey(monkey1, &monkeys),
                monkey2,
                &monkeys,
                &humn_lignee,
            )
        };
        println!("Result: {}", result);
    } else {
        panic!("Root is not an operation")
    };
}

fn parse_monkeys(file_path: &String) -> HashMap<String, Monkey> {
    let monkey_value_regex = Regex::new(r"^(\w{4}): (\d+)$").unwrap();
    let monkey_operation_regex = Regex::new(r"^(\w{4}): (\w{4}) (.) (\w{4})$").unwrap();

    let mut monkeys = HashMap::new();

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);
    let mut line_iterator = buf_reader.lines().into_iter();

    while let Some(Ok(line)) = line_iterator.next() {
        if let Some(value_captures) = monkey_value_regex.captures(&line) {
            monkeys.insert(
                String::from(&value_captures[1]),
                Monkey::Value(value_captures[2].parse::<i64>().unwrap()),
            );
        } else if let Some(operator_capture) = monkey_operation_regex.captures(&line) {
            monkeys.insert(
                String::from(&operator_capture[1]),
                Monkey::Operation(
                    String::from(&operator_capture[2]),
                    String::from(&operator_capture[4]),
                    match &operator_capture[3] {
                        "+" => Operator::Add,
                        "-" => Operator::Minus,
                        "*" => Operator::Multiply,
                        "/" => Operator::Divide,
                        _ => panic!("Unknown operator: {}", &operator_capture[2]),
                    },
                ),
            );
        } else {
            panic!("Could not parse line");
        }
    }

    monkeys
}

fn compute_monkey(monkey_name: &String, monkeys: &HashMap<String, Monkey>) -> i64 {
    let monkey = monkeys.get(monkey_name).unwrap();

    match monkey {
        Monkey::Value(value) => *value,
        Monkey::Operation(monkey1, monkey2, Operator::Add) => {
            compute_monkey(monkey1, monkeys) + compute_monkey(monkey2, monkeys)
        }
        Monkey::Operation(monkey1, monkey2, Operator::Minus) => {
            compute_monkey(monkey1, monkeys) - compute_monkey(monkey2, monkeys)
        }
        Monkey::Operation(monkey1, monkey2, Operator::Multiply) => {
            compute_monkey(monkey1, monkeys) * compute_monkey(monkey2, monkeys)
        }
        Monkey::Operation(monkey1, monkey2, Operator::Divide) => {
            compute_monkey(monkey1, monkeys) / compute_monkey(monkey2, monkeys)
        }
    }
}

fn get_humn_lignee(monkey_name: &String, monkeys: &HashMap<String, Monkey>) -> HashSet<String> {
    if monkey_name == "humn" {
        return HashSet::from([String::from("humn")]);
    }

    let monkey = monkeys.get(monkey_name).unwrap();

    match monkey {
        Monkey::Value(_) => HashSet::new(),
        Monkey::Operation(monkey1, monkey2, _) => {
            let mut humn_lignee_1 = get_humn_lignee(monkey1, monkeys);
            let mut humn_lignee_2 = get_humn_lignee(monkey2, monkeys);

            if humn_lignee_1.len() > 0 {
                humn_lignee_1.insert(monkey_name.clone());
                humn_lignee_1
            } else if humn_lignee_2.len() > 0 {
                humn_lignee_2.insert(monkey_name.clone());
                humn_lignee_2
            } else {
                HashSet::new()
            }
        }
    }
}

fn compute_expected(
    expected_value: i64,
    monkey_name: &String,
    monkeys: &HashMap<String, Monkey>,
    humn_lignee: &HashSet<String>,
) -> i64 {
    if monkey_name == "humn" {
        return expected_value;
    };

    let monkey = monkeys.get(monkey_name).unwrap();

    match monkey {
        Monkey::Value(_) => panic!(),
        Monkey::Operation(monkey1, monkey2, operator) => {
            if humn_lignee.contains(monkey1) {
                match operator {
                    Operator::Add => compute_expected(
                        expected_value - compute_monkey(monkey2, monkeys),
                        monkey1,
                        monkeys,
                        humn_lignee,
                    ),
                    Operator::Minus => compute_expected(
                        expected_value + compute_monkey(monkey2, monkeys),
                        monkey1,
                        monkeys,
                        humn_lignee,
                    ),
                    Operator::Multiply => compute_expected(
                        expected_value / compute_monkey(monkey2, monkeys),
                        monkey1,
                        monkeys,
                        humn_lignee,
                    ),
                    Operator::Divide => compute_expected(
                        expected_value * compute_monkey(monkey2, monkeys),
                        monkey1,
                        monkeys,
                        humn_lignee,
                    ),
                }
            } else {
                match operator {
                    Operator::Add => compute_expected(
                        expected_value - compute_monkey(monkey1, monkeys),
                        monkey2,
                        monkeys,
                        humn_lignee,
                    ),
                    Operator::Minus => compute_expected(
                        compute_monkey(monkey1, monkeys) - expected_value,
                        monkey2,
                        monkeys,
                        humn_lignee,
                    ),
                    Operator::Multiply => compute_expected(
                        expected_value / compute_monkey(monkey1, monkeys),
                        monkey2,
                        monkeys,
                        humn_lignee,
                    ),
                    Operator::Divide => compute_expected(
                        compute_monkey(monkey1, monkeys) / expected_value,
                        monkey2,
                        monkeys,
                        humn_lignee,
                    ),
                }
            }
        }
    }
}
