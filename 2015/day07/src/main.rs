use core::panic;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq)]
enum OneOperandOperation {
    Constant,
    Not,
}

#[derive(Debug, PartialEq)]
enum TwoOperandOperation {
    And,
    Or,
    RShift,
    LShift,
}

#[derive(Debug, PartialEq)]
enum Operand {
    Constant(u16),
    GateId(String),
}

#[derive(Debug, PartialEq)]
enum Gate {
    OneOperandGate(OneOperandOperation, Operand),
    TwoOperandGate(TwoOperandOperation, Operand, Operand),
}

lazy_static! {
    static ref CONSTANT_OPERATOR_RE: Regex = Regex::new(r"^(\d+) -> ([a-z]+)$").unwrap();
    static ref NOT_OPERATOR_RE: Regex = Regex::new(r"^NOT ([a-z]+) -> ([a-z]+)$").unwrap();
    static ref TWO_OPERATORS_RE: Regex =
        Regex::new(r"^([a-z]+|\d+) (AND|OR|LSHIFT|RSHIFT) ([a-z]+|\d+) -> ([a-z]+)$").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");
    let gates = parse_gates(file_path);
    let mut gate_values: HashMap<String, u16> = HashMap::new();
    println!(
        "{:#?}",
        compute_gate_id(&String::from("a"), &gates, &mut gate_values)
    );
}

fn parse_operand(operand: &str) -> Operand {
    if let Ok(constant) = operand.parse::<u16>() {
        Operand::Constant(constant)
    } else {
        Operand::GateId(String::from(operand))
    }
}

fn parse_gate(line: String) -> (String, Gate) {
    let mut line_parts = line.split(" -> ");
    let operation_formula = line_parts.next().expect("to get the operation");
    let gate_id = String::from(line_parts.next().expect("to get the gate ID"));

    let operation_parts = operation_formula.split(" ").collect::<Vec<&str>>();

    if operation_parts.len() == 1 {
        let gate = Gate::OneOperandGate(
            OneOperandOperation::Constant,
            parse_operand(operation_parts[0]),
        );
        return (gate_id, gate);
    }

    if operation_parts.len() == 2 {
        let gate =
            Gate::OneOperandGate(OneOperandOperation::Not, parse_operand(operation_parts[1]));
        return (gate_id, gate);
    }

    if operation_parts.len() == 3 {
        let operation = match operation_parts[1] {
            "AND" => TwoOperandOperation::And,
            "OR" => TwoOperandOperation::Or,
            "LSHIFT" => TwoOperandOperation::LShift,
            "RSHIFT" => TwoOperandOperation::RShift,
            _ => panic!("Unknown operator"),
        };
        let gate = Gate::TwoOperandGate(
            operation,
            parse_operand(operation_parts[0]),
            parse_operand(operation_parts[2]),
        );
        return (gate_id, gate);
    }

    panic!("Unknown gate expression");
}

fn parse_gates(file_path: &String) -> HashMap<String, Gate> {
    let file = File::open(file_path).expect("File not found!");
    let mut line_iter = BufReader::new(file).lines();
    let mut gates = HashMap::new();

    while let Some(Ok(line)) = line_iter.next() {
        let (gate_id, gate) = parse_gate(line);
        gates.insert(gate_id, gate);
    }

    gates
}

fn compute_gate_id(
    gate_id: &String,
    gates: &HashMap<String, Gate>,
    gate_values: &mut HashMap<String, u16>,
) -> u16 {
    if let Some(value) = gate_values.get(gate_id) {
        return *value;
    }

    let gate = gates.get(gate_id).expect("to get gate");

    let value = match gate {
        Gate::OneOperandGate(OneOperandOperation::Constant, operand) => {
            get_operand_value(operand, gates, gate_values)
        }
        Gate::OneOperandGate(OneOperandOperation::Not, operand) => {
            !get_operand_value(operand, gates, gate_values)
        }
        Gate::TwoOperandGate(TwoOperandOperation::And, operand_1, operand_2) => {
            get_operand_value(operand_1, gates, gate_values)
                & get_operand_value(operand_2, gates, gate_values)
        }
        Gate::TwoOperandGate(TwoOperandOperation::Or, operand_1, operand_2) => {
            get_operand_value(operand_1, gates, gate_values)
                | get_operand_value(operand_2, gates, gate_values)
        }
        Gate::TwoOperandGate(TwoOperandOperation::LShift, operand_1, operand_2) => {
            get_operand_value(operand_1, gates, gate_values)
                << get_operand_value(operand_2, gates, gate_values)
        }
        Gate::TwoOperandGate(TwoOperandOperation::RShift, operand_1, operand_2) => {
            get_operand_value(operand_1, gates, gate_values)
                >> get_operand_value(operand_2, gates, gate_values)
        }
    };

    gate_values.insert(gate_id.clone(), value);
    value
}

fn get_operand_value(
    operand: &Operand,
    gates: &HashMap<String, Gate>,
    gate_values: &mut HashMap<String, u16>,
) -> u16 {
    match operand {
        Operand::Constant(constant) => *constant,
        Operand::GateId(gate_id) => compute_gate_id(gate_id, gates, gate_values),
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_constant_gate() {
        assert_eq!(
            parse_gate(String::from("14146 -> ab")),
            (
                String::from("ab"),
                Gate::OneOperandGate(OneOperandOperation::Constant, Operand::Constant(14146))
            )
        );
    }

    #[test]
    fn test_not_gate() {
        assert_eq!(
            parse_gate(String::from("NOT az -> ab")),
            (
                String::from("ab"),
                Gate::OneOperandGate(
                    OneOperandOperation::Not,
                    Operand::GateId(String::from("az"))
                )
            )
        );
    }

    #[test]
    fn test_and_gate() {
        assert_eq!(
            parse_gate(String::from("ad AND az -> ab")),
            (
                String::from("ab"),
                Gate::TwoOperandGate(
                    TwoOperandOperation::And,
                    Operand::GateId(String::from("ad")),
                    Operand::GateId(String::from("az"))
                )
            )
        );
    }

    #[test]
    fn test_and_gate_with_constant() {
        assert_eq!(
            parse_gate(String::from("14 AND 12 -> ab")),
            (
                String::from("ab"),
                Gate::TwoOperandGate(
                    TwoOperandOperation::And,
                    Operand::Constant(14),
                    Operand::Constant(12)
                )
            )
        );
    }

    #[test]
    fn test_lshift_gate() {
        assert_eq!(
            parse_gate(String::from("ad LSHIFT 3 -> ab")),
            (
                String::from("ab"),
                Gate::TwoOperandGate(
                    TwoOperandOperation::LShift,
                    Operand::GateId(String::from("ad")),
                    Operand::Constant(3)
                )
            )
        );
    }

    #[test]
    fn test_parse_operand_constant() {
        assert_eq!(parse_operand("12"), Operand::Constant(12));
    }

    #[test]
    fn test_parse_operand_gate() {
        assert_eq!(parse_operand("ab"), Operand::GateId(String::from("ab")));
    }
}
