use combinations::CombinationsRestrictedBySum;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::{AddAssign, Mul},
};

#[derive(Debug, Default, Clone, Copy)]
struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
}

impl AddAssign for Ingredient {
    fn add_assign(&mut self, rhs: Self) {
        self.capacity += rhs.capacity;
        self.durability += rhs.durability;
        self.flavor += rhs.flavor;
        self.texture += rhs.texture;
    }
}

impl Mul<u32> for Ingredient {
    type Output = Self;

    fn mul(self, rhs: u32) -> Ingredient {
        Ingredient {
            capacity: self.capacity * rhs as i32,
            durability: self.durability * rhs as i32,
            flavor: self.flavor * rhs as i32,
            texture: self.texture * rhs as i32,
        }
    }
}

impl Ingredient {
    fn get_score(&self) -> u32 {
        (i32::max(0, self.capacity)
            * i32::max(0, self.durability)
            * i32::max(0, self.flavor)
            * i32::max(0, self.texture)) as u32
    }
}

lazy_static! {
    static ref INPUT_RE: Regex = Regex::new(
        r"^[A-Za-z]+: capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories -?\d+$"
    )
    .unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("to be given an input file.");
    let ingredients = parse_ingredients(file_path);

    let mut max_combination_score = 0;
    for combination in CombinationsRestrictedBySum::new(ingredients.len(), 100) {
        max_combination_score = u32::max(
            max_combination_score,
            get_combination_score(combination, &ingredients),
        );
    }
    println!("{max_combination_score}");
}

fn parse_ingredients(file_path: &String) -> Vec<Ingredient> {
    let mut ingredients = vec![];

    let file = File::open(file_path).expect("File not found!");
    let mut line_iter = BufReader::new(file).lines();

    while let Some(Ok(line)) = line_iter.next() {
        let captures = INPUT_RE.captures(&line).expect("to parse line");

        let capacity = captures[1].parse::<i32>().expect("a number");
        let durability = captures[2].parse::<i32>().expect("a number");
        let flavor = captures[3].parse::<i32>().expect("a number");
        let texture = captures[4].parse::<i32>().expect("a number");

        ingredients.push(Ingredient {
            capacity,
            durability,
            flavor,
            texture,
        })
    }

    ingredients
}

fn get_combination_score(combination: Vec<u32>, ingredients: &Vec<Ingredient>) -> u32 {
    let mut total_ingredients = Ingredient::default();
    for (&spoon_count, ingredient) in combination.iter().zip(ingredients) {
        total_ingredients += ingredient.clone() * spoon_count;
    }

    total_ingredients.get_score()
}
