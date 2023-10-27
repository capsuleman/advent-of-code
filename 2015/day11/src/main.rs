use std::{
    collections::{HashMap, HashSet},
    env,
    fs::File,
    io::{BufReader, Read},
};

use lazy_static::lazy_static;

lazy_static! {
    #[derive(Debug)]
    static ref NEXT_LETTER: HashMap<char, char> = HashMap::from([
        ('a', 'b'),
        ('b', 'c'),
        ('c', 'd'),
        ('d', 'e'),
        ('e', 'f'),
        ('f', 'g'),
        ('g', 'h'),
        ('h', 'j'),
        ('j', 'k'),
        ('k', 'm'),
        ('m', 'n'),
        ('n', 'p'),
        ('p', 'q'),
        ('q', 'r'),
        ('r', 's'),
        ('s', 't'),
        ('t', 'u'),
        ('u', 'v'),
        ('v', 'w'),
        ('w', 'x'),
        ('x', 'y'),
        ('y', 'z'),
        ('z', 'a'),
    ]);

    static ref ALL_INCREASING: HashSet<String> = HashSet::from([
        String::from("abc"),
        String::from("bcd"),
        String::from("cde"),
        String::from("def"),
        String::from("efg"),
        String::from("fgh"),
        String::from("pqr"),
        String::from("qrs"),
        String::from("rst"),
        String::from("stu"),
        String::from("tuv"),
        String::from("uvw"),
        String::from("vwx"),
        String::from("wxy"),
        String::from("xyz"),
    ]);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("an input path given.");

    let file = File::open(file_path).expect("to find a file");
    let mut password = String::new();
    BufReader::new(file)
        .read_to_string(&mut password)
        .expect("to read file");

    while !is_password_valid(&password) {
        password = get_next_password(password);
    }

    println!("{}", password);
    password = get_next_password(password);

    while !is_password_valid(&password) {
        password = get_next_password(password);
    }
    println!("{}", password);
}

fn is_password_valid(password: &String) -> bool {
    has_increasing(password) && has_multiple_pairs(password)
}

fn has_increasing(password: &String) -> bool {
    for increasing in ALL_INCREASING.iter() {
        if password.contains(increasing) {
            return true;
        }
    }
    false
}

fn has_multiple_pairs(password: &String) -> bool {
    let mut chars = password.chars();
    let mut already_one_pair = false;
    let mut previous_char = chars.next().expect("at least one char.");
    while let Some(char) = chars.next() {
        if char == previous_char {
            if already_one_pair {
                return true;
            }
            already_one_pair = true;
            previous_char = chars.next().unwrap_or_default();
        } else {
            previous_char = char;
        }
    }

    false
}

fn get_next_password(password: String) -> String {
    let mut has_carry = true;
    let reversed_password_chars = password.chars().rev();
    let mut reversed_new_password_chars: Vec<char> = Vec::new();
    for char in reversed_password_chars {
        let new_char = match has_carry {
            true => *NEXT_LETTER.get(&char).expect("unknown letter"),
            false => char,
        };
        reversed_new_password_chars.push(new_char);

        has_carry = char == 'z' && has_carry;
    }

    reversed_new_password_chars.reverse();
    reversed_new_password_chars.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_password_simple() {
        assert_eq!(
            get_next_password(String::from("aaabcddd")),
            String::from("aaabcdde")
        );
    }

    #[test]
    fn test_next_password_carry() {
        assert_eq!(
            get_next_password(String::from("hzzzzzzz")),
            String::from("jaaaaaaa")
        );
    }
}
