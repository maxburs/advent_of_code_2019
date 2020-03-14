use std::fs;
#[macro_use]
extern crate log;

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
        let result = match mode {
            ParameterMode::Position => {
                self.memory[self.memory[self.instruction_pointer + position] as usize]
            }
            ParameterMode::Immediate => self.memory[self.instruction_pointer + position],
        };
        info!("param {} result: {}", position, result);
        result
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
    tests::init();
    assert_eq!(1002 % 100, 2);
    assert_eq!(find_mode(1002, 1), ParameterMode::Position);
    assert_eq!(find_mode(1002, 2), ParameterMode::Immediate);
    assert_eq!(find_mode(1002, 3), ParameterMode::Position);
}

fn add(args: InstructionArguments<'_>, _input: &mut Vec<isize>, _output: &mut Vec<isize>) {
    let val = args.param(1) + args.param(2);
    info!("{} = {} + {}", val, args.param(1), args.param(2));
    info!("{}", "-".repeat(20));
    let pos = args.memory[args.instruction_pointer + 3];
    args.memory[pos as usize] = val;
}

fn multiply(args: InstructionArguments<'_>, _input: &mut Vec<isize>, _output: &mut Vec<isize>) {
    let val = args.param(1) * args.param(2);
    info!("{} = {} * {}", val, args.param(1), args.param(2));
    info!("{}", "-".repeat(20));
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
    // if args.memory[args.instruction_pointer + 1] != 0 {
    //     panic!("exiting early with output: {:?}", output);
    // }
}

fn load() -> Result<Memory, std::io::Error> {
    Ok(fs::read_to_string("./input.txt")?
        .split(',')
        .map(|s| s.trim())
        .filter(|s| *s != "")
        .map(|s| s.parse::<isize>().unwrap())
        .collect())
}

type InstructionFnc = Box<dyn Fn(InstructionArguments<'_>, &mut Vec<isize>, &mut Vec<isize>)>;

fn run(memory: &mut Memory, mut input: Vec<isize>) -> (Vec<isize>, Result<(), std::io::Error>) {
    let mut instruction_pointer = 0;
    let mut output = Vec::new();

    loop {
        info!("{:?}", memory);
        info!("{}", "-".repeat(20));
        let opcode = memory[instruction_pointer] % 100;
        let (instruction, values_in_instruction): (InstructionFnc, usize) = match opcode {
            1 => (Box::new(&add), 4),
            2 => (Box::new(&multiply), 4),
            3 => (Box::new(&read_input), 2),
            4 => (Box::new(&print_output), 2),
            99 => return (output, Ok(())),
            _ => {
                let msg = format!(
                    "Invalid opcode {} at instruction_pointer {}",
                    opcode, instruction_pointer
                );
                return (
                    output,
                    Err(std::io::Error::new(std::io::ErrorKind::Other, msg)),
                );
            }
        };
        info!(
            "Running: {:?}",
            &memory[instruction_pointer..(instruction_pointer + values_in_instruction)]
        );
        info!("{}", "-".repeat(20));
        instruction(
            InstructionArguments {
                memory,
                instruction_pointer,
            },
            &mut input,
            &mut output,
        );
        instruction_pointer += values_in_instruction;
    }
}

fn main() -> Result<(), std::io::Error> {
    env_logger::init();

    let mut memory = load()?;
    let (output, err) = run(&mut memory, vec![1]);

    info!("Output: {:#?}", output);
    if let Ok(()) = err {
        info!("Success");
    }

    return err;
}


#[cfg(test)]
mod tests {
    use super::*;

    pub fn init() {
        let _ = env_logger::builder().is_test(true).format_timestamp(None).try_init();
    }

    fn test_ok(mut program: Vec<isize>, args: Vec<isize>, result: Vec<isize>) {
        let (_output, err) = run(&mut program, args);
        err.unwrap();
        assert_eq!(program, result);
    }

    #[test]
    fn day_2() {
        init();

        test_ok(vec![1, 0, 0, 0, 99], vec![], vec![2, 0, 0, 0, 99]);
        test_ok(vec![2, 3, 0, 3, 99], vec![], vec![2, 3, 0, 6, 99]);
        test_ok(vec![2, 4, 4, 5, 99, 0], vec![], vec![2, 4, 4, 5, 99, 9801]);
        test_ok(
            vec![1, 1, 1, 4, 99, 5, 6, 0, 99],
            vec![],
            vec![30, 1, 1, 4, 2, 5, 6, 0, 99],
        );
    }

    #[test]
    fn day_5() {
        init();

        // From instructions
        test_ok(vec![1002, 4, 3, 4, 33], vec![], vec![1002, 4, 3, 4, 99]);
        test_ok(
            vec![1101, 100, -1, 4, 0],
            vec![],
            vec![1101, 100, -1, 4, 99],
        );
    }

    #[test]
    fn homemade() {
        init();

        test_ok(
            vec![1101, 100, -1, 4, 0, 1002, 4, 3, 5],
            vec![],
            vec![1101, 100, -1, 4, 99, -4, 3, 5],
        );
    }
}
