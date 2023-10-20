use crate::packet::Packet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub mod packet;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let signal_2 = Packet::List(Vec::from([Packet::List(Vec::from([Packet::Integer(2)]))]));
    let signal_6 = Packet::List(Vec::from([Packet::List(Vec::from([Packet::Integer(6)]))]));

    let mut packets = Vec::from([signal_2.clone(), signal_6.clone()]);

    while let Some(Ok(line)) = line_iterator.next() {
        if line.is_empty() {
            continue;
        }
        packets.push(Packet::from(line));
    }

    packets.sort();

    let index_2 = packets
        .iter()
        .position(|packet| packet == &signal_2)
        .expect("Signal 2 not found.")
        + 1;

    let index_6 = packets
        .iter()
        .position(|packet| packet == &signal_6)
        .expect("Signal 6 not found.")
        + 1;

    println!("Result: {} * {} = {}", index_2, index_6, index_2 * index_6);
}
