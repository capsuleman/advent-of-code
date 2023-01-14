use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
struct Droplet {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Side {
    X,
    Y,
    Z,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Facet {
    x: u64,
    y: u64,
    z: u64,
    side: Side,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("No input file given.");
    let droplets = parse_droplets(file_path);

    let facets = get_facets(&droplets);
    println!("{}", facets.len());
}

fn parse_droplets(file_path: &String) -> Vec<Droplet> {
    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let mut droplets: Vec<Droplet> = Vec::new();

    while let Some(Ok(line)) = line_iterator.next() {
        let droplet_coordinates: Vec<u64> = line
            .split(',')
            .map(|coordinate| coordinate.parse::<u64>().unwrap())
            .collect();

        droplets.push(Droplet {
            x: droplet_coordinates[0],
            y: droplet_coordinates[1],
            z: droplet_coordinates[2],
        })
    }

    droplets
}

fn get_facets_from_droplet(droplet: &Droplet) -> Vec<Facet> {
    let x = droplet.x;
    let y = droplet.y;
    let z = droplet.z;

    Vec::from([
        Facet {
            x,
            y,
            z,
            side: Side::X,
        },
        Facet {
            x,
            y,
            z,
            side: Side::Y,
        },
        Facet {
            x,
            y,
            z,
            side: Side::Z,
        },
        Facet {
            x: x + 1,
            y,
            z,
            side: Side::X,
        },
        Facet {
            x,
            y: y + 1,
            z,
            side: Side::Y,
        },
        Facet {
            x,
            y,
            z: z + 1,
            side: Side::Z,
        },
    ])
}

fn get_facets(droplets: &Vec<Droplet>) -> HashSet<Facet> {
    let mut facets: HashSet<Facet> = HashSet::new();

    for droplet in droplets.into_iter() {
        for facet in get_facets_from_droplet(&droplet).into_iter() {
            if facets.contains(&facet) {
                facets.remove(&facet);
            } else {
                facets.insert(facet);
            }
        }
    }

    facets
}
