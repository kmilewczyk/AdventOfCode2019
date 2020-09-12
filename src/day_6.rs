use anyhow;

use std::io;
use std::io::prelude::*;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

type Orbits = HashMap<String, HashSet<String>>;

pub fn total_orbit_count(filepath: String) -> anyhow::Result<()> {
    let orbits: Orbits = read_orbits(filepath)?;

    count_orbits(orbits)?;

    Ok(())
}

fn read_orbits(filepath: String) -> anyhow::Result<Orbits> {
    let mut orbits: Orbits = HashMap::new();

    let file = io::BufReader::new(fs::File::open(filepath)?);
    for line in file.lines() {
        let relationship: Vec<String> = line?.split(")").take(2).map(|s| s.to_owned()).collect();

        let key = relationship.get(0).ok_or(anyhow!("Empty line"))?.clone();
        let value = relationship.get(1).ok_or(anyhow!("Only one item was given"))?;

        match orbits.get_mut(&key) {
            Some(set) => { set.insert(value.to_owned()); },
            None => {
                let mut set = HashSet::new();
                set.insert(value.to_owned());
                orbits.insert(key, set);
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
        if let Some(children) = orbits.get(planet.name) {
            for child in children {
                orbit_stack.push_back(PlanetNode{name: child, depth: planet.depth+1});
            }
        }
    }

    println!("Total count of the orbits: {}", orbit_count);

    Ok(())
}
