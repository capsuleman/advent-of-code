use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_path = "real.txt";

    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut x_history = vec![1];

    for line in buf_reader.lines() {
        let line_value = line?;

        if line_value == "noop" {
            x_history.push(*x_history.last().unwrap());
        } else {
            let added_value = &line_value[5..].parse::<i32>().unwrap();
            x_history.push(*x_history.last().unwrap());
            x_history.push(*x_history.last().unwrap() + added_value);
        }
    }

    let screen = x_history
        .into_iter()
        .enumerate()
        .map(|(index, x)| {
            let singed_index = i32::try_from(index % 40).unwrap();
            if singed_index - 1 <= x && x <= singed_index + 1 {
                '#'
            } else {
                '.'
            }
        })
        .collect::<String>();
    println!("{}", &screen[0..40]);
    println!("{}", &screen[40..80]);
    println!("{}", &screen[80..120]);
    println!("{}", &screen[120..160]);
    println!("{}", &screen[160..200]);
    println!("{}", &screen[200..240]);

    Ok(())
}
