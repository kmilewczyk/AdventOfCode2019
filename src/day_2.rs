use anyhow;

use std::fs;

const VALUE_SEARCHED: isize = 19690720;

pub fn program_alarm (filepath: String) -> anyhow::Result<()> {
    let instructions: Vec<isize> = get_instructions(filepath)?;

    let val = run_program(instructions.to_vec(), 12, 2)?;

    println!("Value at the position 0 is {}", val);

    Ok(())
}

pub fn search_answer(filepath: String) -> anyhow::Result<()> {
    let instructions: Vec<isize> = get_instructions(filepath)?;

    let mut answer: Option<isize> = None;

    for (noun, verb) in iproduct!(1..100, 1..100) {
        let value = run_program(instructions.to_vec(), noun, verb)?;

        if value == VALUE_SEARCHED {
            answer = Some(100*noun+verb);
            break;
        }
    }

    match answer {
        Some(val) => { println!("Answer is {}", val); },
        None => { println!("Answer was not found"); },
    }

    Ok(())
}

fn get_instructions(filepath: String) -> anyhow::Result<Vec<isize>> {
    let mut instructions: Vec<isize> = Vec::new();

    for substring in fs::read_to_string(filepath)?.split(',') {
        let code = substring.trim().parse::<isize>()?;
        instructions.push(code);
    }

    Ok(instructions)
}

fn run_program(mut instructions: Vec<isize>, noun: isize, verb: isize) -> anyhow::Result<isize> {
    instructions[1] = noun;
    instructions[2] = verb;

    let mut pos = 0;

    loop {
        trace!("{:?}", instructions);

        match *instructions.get(pos).ok_or(anyhow!("No element at the position {}", pos))? {
            1 => {
                let (a, b, dst) = get_values(&instructions, pos+1, pos+2, pos+3)?;

                instructions[dst as usize] = a+b;
            },
            2 => {
                let (a, b, dst) = get_values(&instructions, pos+1, pos+2, pos+3)?;

                if dst < 0 { return Err(anyhow!("Position {} is negative", dst)) };

                instructions[dst as usize] = a*b;
            },
            99 => { break; },
            op => { return Err(anyhow!("Operand {} is unspecified or unknown", op)); }
        };

        pos += 4;
    }

    Ok(*instructions.get(0).ok_or(anyhow!("Instruction set is empty"))?)
}

fn get_values(vec: &Vec<isize>, pos1: usize, pos2: usize, pos3: usize) -> anyhow::Result<(isize, isize, isize)> {
    let a = *vec.get(pos1).ok_or(anyhow!("No element at the position {}", pos1))?;
    let b = *vec.get(pos2).ok_or(anyhow!("No element at the position {}", pos2))?;
    let dst = *vec.get(pos3).ok_or(anyhow!("No element at the position {}", pos3))?;

    let v1 = *vec.get(a as usize).ok_or(anyhow!("No element at the position {}", a))?;
    let v2 = *vec.get(b as usize).ok_or(anyhow!("No element at the position {}", b))?;

    Ok((v1, v2, dst))
}
