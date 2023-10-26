use lazy_static::lazy_static;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

lazy_static! {
    static ref DISTANCE_RE: Regex = Regex::new(r"^([A-Za-z]+) to ([A-Za-z]+) = (\d+)$").unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");
    let distances_graph = parse_distances(file_path);
    let maximum_distance = get_maximum_distance(&distances_graph);
    println!("{:#?}", maximum_distance)
}

fn parse_distances(file_path: &String) -> HashMap<String, HashMap<String, u32>> {
    let mut graph = HashMap::new();

    let file = File::open(file_path).expect("File not found!");
    let mut line_iter = BufReader::new(file).lines();

    while let Some(Ok(line)) = line_iter.next() {
        let captures = DISTANCE_RE.captures(&line).expect("to parse line");
        let point_1 = String::from(&captures[1]);
        let point_2 = String::from(&captures[2]);
        let distance = captures[3].parse::<u32>().expect("to parse a number");

        insert_distance_into_graph(&point_1, &point_2, distance, &mut graph);
        insert_distance_into_graph(&point_2, &point_1, distance, &mut graph);
    }

    graph
}

fn insert_distance_into_graph(
    from: &String,
    to: &String,
    distance: u32,
    graph: &mut HashMap<String, HashMap<String, u32>>,
) {
    graph
        .entry(from.clone())
        .and_modify(|sub_graph| {
            sub_graph.insert(to.clone(), distance);
        })
        .or_insert(HashMap::from([(to.clone(), distance)]));
}

fn get_maximum_distance(graph: &HashMap<String, HashMap<String, u32>>) -> u32 {
    let mut max_distance = u32::MIN;

    for (position, _sub_graph) in graph.into_iter() {
        max_distance = u32::max(
            max_distance,
            get_maximum_distance_aux(HashSet::from([position.clone()]), position, graph),
        );
    }

    max_distance
}

fn get_maximum_distance_aux(
    already_seen: HashSet<String>,
    current_position: &String,
    graph: &HashMap<String, HashMap<String, u32>>,
) -> u32 {
    if graph.len() == already_seen.len() {
        return 0;
    }

    let position_sub_graph = graph.get(current_position).expect("to get sub graph");
    let mut max_distance = u32::MIN;

    for (next_position, distance) in position_sub_graph.iter() {
        if already_seen.contains(next_position) {
            continue;
        }
        let mut new_already_seen = already_seen.clone();
        new_already_seen.insert(next_position.clone());
        let next_maximum_distance =
            get_maximum_distance_aux(new_already_seen, next_position, graph);

        max_distance = u32::max(max_distance, next_maximum_distance + *distance);
    }
    max_distance
}
