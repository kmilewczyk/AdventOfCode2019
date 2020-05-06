use anyhow;

use std::io;
use std::io::prelude::*;
use std::fs;

pub fn calculate_fuel (filepath: String) -> anyhow::Result<()> {
    let file = io::BufReader::new(fs::File::open(filepath)?);

    let mut total_fuel: usize = 0;

    for line in file.lines() {
        let mass = line?.parse::<usize>()?;
        total_fuel += mass/3-2;
    }

    println!("Total fuel required: {}", total_fuel);

    Ok(())
}


pub fn calculate_fuel_extended (filepath: String) -> anyhow::Result<()> {
    let file = io::BufReader::new(fs::File::open(filepath)?);

    let mut total_fuel: usize = 0;

    for line in file.lines() {
        let mut mass = line?.parse::<usize>()? as isize;

        loop {
            mass = mass/3-2;
            if mass <= 0 { break; }

            total_fuel += mass as usize;
        }
    }

    println!("Total fuel required: {}", total_fuel);

    Ok(())
}
