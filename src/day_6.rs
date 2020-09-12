use anyhow;

use std::io;
use std::io::prelude::*;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;


struct Planet {
    parent: String,
    satellites: HashSet<String>,
}

type Orbits = HashMap<String, Planet>;

pub fn total_orbit_count(filepath: String) -> anyhow::Result<()> {
    let orbits: Orbits = read_orbits(filepath)?;

    count_orbits(orbits)?;

    Ok(())
}

pub fn orbital_transfer_length(filepath: String) -> anyhow::Result<()> {
    let orbits: Orbits = read_orbits(filepath)?;

    find_orbital_transfer_length(orbits)?;

    Ok(())
}

fn read_orbits(filepath: String) -> anyhow::Result<Orbits> {
    let mut orbits: Orbits = HashMap::new();

    let file = io::BufReader::new(fs::File::open(filepath)?);
    for line in file.lines() {
        let relationship: Vec<String> = line?.split(")").take(2).map(|s| s.to_owned()).collect();

        let key = relationship.get(0).ok_or(anyhow!("Empty line"))?.clone();
        let value = relationship.get(1).ok_or(anyhow!("Only one item was given"))?;

        // If planet exists then add parent info
        if let Some(planet) = orbits.get_mut(value) {
            planet.parent = key.to_owned();
        } else {
            orbits.insert(value.to_owned(), Planet { parent: key.to_owned(), satellites: HashSet::new() });
        }

        match orbits.get_mut(&key) {
            Some(planet) => { planet.satellites.insert(value.to_owned()); },
            None => {
                // If planet was not yet mention, then create instance without filling in the parent
                let mut satellites: HashSet<String> = HashSet::new();
                satellites.insert(value.to_owned());

                orbits.insert(key.to_owned(), Planet { parent: "".to_string(), satellites });
            },
        };
    }

    Ok(orbits)
}


fn count_orbits(orbits: Orbits) -> anyhow::Result<()> {
    struct PlanetNode<'a> {
        name: &'a str,
        depth: usize,
    };

    let mut orbit_count: usize = 0;
    let mut orbit_stack: VecDeque<PlanetNode> = VecDeque::new();

    orbit_stack.push_back(PlanetNode{name: "COM", depth: 0});

    while let Some(planet) = orbit_stack.pop_back() {
        // Add current path length to the sum. It represents all indirect orbits from "planet" to
        // the parents in the tree graph
        orbit_count += planet.depth;

        // DFS children via stack
        if let Some(planet_info) = orbits.get(planet.name) {
            for satellite in planet_info.satellites.iter() {
                orbit_stack.push_back(PlanetNode{name: satellite, depth: planet.depth+1});
            }
        }
    }

    println!("Total count of the orbits: {}", orbit_count);

    Ok(())
}

fn find_orbital_transfer_length(orbits: Orbits) -> anyhow::Result<()> {
    let you_chain = construct_orbital_chain(&orbits, "YOU")?;
    let san_chain = construct_orbital_chain(&orbits, "SAN")?;

    // length from COM to last common ancestor
    let common_length: usize = {
        let mut common_length = you_chain.len().min(san_chain.len());

        for i in 0..you_chain.len().min(san_chain.len()) {
            if you_chain.get(i).unwrap() != san_chain.get(i).unwrap() {
                common_length = i;
                break;
            }
        }

       common_length
    };

    let orbital_transfer_length = (you_chain.len() - common_length) + (san_chain.len() - common_length);

    println!("Length of orbital transfer is equal to {}", orbital_transfer_length);

    Ok(())
}

fn construct_orbital_chain<'a, 'b>(orbits: &'a Orbits, planet: &'b str) -> anyhow::Result<VecDeque<&'a str>> {
    let mut chain: VecDeque<&str> = VecDeque::new();
    let mut node = planet;


    while node != "" { // while parent of a planet exists
        let parent = orbits.get(node).ok_or(anyhow!("Node {} was not found", node))?.parent.as_str();
        chain.push_front(parent);
        node = parent;
    }

    Ok(chain)
}
