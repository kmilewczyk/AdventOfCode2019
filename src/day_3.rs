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
    // Compare each segment to every segment of the wire
    for i in 2..wire_1.len() {
        let (x1, y1, x2, y2) = {
            let p1 = wire_1.get(i-1).unwrap();
            let p2 = wire_1.get(i).unwrap();
            (p1.0, p1.1, p2.0, p2.1)
        };

        for j in 1..wire_2.len() {
            let (x3, y3, x4, y4) = {
                let p1 = wire_2.get(j-1).unwrap();
                let p2 = wire_2.get(j).unwrap();
                (p1.0, p1.1, p2.0, p2.1)
            };

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
                    let p = (x1 + (f * (x2-x1) as f32) as isize, y1 + (f * (y2-y1) as f32) as isize);

                    answer = match answer {
                        None => Some(p),
                        Some(prv) => {
                            if manhattan_distance(prv, (0,0)) > manhattan_distance(p, (0,0)) {
                                Some(p)
                            } else { Some(prv) }
                        }
                    };
                }
            }
        }
    }

    match answer {
        Some(value) => { println!("Answer is: {}", manhattan_distance((0,0), value)) },
        None => { println!("Wires do not cross") },
    }

    Ok(())
}


fn manhattan_distance(a: (isize, isize), b: (isize, isize)) -> usize {
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
