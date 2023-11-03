use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
enum Instruction {
    Half(char),
    Triple(char),
    Increment(char),
    Jump(i32),
    JumpEven(char, i32),
    JumpOne(char, i32),
}

#[derive(Debug)]
struct InnerState {
    instruction_index: i32,
    a: u32,
    b: u32,
}

impl InnerState {
    fn get(&self, register: char) -> u32 {
        match register {
            'a' => self.a,
            'b' => self.b,
            _ => panic!("Unknown register: {}", register),
        }
    }

    fn set(&mut self, register: char, value: u32) {
        match register {
            'a' => self.a = value,
            'b' => self.b = value,
            _ => panic!("Unknown register: {}", register),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");

    let instructions = parse_instructions(&file_path);
    println!("{:?}", instructions);

    let mut inner_state = InnerState {
        instruction_index: 0,
        a: 0,
        b: 0,
    };

    while inner_state.instruction_index < instructions.len() as i32 {
        execute_one(&mut inner_state, &instructions);
    }

    println!("{:?}", inner_state);
}

fn parse_instructions(file_path: &String) -> Vec<Instruction> {
    BufReader::new(File::open(file_path).unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts = line.split(" ").collect::<Vec<_>>();
            match parts[0] {
                "hlf" => Instruction::Half(parts[1].chars().nth(0).unwrap()),
                "tpl" => Instruction::Triple(parts[1].chars().nth(0).unwrap()),
                "inc" => Instruction::Increment(parts[1].chars().nth(0).unwrap()),
                "jmp" => Instruction::Jump(parts[1].parse::<i32>().unwrap()),
                "jie" => Instruction::JumpEven(
                    parts[1].chars().nth(0).unwrap(),
                    parts[2].parse::<i32>().unwrap(),
                ),
                "jio" => Instruction::JumpOne(
                    parts[1].chars().nth(0).unwrap(),
                    parts[2].parse::<i32>().unwrap(),
                ),
                _ => panic!("Unknown instruction: {}", line),
            }
        })
        .collect()
}

fn execute_one(inner_state: &mut InnerState, instructions: &Vec<Instruction>) {
    let instruction = &instructions[inner_state.instruction_index as usize];

    match instruction {
        Instruction::Half(register) => {
            inner_state.set(*register, inner_state.get(*register) / 2);
            inner_state.instruction_index += 1;
        }
        Instruction::Triple(register) => {
            inner_state.set(*register, inner_state.get(*register) * 3);
            inner_state.instruction_index += 1;
        }
        Instruction::Increment(register) => {
            inner_state.set(*register, inner_state.get(*register) + 1);
            inner_state.instruction_index += 1;
        }
        Instruction::Jump(offset) => {
            inner_state.instruction_index += offset;
        }
        Instruction::JumpEven(register, offset) => {
            if inner_state.get(*register) % 2 == 0 {
                inner_state.instruction_index += offset;
            } else {
                inner_state.instruction_index += 1;
            }
        }
        Instruction::JumpOne(register, offset) => {
            if inner_state.get(*register) == 1 {
                inner_state.instruction_index += offset;
            } else {
                inner_state.instruction_index += 1;
            }
        }
    }
}
