use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");

    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);
    let mut line_iter = buf_reader.lines();

    let mut good_word_count: usize = 0;

    while let Some(Ok(line)) = line_iter.next() {
        if check_word(&line) {
            println!("{}", line);
            good_word_count += 1;
        };
    }
    println!("{good_word_count}")
}

fn check_word(word: &str) -> bool {
    check_double_pair(word) && check_sandwich(word)
}

fn check_double_pair(word: &str) -> bool {
    let mut word_iter = word.chars().into_iter().enumerate();
    let mut already_seen: HashMap<[char; 2], usize> = HashMap::new();
    let (_index, mut previous_letter) = word_iter.next().expect("the first letter");
    while let Some((index, letter)) = word_iter.next() {
        let letter_pair = [previous_letter, letter];
        if let Some(&last_index) = already_seen.get(&letter_pair) {
            if last_index + 1 < index {
                return true;
            }
            continue;
        }
        already_seen.insert(letter_pair, index);
        previous_letter = letter;
    }
    false
}

fn check_sandwich(word: &str) -> bool {
    let mut word_iter = word.chars().into_iter();
    let mut previous_previous_letter = word_iter.next().expect("the first letter");
    let mut previous_letter = word_iter.next().expect("the second letter");

    for letter in word_iter {
        if letter == previous_previous_letter {
            return true;
        }
        previous_previous_letter = previous_letter;
        previous_letter = letter;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_double_overlap() {
        assert_eq!(check_double_pair("aaabcddd"), false);
    }

    #[test]
    fn test_double_double() {
        assert_eq!(check_double_pair("abbcddaab"), true);
    }

    #[test]
    fn test_double_double_row() {
        assert_eq!(check_double_pair("aaaa"), true);
    }

    #[test]
    fn test_sandwich_1() {
        assert_eq!(check_sandwich("xyx"), true);
    }

    #[test]
    fn test_sandwich_2() {
        assert_eq!(check_sandwich("xyz"), false);
    }

    #[test]
    fn test_sandwich_3() {
        assert_eq!(check_sandwich("aaa"), true);
    }

    #[test]
    fn test_all_1() {
        assert_eq!(check_double_pair("qjhvhtzxzqqjkmpb"), true);
    }

    #[test]
    fn test_all_2() {
        assert_eq!(check_word("xxyxx"), true);
    }

    #[test]
    fn test_all_3() {
        assert_eq!(check_word("uurcxstgmygtbstg"), false);
    }

    #[test]
    fn test_all_4() {
        assert_eq!(check_word("ieodomkazucvgmuy"), false);
    }
}
