use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const NUMBER_OF_ROUNDS: usize = 10000;

#[derive(Debug)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug)]
enum OperationMember {
    Constant(i64),
    Old,
}

#[derive(Debug)]
struct Operation {
    left_member: OperationMember,
    operator: Operator,
    right_member: OperationMember,
}

#[derive(Debug)]
struct Monkey {
    item_list: VecDeque<i64>,
    operation: Operation,
    divisible_value_test: i64,
    target_true: usize,
    target_false: usize,
    inspect_count: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_information: Vec<String> = Vec::new();
    let mut monkey_index: usize = 0;
    while let Some(Ok(line)) = line_iterator.next() {
        if !line.is_empty() {
            monkey_information.push(line);
        } else {
            let monkey = parse_monkey(monkey_information);
            assert_ne!(
                monkey_index, monkey.target_true,
                "Monkey target true could not be itself"
            );
            assert_ne!(
                monkey_index, monkey.target_false,
                "Monkey target false could not be itself"
            );
            monkeys.push(monkey);
            monkey_information = Vec::new();
            monkey_index += 1;
        }
    }

    let monkey_common_multiple = monkeys.iter().fold(1, |common_multiple, monkey| {
        common_multiple * monkey.divisible_value_test
    });

    for _ in 0..NUMBER_OF_ROUNDS {
        for monkey_index in 0..monkeys.len() {
            let mut items_to_append_true: VecDeque<i64> = VecDeque::new();
            let mut items_to_append_false: VecDeque<i64> = VecDeque::new();
            let monkey_target_true;
            let monkey_target_false;

            {
                let monkey = &mut monkeys[monkey_index];
                monkey_target_true = monkey.target_true;
                monkey_target_false = monkey.target_false;

                while let Some(mut item_value) = monkey.item_list.pop_front() {
                    monkey.inspect_count += 1;

                    let left_member = match monkey.operation.left_member {
                        OperationMember::Old => item_value,
                        OperationMember::Constant(constant) => constant,
                    };
                    let right_member = match monkey.operation.right_member {
                        OperationMember::Old => item_value,
                        OperationMember::Constant(constant) => constant,
                    };
                    item_value = match monkey.operation.operator {
                        Operator::Add => left_member + right_member,
                        Operator::Multiply => left_member * right_member,
                    };
                    item_value %= monkey_common_multiple;

                    if item_value % monkey.divisible_value_test == 0 {
                        items_to_append_true.push_back(item_value);
                    } else {
                        items_to_append_false.push_back(item_value);
                    }
                }
            }

            while let Some(item_value) = items_to_append_true.pop_front() {
                monkeys[monkey_target_true].item_list.push_back(item_value);
            }
            while let Some(item_value) = items_to_append_false.pop_front() {
                monkeys[monkey_target_false].item_list.push_back(item_value);
            }
        }
    }

    let mut monkey_inspection_counts: Vec<usize> =
        monkeys.iter().map(|monkey| monkey.inspect_count).collect();
    monkey_inspection_counts.sort();
    monkey_inspection_counts.reverse();

    println!(
        "Result: {}",
        monkey_inspection_counts[0] * monkey_inspection_counts[1]
    );
}

fn parse_monkey(monkey_information: Vec<String>) -> Monkey {
    let item_list: VecDeque<i64> = monkey_information[1][18..]
        .split(", ")
        .map(|item| item.parse::<i64>().expect("Issue parsing item"))
        .collect();

    let operation_data: Vec<&str> = monkey_information[2][19..].split(" ").collect();
    let operation = Operation {
        left_member: parse_member(operation_data[0]),
        right_member: parse_member(operation_data[2]),
        operator: parse_operator(operation_data[1]),
    };

    let divisible_value_test = monkey_information[3][21..]
        .parse::<i64>()
        .expect("Issue parsing divisible value test");

    let target_true = monkey_information[4][29..]
        .parse::<usize>()
        .expect("Issue parsing target if True");

    let target_false = monkey_information[5][30..]
        .parse::<usize>()
        .expect("Issue parsing target if False");

    Monkey {
        item_list,
        operation,
        divisible_value_test,
        target_true,
        target_false,
        inspect_count: 0,
    }
}

fn parse_member(member: &str) -> OperationMember {
    match member {
        "old" => OperationMember::Old,
        _ => OperationMember::Constant(member.parse::<i64>().expect("Issue parsing member to int")),
    }
}

fn parse_operator(operator: &str) -> Operator {
    match operator {
        "+" => Some(Operator::Add),
        "*" => Some(Operator::Multiply),
        _ => None,
    }
    .expect("Issue parsing operator")
}
