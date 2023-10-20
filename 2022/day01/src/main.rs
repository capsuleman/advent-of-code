use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file_path = "example.txt";

    let file = File::open(file_path).expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut maximums_sorted_calories: [u32; 3] = [0; 3];
    let mut current_calories = 0;

    for line in buf_reader.lines() {
        let line_value = line?;
        if line_value != "" {
            current_calories += line_value.parse::<u32>().unwrap();
        } else {
            maximums_sorted_calories =
                update_maximum_calories(maximums_sorted_calories, current_calories);
            current_calories = 0;
        }
    }
    println!(
        "{} + {} + {} = {}",
        maximums_sorted_calories[0],
        maximums_sorted_calories[1],
        maximums_sorted_calories[2],
        maximums_sorted_calories[0] + maximums_sorted_calories[1] + maximums_sorted_calories[2]
    );

    Ok(())
}

fn update_maximum_calories(maximums_sorted_calories: [u32; 3], new_calories: u32) -> [u32; 3] {
    let mut calories_to_insert = new_calories;
    let mut new_maximums_sorted_calories = [0; 3];

    for item in maximums_sorted_calories.into_iter().enumerate() {
        let (index, calories) = item;
        if calories_to_insert > calories {
            new_maximums_sorted_calories[index] = calories_to_insert;
            calories_to_insert = maximums_sorted_calories[index];
        } else {
            new_maximums_sorted_calories[index] = maximums_sorted_calories[index];
        }
    }

    new_maximums_sorted_calories
}
