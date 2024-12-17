type Registers = [usize; 3];

#[derive(Debug, PartialEq)]
enum Instruction {
    Adv, // 0
    Bxl, // 1
    Bst, // 2
    Jnz, // 3
    Bxc, // 4
    Out, // 5
    Bdv, // 6
    Cdv, // 7
}

impl From<u8> for Instruction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!("Invalid opcode: {}", value),
        }
    }
}

fn parse_input(input: &str) -> (Registers, Vec<u8>) {
    let mut registers = [0; 3];
    let mut program = Vec::new();

    for line in input.lines().map(|l| l.trim()).filter(|l| !l.is_empty()) {
        if line.starts_with("Register A:") {
            registers[0] = line.split_whitespace().last().unwrap().parse().unwrap();
        } else if line.starts_with("Register B:") {
            registers[1] = line.split_whitespace().last().unwrap().parse().unwrap();
        } else if line.starts_with("Register C:") {
            registers[2] = line.split_whitespace().last().unwrap().parse().unwrap();
        } else if line.starts_with("Program:") {
            program = line["Program:".len()..]
                .split(',')
                .map(|x| x.trim().parse::<u8>().unwrap())
                .collect();
        }
    }
    (registers, program)
}

fn combo(registers: &Registers, operand: u8) -> usize {
    match operand {
        0..=3 => operand as usize,       // Literal values 0-3
        4 => registers[0],               // Register A
        5 => registers[1],               // Register B
        6 => registers[2],               // Register C
        _ => panic!("Invalid combo operand: {}", operand),
    }
}

fn run(mut registers: Registers, program: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut ip = 0; // Instruction pointer

    while ip < program.len() {
        let opcode = program[ip];
        let operand = program[ip + 1];
        ip += 2;

        let instruction: Instruction = opcode.into();

        match instruction {
            Instruction::Adv => {
                let value = 2_usize.pow(combo(&registers, operand) as u32);
                registers[0] /= value;
            }
            Instruction::Bxl => {
                registers[1] ^= operand as usize;
            }
            Instruction::Bst => {
                registers[1] = combo(&registers, operand) % 8;
            }
            Instruction::Jnz => {
                if registers[0] != 0 {
                    ip = operand as usize;
                }
            }
            Instruction::Bxc => {
                registers[1] ^= registers[2];
            }
            Instruction::Out => {
                let value = combo(&registers, operand) % 8;
                output.push(value as u8);
            }
            Instruction::Bdv => {
                let value = 2_usize.pow(combo(&registers, operand) as u32);
                registers[1] = registers[0] / value;
            }
            Instruction::Cdv => {
                let value = 2_usize.pow(combo(&registers, operand) as u32);
                registers[2] = registers[0] / value;
            }
        }
    }

    output
}

pub fn solve(input: &str) -> String {
    let (registers, program) = parse_input(input);
    let output = run(registers, &program);
    output
        .into_iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    let input = "\
    Register A: 66752888
    Register B: 0
    Register C: 0

    Program:  2,4,1,7,7,5,1,7,0,3,4,1,5,5,3,0
    ";
    let result = solve(input);
    println!("Output: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
        Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0
        ";
        assert_eq!(solve(input), "4,6,3,5,6,3,5,2,1,0");
    }
}
