use std::cmp::min;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum Packet {
    Integer(u32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match are_ordered(self, other) {
            None => std::cmp::Ordering::Equal,
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Packet {}

impl Packet {
    pub fn from(line: String) -> Packet {
        parse_packet(line)
    }
}

fn are_ordered(left_packet: &Packet, right_packet: &Packet) -> Option<bool> {
    match (left_packet, right_packet) {
        (Packet::Integer(left_value), Packet::Integer(right_value)) => {
            are_integers_ordered(left_value, right_value)
        }
        (Packet::List(left_list), Packet::List(right_list)) => {
            are_lists_ordered(left_list, right_list)
        }
        (Packet::Integer(left_value), Packet::List(right_list)) => {
            are_lists_ordered(&Vec::from([Packet::Integer(*left_value)]), right_list)
        }
        (Packet::List(left_list), Packet::Integer(right_value)) => {
            are_lists_ordered(left_list, &Vec::from([Packet::Integer(*right_value)]))
        }
    }
}

fn are_integers_ordered(left_value: &u32, right_value: &u32) -> Option<bool> {
    if left_value == right_value {
        None
    } else {
        Some(left_value < right_value)
    }
}

fn are_lists_ordered(left_list: &Vec<Packet>, right_list: &Vec<Packet>) -> Option<bool> {
    let min_len = min(left_list.len(), right_list.len());
    for index in 0..min_len {
        let are_element_ordered = are_ordered(&left_list[index], &right_list[index]);
        if are_element_ordered.is_some() {
            return are_element_ordered;
        }
        continue;
    }

    if left_list.len() == right_list.len() {
        return None;
    }

    return Some(left_list.len() <= right_list.len());
}

fn parse_packet(line: String) -> Packet {
    let (packet, _line_size) = parse_packet_aux(&line);
    packet
}

fn parse_packet_aux(line: &str) -> (Packet, usize) {
    if !line.starts_with('[') {
        panic!("Packet does not start with `[`");
    }

    let mut index = 1;
    let mut packet_list: Vec<Packet> = Vec::new();

    let mut current_value = String::new();

    while index < line.len() {
        let char = line.as_bytes()[index] as char;

        if char == '[' {
            let (nested_packet, str_size) = parse_packet_aux(&line[index..]);
            packet_list.push(nested_packet);
            index += str_size + 1;
            continue;
        }

        if char == ',' || char == ']' {
            if !current_value.is_empty() {
                let parsed_current_value = current_value.parse::<u32>().unwrap();
                packet_list.push(Packet::Integer(parsed_current_value));
                current_value = String::new();
            }

            if char == ']' {
                return (Packet::List(packet_list), index);
            }
            index += 1;
            continue;
        }
        current_value.push(char);
        index += 1;
    }

    panic!("Error parsing packet.");
}
