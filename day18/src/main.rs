use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Hash, Eq, PartialEq)]
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

    let facets_count = get_facets_count_using_inverted_droplets(&droplets);

    println!("Result: {facets_count}");
}

fn parse_droplets(file_path: &String) -> HashSet<Droplet> {
    let file = File::open(file_path).expect("File not found!");
    let buf_reader = BufReader::new(file);

    let mut line_iterator = buf_reader.lines().into_iter();

    let mut droplets: HashSet<Droplet> = HashSet::new();

    while let Some(Ok(line)) = line_iterator.next() {
        let droplet_coordinates: Vec<u64> = line
            .split(',')
            .map(|coordinate| coordinate.parse::<u64>().unwrap())
            .collect();

        droplets.insert(Droplet {
            x: droplet_coordinates[0],
            y: droplet_coordinates[1],
            z: droplet_coordinates[2],
        });
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

fn get_facets(droplets: &HashSet<Droplet>) -> HashSet<Facet> {
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

fn get_facets_count_using_inverted_droplets(droplets: &HashSet<Droplet>) -> usize {
    let max_x = droplets.iter().map(|droplet| droplet.x).max().unwrap() + 1;
    let max_y = droplets.iter().map(|droplet| droplet.y).max().unwrap() + 1;
    let max_z = droplets.iter().map(|droplet| droplet.z).max().unwrap() + 1;

    let mut inverted_droplets: HashSet<Droplet> = HashSet::new();
    let mut droplets_to_explore = HashSet::from([Droplet { x: 0, y: 0, z: 0 }]);

    while droplets_to_explore.len() > 0 {
        let neighbors: HashSet<Droplet> = droplets_to_explore
            .iter()
            .flat_map(|droplet_to_explore| get_neighbors(&droplet_to_explore, max_x, max_y, max_z))
            .collect();

        let next_droplets_to_explore: HashSet<Droplet> = neighbors
            .into_iter()
            .filter(|neighbor| {
                !inverted_droplets.contains(&neighbor) && !droplets.contains(&neighbor)
            })
            .collect();

        inverted_droplets.extend(droplets_to_explore);

        droplets_to_explore = next_droplets_to_explore;
    }

    let droplet_without_holes = get_inverted_droplets(&inverted_droplets, max_x, max_y, max_z);
    let facets_without_holes = get_facets(&droplet_without_holes);

    facets_without_holes.len()
}

fn get_neighbors(droplet: &Droplet, max_x: u64, max_y: u64, max_z: u64) -> Vec<Droplet> {
    let x = droplet.x;
    let y = droplet.y;
    let z = droplet.z;

    let mut neighbors: Vec<Droplet> = Vec::new();

    if droplet.x > 0 {
        neighbors.push(Droplet { x: x - 1, y, z })
    }
    if droplet.y > 0 {
        neighbors.push(Droplet { x, y: y - 1, z })
    }
    if droplet.z > 0 {
        neighbors.push(Droplet { x, y, z: z - 1 })
    }

    if droplet.x < max_x {
        neighbors.push(Droplet { x: x + 1, y, z })
    }
    if droplet.y < max_y {
        neighbors.push(Droplet { x, y: y + 1, z })
    }
    if droplet.z < max_z {
        neighbors.push(Droplet { x, y, z: z + 1 })
    }

    neighbors
}

fn get_inverted_droplets(
    droplets: &HashSet<Droplet>,
    max_x: u64,
    max_y: u64,
    max_z: u64,
) -> HashSet<Droplet> {
    let mut inverted_droplet: HashSet<Droplet> = HashSet::new();

    for x in 0..max_x + 1 {
        for y in 0..max_y + 1 {
            for z in 0..max_z + 1 {
                let droplet = Droplet { x, y, z };
                if !droplets.contains(&droplet) {
                    inverted_droplet.insert(droplet);
                }
            }
        }
    }

    inverted_droplet
}
