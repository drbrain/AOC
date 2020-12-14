use anyhow::Result;

use aoc2020::read;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::collections::HashMap;
use std::convert::From;

fn main() -> Result<()> {
    let input = read("./14.input")?;

    println!("part A: {}", day_14_a(&input));
    //println!("part B: {}", day_14_b(&input));

    Ok(())
}

fn day_14_a(input: &str) -> u64 {
    let mut memory = Memory::from(input);

    memory.apply();

    memory.sum()
}

//fn day_14_b(input: &str) -> i64 {
//    0
//}

#[derive(Debug)]
struct Memory {
    instructions: Vec<Instruction>,
    memory: HashMap<usize, u64>,
}

impl Memory {
    fn apply(&mut self) {
        let mut mask = &Mask::default();

        for instruction in &self.instructions {
            match instruction {
                Instruction::Mask(m) => {
                    mask = m;
                }
                Instruction::Set(s) => {
                    let address = s.address;
                    let value = mask.apply(s.value);

                    self.memory.insert(address, value);
                }
            }
        }
    }

    fn sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

impl From<&str> for Memory {
    fn from(input: &str) -> Memory {
        let instructions = instructions(input).unwrap().1;
        let memory = HashMap::new();

        Memory {
            instructions,
            memory,
        }
    }
}

#[derive(Debug)]
struct Mask {
    ones: u64,
    zeros: u64,
}

impl Mask {
    fn default() -> Mask {
        Mask { ones: 0, zeros: 0 }
    }

    #[allow(dead_code)]
    fn new(ones: u64, zeros: u64) -> Mask {
        Mask { ones, zeros }
    }

    fn apply(&self, input: u64) -> u64 {
        let one_masked = input | self.ones;
        let zero_masked = one_masked & self.zeros;
        let trim = (1 << 37) - 1;

        zero_masked & trim
    }
}

impl From<&str> for Mask {
    fn from(input: &str) -> Mask {
        let len = input.len() - 1;
        let ones = input
            .match_indices("1")
            .fold(0, |mask, (i, _)| mask | 1 << (len - i));
        let zeros = !input
            .match_indices("0")
            .fold(0, |mask, (i, _)| mask | 1 << (len - i));

        Mask { ones, zeros }
    }
}

#[derive(Debug)]
struct Set {
    address: usize,
    value: u64,
}

impl From<(usize, u64)> for Set {
    fn from(input: (usize, u64)) -> Set {
        let (address, value) = input;

        Set { address, value }
    }
}

#[derive(Debug)]
enum Instruction {
    Mask(Mask),
    Set(Set),
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag("\n"), instruction)(input)
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((mask, set))(input)
}

fn mask(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("mask = "), not_line_ending), |m| {
        Instruction::Mask(Mask::from(m))
    })(input)
}

fn set(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("mem["), num_usize),
            preceded(tag("] = "), num_u64),
        )),
        |t| Instruction::Set(Set::from(t)),
    )(input)
}

fn num_u64(input: &str) -> IResult<&str, u64> {
    map(recognize(digit1), |s: &str| s.parse::<u64>().unwrap())(input)
}

fn num_usize(input: &str) -> IResult<&str, usize> {
    map(recognize(digit1), |s: &str| s.parse::<usize>().unwrap())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_14_a() {
        let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

        assert_eq!(165, day_14_a(input));
    }

    #[test]
    fn test_day_14_mask_apply() {
        let mask = Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

        assert_eq!(73, mask.apply(11));
        assert_eq!(101, mask.apply(101));
        assert_eq!(64, mask.apply(0));
    }
}
