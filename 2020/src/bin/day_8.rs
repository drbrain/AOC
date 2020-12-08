use anyhow::Result;

use aoc2020::read;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::collections::HashSet;
use std::convert::From;
use std::fmt;

fn main() -> Result<()> {
    let input = read("./08.input")?;

    println!("part A: {}", day_8_a(&input));
    println!("part B: {}", day_8_b(&input));

    Ok(())
}

fn day_8_a(instructions: &str) -> i32 {
    let instructions = Instructions::from(instructions);

    let mut machine = Machine::new(instructions);

    machine.run().err().unwrap()
}

fn day_8_b(instructions: &str) -> i32 {
    let instructions = Instructions::from(instructions);
    let mutator = InstructionMutator::new(instructions);

    for instructions in mutator {
        let mut machine = Machine::new(instructions);

        match machine.run() {
            Ok(acc) => {
                return acc;
            }
            Err(_) => (),
        }
    }

    0
}

#[derive(Debug)]
struct Machine {
    instructions: Instructions,
    acc: i32,
    pc: usize,
}

impl Machine {
    fn new(instructions: Instructions) -> Self {
        let acc = 0;
        let pc = 0;

        Machine {
            instructions,
            acc,
            pc,
        }
    }

    fn execute(&mut self) -> Result<(), String> {
        let instruction = match self.instructions.at(self.pc) {
            Some(i) => i,
            None => {
                return Err(format!("PC overflow, pc: {} acc: {}", self.pc, self.acc));
            }
        };

        match instruction {
            Instruction::Acc(n) => {
                self.acc += n;
                self.pc += 1;
            }
            Instruction::Jmp(n) => {
                self.pc = (self.pc as i32 + n) as usize;
            }
            Instruction::Nop(_n) => {
                self.pc += 1;
            }
        }

        Ok(())
    }

    fn run(&mut self) -> Result<i32, i32> {
        let mut visited = HashSet::new();

        loop {
            if visited.contains(&self.pc) {
                return Err(self.acc);
            } else {
                visited.insert(self.pc);
            }

            match self.execute() {
                Ok(_) => (),
                Err(_) => {
                    return Ok(self.acc);
                }
            }
        }
    }
}

#[derive(Debug)]
struct InstructionMutator {
    instructions: Instructions,
    max: usize,
    pc: usize,
}

impl InstructionMutator {
    fn new(instructions: Instructions) -> Self {
        let max = instructions.len();
        let pc = 0;

        InstructionMutator {
            instructions,
            max,
            pc,
        }
    }
}

impl Iterator for InstructionMutator {
    type Item = Instructions;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pc == self.max {
            return None;
        }

        self.instructions.swap(self.pc);

        let instructions = self.instructions.clone();

        self.instructions.swap(self.pc);
        self.pc += 1;

        Some(instructions)
    }
}

#[derive(Clone, Debug)]
struct Instructions {
    instructions: Vec<Instruction>,
}

impl Instructions {
    fn at(&self, pc: usize) -> Option<&Instruction> {
        if pc < self.instructions.len() {
            Some(&self.instructions[pc])
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.instructions.len()
    }

    fn swap(&mut self, pc: usize) {
        match self.instructions[pc] {
            Instruction::Acc(_) => (),
            _ => {
                let instruction = self.instructions.remove(pc).swap();
                self.instructions.insert(pc, instruction);
            }
        }
    }
}

impl From<&str> for Instructions {
    fn from(input: &str) -> Instructions {
        instructions(input).unwrap().1
    }
}

#[derive(Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

impl Instruction {
    fn swap(self) -> Self {
        match self {
            Instruction::Jmp(n) => Instruction::Nop(n),
            Instruction::Nop(n) => Instruction::Jmp(n),
            Instruction::Acc(_) => unreachable!(),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Acc(n) => f.write_fmt(format_args!("acc {:+}", n)),
            Instruction::Jmp(n) => f.write_fmt(format_args!("jmp {:+}", n)),
            Instruction::Nop(n) => f.write_fmt(format_args!("nop {:+}", n)),
        }
    }
}

fn instructions(input: &str) -> IResult<&str, Instructions> {
    map(
        separated_list1(tag("\n"), instruction),
        |instructions: Vec<Instruction>| Instructions { instructions },
    )(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((acc, jmp, nop))(input)
}

fn acc(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("acc "), number), |n: i32| Instruction::Acc(n))(input)
}

fn jmp(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("jmp "), number), |n: i32| Instruction::Jmp(n))(input)
}

fn nop(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("nop "), number), |n: i32| Instruction::Nop(n))(input)
}

fn number(input: &str) -> IResult<&str, i32> {
    map(recognize(preceded(one_of("+-"), digit1)), |s: &str| {
        s.parse::<i32>().unwrap()
    })(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_8_a() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        assert_eq!(5, day_8_a(input));
    }

    #[test]
    fn test_day_8_b() {
        let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

        assert_eq!(8, day_8_b(input));
    }
}
