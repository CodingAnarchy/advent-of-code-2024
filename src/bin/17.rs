advent_of_code::solution!(17);

use itertools::Itertools;
use std::ops::ControlFlow;

#[derive(Clone, Debug)]
struct Instruction {
    opcode: u8,
    operand: u8,
}

impl Instruction {
    fn new(opcode: u8, operand: u8) -> Self {
        Self { opcode, operand }
    }
}

#[derive(Clone, Debug)]
struct Computer {
    registers: [usize; 3],
    ip: usize,
    instructions: Vec<Instruction>,
}

impl Computer {
    fn new(input: &str) -> Self {
        let registers = input
            .lines()
            .take(3)
            .map(|line| line.split_whitespace().last().unwrap().parse().unwrap())
            .collect::<Vec<_>>();

        let instructions = input
            .lines()
            .last()
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .split(',')
            .chunks(2)
            .into_iter()
            .map(|mut inst| {
                let (opcode, operand) = (
                    inst.next().unwrap().parse().unwrap(),
                    inst.next().unwrap().parse().unwrap(),
                );

                Instruction::new(opcode, operand)
            })
            .collect::<Vec<_>>();

        Self {
            registers: [registers[0], registers[1], registers[2]],
            ip: 0,
            instructions,
        }
    }

    fn from_instructions(registers: [usize; 3], instructions: Vec<usize>) -> Self {
        let instructions = instructions
            .chunks(2)
            .into_iter()
            .map(|inst| {
                let (opcode, operand) = (inst[0] as u8, inst[1] as u8);

                Instruction::new(opcode, operand)
            })
            .collect::<Vec<_>>();

        Self {
            registers,
            ip: 0,
            instructions,
        }
    }

    fn combo_operand(&self, inst: &Instruction) -> usize {
        match inst.operand {
            0..=3 => inst.operand as usize,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!("Invalid operand: reserved combo operand"),
        }
    }

    fn run(&mut self) -> Option<usize> {
        while self.ip < self.instructions.len() {
            let instruction = &self.instructions[self.ip];
            let mut jmp = false;

            match instruction.opcode {
                0 => {
                    self.registers[0] =
                        self.registers[0] / 2usize.pow(self.combo_operand(instruction) as u32);
                }
                1 => {
                    self.registers[1] = self.registers[1] ^ instruction.operand as usize;
                }
                2 => {
                    self.registers[1] = self.combo_operand(instruction) % 8;
                }
                3 => {
                    if self.registers[0] != 0 {
                        self.ip = instruction.operand as usize;
                        jmp = true;
                    }
                }
                4 => {
                    self.registers[1] = self.registers[1] ^ self.registers[2];
                }
                5 => {
                    self.ip += 1;
                    return Some(self.combo_operand(instruction) % 8);
                }
                6 => {
                    self.registers[1] =
                        self.registers[0] / 2usize.pow(self.combo_operand(instruction) as u32);
                }
                7 => {
                    self.registers[2] =
                        self.registers[0] / 2usize.pow(self.combo_operand(instruction) as u32);
                }
                _ => return None, // machine halts if there is no valid opcode
            }

            if !jmp {
                self.ip += 1;
            }
        }

        None
    }

    fn execute_instructions(&mut self) -> Vec<usize> {
        let mut output = Vec::new();

        while let Some(out) = self.run() {
            output.push(out);
        }

        output
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut computer = Computer::new(input);

    let output = computer.execute_instructions();
    Some(output.iter().join(",").to_string())
}

fn inner(program: &Vec<usize>, index: isize, a: usize) -> ControlFlow<usize> {
    if index < 0 {
        return ControlFlow::Break(a);
    }

    // Try all 8 combinations of lower bits
    for i in 0..8 {
        let next_a = (a << 3) | i;
        let out = Computer::from_instructions([next_a, 0, 0], program.clone())
            .run()
            .unwrap();

        if out == program[index as usize] {
            inner(program, index - 1, next_a)?;
        }
    }

    ControlFlow::Continue(())
}

pub fn part_two(input: &str) -> Option<usize> {
    let orig_computer = Computer::new(input);
    let program = orig_computer
        .instructions
        .iter()
        .flat_map(|inst| [inst.opcode as usize, inst.operand as usize])
        .collect::<Vec<usize>>();

    let a = inner(&program, program.len() as isize - 1, 0)
        .break_value()
        .unwrap();

    Some(a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
