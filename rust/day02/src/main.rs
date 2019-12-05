use std::fs;

type Program = Vec<usize>;

fn load() -> Result<Program, std::io::Error> {
    Ok(fs::read_to_string("./input.txt")?
        .split(',')
        .map(|s| s.trim())
        .filter(|s| *s != "")
        .map(|s| s.parse::<usize>().unwrap()).collect())
}

fn run(program: &mut Program) -> Result<(), std::io::Error> {
    let mut position = 0;

    loop {
        let value = program[position];
        match value {
            1 => {
                let val = program[program[position + 1]] + program[program[position + 2]];
                let pos = program[position + 3];
                program[pos] = val;
            },
            2 =>{
                let val = program[program[position + 1]] * program[program[position + 2]];
                let pos = program[position + 3];
                program[pos] = val;
            },
            99 => return Ok(()),
            _ => panic!("Invalid value {} at position {}", value, position),
        }
        position += 4;
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut memory = load()?;
    run(&mut memory)?;
    println!("Position 0: {}", memory[0]);
    Ok(())
}


