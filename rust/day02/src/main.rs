use std::fs;

type Program = Vec<usize>;

fn load() -> Result<Program, std::io::Error> {
    Ok(fs::read_to_string("./input.txt")?
        .split(',')
        .map(|s| s.trim())
        .filter(|s| *s != "")
        .map(|s| s.parse::<usize>().unwrap()).collect())
}

fn run(mut program: Program) -> Program{
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
            99 => return program,
            _ => panic!("Invalid value {} at position {}", value, position),
        }
        position += 4;
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut program = load()?;
    program[1] = 12;
    program[2] = 2;
    let program = run(program);
    println!("Position 0: {}", program[0]);
    Ok(())
}

#[test]
fn tests() {
    assert_eq!(run(vec![1,0,0,0,99]), vec![2,0,0,0,99]);
    assert_eq!(run(vec![2,3,0,3,99]), vec![2,3,0,6,99]);
    assert_eq!(run(vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
    assert_eq!(run(vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
}
