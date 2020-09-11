use anyhow;

use std::io;
use std::io::prelude::*;

use crate::day_2;

pub fn diagnostic_tests(filename: String) -> anyhow::Result<()> {
    let mut instructions = day_2::get_instructions(filename)?;

    run_program(&mut instructions)?;

    Ok(())
}

pub fn run_program (instructions: &mut Vec<isize>) -> anyhow::Result<()> {
    let mut ip = 0; // instruction pointer

    loop {
        let instruction = get_value(instructions, ip)?;
        let opcode = instruction % 100;
        let parameter_modes = instruction / 100;

        match opcode {
            1 => {
                let a = get_paramater(instructions, parameter_modes, 1, ip+1)?;
                let b = get_paramater(instructions, parameter_modes, 10, ip+2)?;
                let dst = get_value(instructions, ip+3)? as usize;

                instructions[dst] = a+b;
                ip += 4;
            },
            2 => {
                let a = get_paramater(instructions, parameter_modes, 1, ip+1)?;
                let b = get_paramater(instructions, parameter_modes, 10, ip+2)?;
                let dst = get_value(instructions, ip+3)? as usize;

                instructions[dst] = a*b;
                ip += 4;
            },
            3 => {
                let mut buf = String::new();

                print!("<- ");
                io::stdout().flush()?;
                io::stdin().read_line(&mut buf)?;

                let value = buf.trim().parse::<isize>()?;
                let dst = get_value(instructions, ip+1)? as usize;
                instructions[dst] = value;

                ip += 2;
            },
            4 => {
                let pos = get_value(instructions, ip+1)? as usize;

                println!("-> {}", instructions[pos]);

                ip += 2;
            },
            99 => { break; },
            op => { return Err(anyhow!("Operand {} is unknown", op)); }
        };
    }

    Ok(())
}

// divisor is a digit that needs to be read, hunders digit would be 100, thousands digit would be
// 1000
fn get_paramater(instructions: &Vec<isize>, parameter_modes: isize, divisor: isize, pos: usize) -> anyhow::Result<isize> {
    match (parameter_modes / divisor) % 10 {
        0 => get_value(instructions, get_value(instructions, pos)? as usize),
        1 => get_value(instructions, pos),
        parameter => return Err(anyhow!("Unknown paramter mode {}", parameter)),
    }
}

fn get_value(instructions: &Vec<isize>, pos: usize) -> anyhow::Result<isize> {
    instructions.get(pos).ok_or(anyhow!("No element at the position {}", pos)).map(|i| *i)
}
