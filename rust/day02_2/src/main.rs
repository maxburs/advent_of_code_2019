use std::fs;

type Memory = Vec<usize>;

fn load() -> Result<Memory, std::io::Error> {
    Ok(fs::read_to_string("./input.txt")?
        .split(',')
        .map(|s| s.trim())
        .filter(|s| *s != "")
        .map(|s| s.parse::<usize>().unwrap()).collect())
}

fn add(memory: &mut Memory, instruction_pointer: usize) {
    let val = memory[memory[instruction_pointer + 1]] + memory[memory[instruction_pointer + 2]];
    let pos = memory[instruction_pointer + 3];
    memory[pos] = val;
}

fn multiply(memory: &mut Memory, instruction_pointer: usize) {
    let val = memory[memory[instruction_pointer + 1]] * memory[memory[instruction_pointer + 2]];
    let pos = memory[instruction_pointer + 3];
    memory[pos] = val;
}

fn run(mut memory: Memory) -> Result<Memory, String>{
    let mut instruction_pointer = 0;

    loop {
        let opcode = memory[instruction_pointer];
        match opcode {
            1 => add(&mut memory, instruction_pointer),
            2 => multiply(&mut memory, instruction_pointer),
            99 => return Ok(memory),
            _ => return Err(format!("Invalid opcode {} at instruction_pointer {}", opcode, instruction_pointer)),
        }
        instruction_pointer += 4;
    }
}

fn main() -> Result<(), std::io::Error> {
    let memory = load()?;

    for noun in 0..100 {
        for verb in 0..100 {

            let mut prospect = memory.clone();
            prospect[1] = noun;
            prospect[2] = verb;

            match run(prospect) {
                Ok(result) => {
                    if result[0] == 19690720 {
                        println!("noun: {}, verb: {}", noun, verb);
                        return Ok(());
                    }
                },
                Err(_) => { println!("hello world")}
            }
        }
    }
    
    println!("No answer found");
    Ok(())
}

#[test]
fn tests() {
    assert_eq!(run(vec![1,0,0,0,99]).unwrap(), vec![2,0,0,0,99]);
    assert_eq!(run(vec![2,3,0,3,99]).unwrap(), vec![2,3,0,6,99]);
    assert_eq!(run(vec![2,4,4,5,99,0]).unwrap(), vec![2,4,4,5,99,9801]);
    assert_eq!(run(vec![1,1,1,4,99,5,6,0,99]).unwrap(), vec![30,1,1,4,2,5,6,0,99]);
}
