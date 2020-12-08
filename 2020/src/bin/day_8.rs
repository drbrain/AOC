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

fn main() -> Result<()> {
    let input = read("./08.input")?;

    println!("part A: {}", day_8_a(&input));
    //println!("part B: {}", day_8_b(&input));

    Ok(())
}

fn day_8_a(instructions: &str) -> i32 {
    let instructions = Instructions::from(instructions);

    let mut machine = Machine::new(instructions);

    machine.run().err().unwrap()
}

//fn day_8_b(instructions: &str) -> i32 {
//    let instructions = Instructions::from(instructions);
//
//    let mut machine = Machine::new(instructions);
//
//    machine.fix_corrupt()
//}

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
            None => { return Err(format!("PC overflow, pc: {} acc: {}", self.pc, self.acc)); },
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
                Err(_) => { return Ok(self.acc); },
            }
        }
    }
}

#[derive(Debug)]
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
}

impl From<&str> for Instructions {
    fn from(input: &str) -> Instructions {
        instructions(input).unwrap().1
    }
}

#[derive(Debug)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
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

//    #[test]
//    fn test_day_8_b() {
//        let input = "nop +0
//acc +1
//jmp +4
//acc +3
//jmp -3
//acc -99
//acc +1
//jmp -4
//acc +6";
//
//        assert_eq!(8, day_8_b(input));
//    }
}
