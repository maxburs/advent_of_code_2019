use std::fs;

type Memory = Vec<isize>;

#[derive(Debug, Eq, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

struct InstructionArguments<'a> {
    memory: &'a mut Memory,
    instruction_pointer: usize,
}

impl InstructionArguments<'_> {
    fn param(&self, position: usize) -> isize {
        let mode = find_mode(self.memory[self.instruction_pointer], position);
        match mode {
            ParameterMode::Position => {
                self.memory[self.memory[self.instruction_pointer + position] as usize]
            }
            ParameterMode::Immediate => self.memory[self.instruction_pointer + position],
        }
    }
}

// TODO: Don't use strings for this
fn find_mode(opcode: isize, position: usize) -> ParameterMode {
    let mut opcode: Vec<char> = opcode.to_string().chars().into_iter().collect();
    opcode.reverse();
    match opcode.get(position + 1) {
        None | Some('0') => ParameterMode::Position,
        Some('1') => ParameterMode::Immediate,
        _ => panic!("Invalid mode for {:#?} at position {}", opcode, position),
    }
}

#[test]
fn find_mode_tests() {
    assert_eq!(1002 % 100, 2);
    assert_eq!(find_mode(1002, 1), ParameterMode::Position);
    assert_eq!(find_mode(1002, 2), ParameterMode::Immediate);
    assert_eq!(find_mode(1002, 3), ParameterMode::Position);
}

fn add(args: InstructionArguments<'_>, _input: &mut Vec<isize>, _output: &mut Vec<isize>) {
    let val = args.param(1) + args.param(2);
    let pos = args.memory[args.instruction_pointer + 3];
    args.memory[pos as usize] = val;
}

fn multiply(args: InstructionArguments<'_>, _input: &mut Vec<isize>, _output: &mut Vec<isize>) {
    let val = args.param(1) * args.param(2);
    let pos = args.memory[args.instruction_pointer + 3];
    args.memory[pos as usize] = val;
}

fn read_input(args: InstructionArguments<'_>, input: &mut Vec<isize>, _output: &mut Vec<isize>) {
    let val = input.pop().unwrap();
    let pos = args.memory[args.instruction_pointer + 1];
    args.memory[pos as usize] = val;
}

fn print_output(args: InstructionArguments<'_>, _input: &mut Vec<isize>, output: &mut Vec<isize>) {
    output.push(args.memory[args.instruction_pointer + 1]);
}

fn load() -> Result<Memory, std::io::Error> {
    Ok(fs::read_to_string("./input.txt")?
        .split(',')
        .map(|s| s.trim())
        .filter(|s| *s != "")
        .map(|s| s.parse::<isize>().unwrap())
        .collect())
}

fn run(memory: &mut Memory, mut input: Vec<isize>) -> Vec<isize>{
    let mut instruction_pointer = 0;

    let mut output = Vec::new();

    loop {
        let opcode = memory[instruction_pointer] % 100;
        let (instruction, values_in_instruction): (Box<dyn Fn(InstructionArguments<'_>, &mut Vec<isize>, &mut Vec<isize>)>, usize) =
            match opcode {
                1 => (Box::new(&add), 4),
                2 => (Box::new(&multiply), 4),
                3 => (Box::new(&read_input), 2),
                4 => (Box::new(&print_output), 2),
                99 => return output,
                _ => panic!(
                    "Invalid opcode {} at instruction_pointer {}",
                    opcode, instruction_pointer
                ),
            };
        instruction(InstructionArguments {
            memory,
            instruction_pointer,
        }, &mut input, &mut output);
        instruction_pointer += values_in_instruction;
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut memory = load()?;
    let output = run(&mut memory, vec![1]);
    println!("Ouputput: {:#?}", output);
    Ok(())
}

#[test]
fn tests() {
    // # Day 2 tests
    assert_eq!(run(vec![1, 0, 0, 0, 99]).unwrap(), vec![2, 0, 0, 0, 99]);
    assert_eq!(run(vec![2, 3, 0, 3, 99]).unwrap(), vec![2, 3, 0, 6, 99]);
    assert_eq!(
        run(vec![2, 4, 4, 5, 99, 0]).unwrap(),
        vec![2, 4, 4, 5, 99, 9801]
    );
    assert_eq!(
        run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).unwrap(),
        vec![30, 1, 1, 4, 2, 5, 6, 0, 99]
    );

    // # Day 5 tests
    // From instructions
    assert_eq!(
        run(vec![1002, 4, 3, 4, 33]).unwrap(),
        vec![1002, 4, 3, 4, 99]
    );
    assert_eq!(
        run(vec![1101, 100, -1, 4, 0]).unwrap(),
        vec![1101, 100, -1, 4, 99]
    );
    // Homemade
    assert_eq!(
        run(vec![1101, 100, -1, 4, 0, 1002, 4, 3, 8, 33]).unwrap(),
        vec![1101, 100, -1, 4, 0, 1002, 4, 3, 4, 99]
    );
}
