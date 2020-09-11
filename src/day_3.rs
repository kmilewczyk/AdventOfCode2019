use anyhow;

use std::io;
use std::io::prelude::*;
use std::fs;

pub fn find_closest_crossing (filepath: String) -> anyhow::Result<()> {
    let mut wires: Vec<Vec<(isize, isize)>> = Vec::new();

    let file = io::BufReader::new(fs::File::open(filepath)?);
    for line in file.lines() {
        wires.push(read_wire_pos(line?)?);
    }

    // Consider only 1st and 2nd wires (if others exist)
    let wire_1 = wires.get(0).ok_or(anyhow!("File was empty"))?;
    let wire_2 = wires.get(1).ok_or(anyhow!("File had only one line"))?;

    let mut answer: Option<(isize, isize)> = None;
    // Compare each segment to every segment of the second wire
    for i in 1..wire_1.len() {
        let a1 = wire_1.get(i-1).unwrap();
        let a2 = wire_1.get(i).unwrap();

        for j in 1..wire_2.len() {
            let b1 = wire_2.get(j-1).unwrap();
            let b2 = wire_2.get(j).unwrap();

            if let Some(p) = find_crossing(a1, a2, b1, b2) {
                if p == (0, 0) {
                    continue;
                }

                answer = match answer {
                    None => Some(p),
                    Some(prv) => {
                        if manhattan_distance(&prv, &(0,0)) > manhattan_distance(&p, &(0,0)) {
                            Some(p)
                        } else { Some(prv) }
                    }
                };
            }
        }
    }

    match answer {
        Some(value) => { println!("Answer is: {}", manhattan_distance(&(0,0), &value)) },
        None => { println!("Wires do not cross") },
    }

    Ok(())
}

// Find crossing between two segments if it exists
fn find_crossing(a1: &(isize, isize), a2: &(isize, isize), b1: &(isize, isize), b2: &(isize, isize)) -> Option<(isize, isize)> {
    let (x1, y1, x2, y2) = (a1.0, a1.1, a2.0, a2.1);
    let (x3, y3, x4, y4) = (b1.0, b1.1, b2.0, b2.1);

    // Overkill since lines are either horizontal or vertical
    let denominator = (x1-x2)*(y3-y4) - (y1-y2)*(x3-x4);
    if denominator != 0 {
        let t =   (x1-x3)*(y3-y4) - (y1-y3)*(x3-x4);
        let u = -((x1-x2)*(y1-y3) - (y1-y2)*(x1-x3));
        // Check if t / denominator is in (0, 1) range
        // If yes then segments intersect
        if t.signum() * denominator.signum() >= 0 && t.abs() <= denominator.abs()
        && u.signum() * denominator.signum() >= 0 && u.abs() <= denominator.abs() {
            // Check if intersecting point is closer than a previous one
            let f = t as f32 / denominator as f32;

            Some((x1 + (f * (x2-x1) as f32) as isize, y1 + (f * (y2-y1) as f32) as isize))
        } else {
            None
        }
    } else {
        None
    }
}


pub fn find_lowest_latency(filepath: String) -> anyhow::Result<()> {
    let mut wires: Vec<Vec<(isize, isize)>> = Vec::new();

    let file = io::BufReader::new(fs::File::open(filepath)?);
    for line in file.lines() {
        wires.push(read_wire_pos(line?)?);
    }

    // Consider only 1st and 2nd wires (if others exist)
    let wire_1 = wires.get(0).ok_or(anyhow!("File was empty"))?;
    let wire_2 = wires.get(1).ok_or(anyhow!("File had only one line"))?;

    let sums1 = calculate_accumulated_sum(&wire_1);
    let sums2 = calculate_accumulated_sum(&wire_2);

    let mut answer: Option<isize> = None;
    // Compare each segment to every segment of the second wire
    for i in 1..wire_1.len() {
        let a1 = wire_1.get(i-1).unwrap();
        let a2 = wire_1.get(i).unwrap();

        for j in 1..wire_2.len() {
            let b1 = wire_2.get(j-1).unwrap();
            let b2 = wire_2.get(j).unwrap();

            if let Some(p) = find_crossing(a1, a2, b1, b2) {
                if p == (0, 0) {
                    continue;
                }

                let latency = sums1.get(i-1).unwrap() + manhattan_distance(a1, &p) as isize
                    + sums2.get(j-1).unwrap() + manhattan_distance(b1, &p) as isize;

                answer = match answer {
                    None => Some(latency),
                    Some(prv) => if latency < prv { Some(latency) } else { Some(prv) },
                };
            }
        }
    }

    match answer {
        Some(value) => println!("Minimal latency is: {}", value),
        None => println!("Wires do not cross"),
    }

    Ok(())
}

fn calculate_accumulated_sum(v: &Vec<(isize, isize)>) -> Vec<isize> {
    let mut sums = Vec::with_capacity(v.len());

    sums.push(0);

    for i in 1..v.len() {
        let p1 = v.get(i-1).unwrap();
        let p2 = v.get(i).unwrap();

        sums.push(manhattan_distance(p1, p2) as isize + sums.last().unwrap());
    }

    sums
}


fn manhattan_distance(a: &(isize, isize), b: &(isize, isize)) -> usize {
    ( (a.1-b.1).abs() + (a.0-b.0).abs() ) as usize
}


fn read_wire_pos (line: String) -> anyhow::Result<Vec<(isize,isize)>> {
    let mut wire_pos: Vec<(isize, isize)> = Vec::new();

    wire_pos.push((0,0));

    for split in line.split(',') {
        let dir = split.as_bytes()[0] as char;
        let value = &split[1..].trim().parse::<isize>()?;
        // Next posistion initialized as previous one
        let mut pos = *wire_pos.last().unwrap_or(&(0,0));

        match dir {
            'R' => { pos.0 += value; },
            'L' => { pos.0 -= value; },
            'U' => { pos.1 += value; },
            'D' => { pos.1 -= value; },
            _ => { return Err(anyhow!("Unknown direction prefix: {}", dir)) },
        }

        wire_pos.push(pos);
    }

    Ok(wire_pos)
}
