use std::{env, fs::File, io::BufReader};

use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");
    let file = File::open(file_path).expect("to find a file");

    let v: Value = serde_json::from_reader(BufReader::new(file)).expect("to parse JSON");

    println!("{}", count_json(v));
}

fn count_json(json: Value) -> i64 {
    match json {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(json_vec) => json_vec.into_iter().map(count_json).sum(),
        Value::Object(json_map) => json_map
            .into_iter()
            .map(|(_key, json)| count_json(json))
            .sum(),
    }
}
