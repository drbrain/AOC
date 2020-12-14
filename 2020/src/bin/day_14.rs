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
    println!("part B: {}", day_14_b(&input));

    Ok(())
}

fn day_14_a(input: &str) -> u64 {
    let mut memory = Memory::from(input);

    memory.apply_values();

    memory.sum()
}

fn day_14_b(input: &str) -> u64 {
    let mut memory = Memory::from(input);

    memory.apply_addresses();

    memory.sum()
}

#[derive(Debug)]
struct Memory {
    instructions: Vec<Instruction>,
    memory: HashMap<usize, u64>,
}

impl Memory {
    fn apply_addresses(&mut self) {
        let mut mask = &Mask::default();

        for instruction in &self.instructions {
            match instruction {
                Instruction::Mask(m) => {
                    mask = m;
                }
                Instruction::Set(s) => {
                    for address in mask.apply_float(s.address) {
                        self.memory.insert(address, s.value);
                    }
                }
            }
        }
    }

    fn apply_values(&mut self) {
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
    xs: u64,
}

const TRIM: u64 = (1 << 36) - 1;

impl Mask {
    fn default() -> Mask {
        Mask {
            ones: 0,
            zeros: 0,
            xs: 0,
        }
    }

    #[allow(dead_code)]
    fn new(ones: u64, zeros: u64) -> Mask {
        let xs = TRIM ^ (ones | zeros);

        Mask { ones, zeros, xs }
    }

    fn apply(&self, input: u64) -> u64 {
        let one_masked = input | self.ones;
        let zero_masked = one_masked & !self.zeros;

        zero_masked & TRIM
    }

    fn apply_float(&self, input: usize) -> Vec<usize> {
        let x_bits = self.xs.count_ones() as usize;
        let mut result: Vec<usize> = Vec::with_capacity(x_bits);

        let one_masked = input as u64 | self.ones;
        let base = one_masked & !self.xs;

        let bit_sets = (0..1 << x_bits).map(|b| expand_to_bits(b, x_bits));

        for mut bits in bit_sets {
            let mut output = base;

            for i in 0..=36 {
                let offset = 1 << i;

                if self.xs & offset == offset {
                    let bit = bits.pop().unwrap();

                    output = if bit == 1 {
                        output | offset
                    } else {
                        (output & !offset) & TRIM
                    };
                }
            }

            result.push(output as usize);
        }

        result
    }
}

impl From<&str> for Mask {
    fn from(input: &str) -> Mask {
        let len = input.len() - 1;
        let ones = input
            .match_indices('1')
            .fold(0, |mask, (i, _)| mask | 1 << (len - i));
        let zeros = input
            .match_indices('0')
            .fold(0, |mask, (i, _)| mask | 1 << (len - i));
        let xs = TRIM ^ (ones | zeros);

        Mask { ones, zeros, xs }
    }
}

fn expand_to_bits(number: usize, max: usize) -> Vec<usize> {
    let mut bits: Vec<usize> = Vec::with_capacity(number);

    for i in 0..max {
        let offset = 1 << i;
        if number & offset == offset {
            bits.push(1);
        } else {
            bits.push(0);
        }
    }

    bits
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
    fn test_day_14_b() {
        let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

        assert_eq!(208, day_14_b(input));
    }

    #[test]
    fn test_day_14_expand_to_bits() {
        assert_eq!(expand_to_bits(0, 3), vec![0, 0, 0]);
        assert_eq!(expand_to_bits(1, 3), vec![1, 0, 0]);
        assert_eq!(expand_to_bits(2, 3), vec![0, 1, 0]);
        assert_eq!(expand_to_bits(3, 3), vec![1, 1, 0]);
        assert_eq!(expand_to_bits(6, 3), vec![0, 1, 1]);
        assert_eq!(expand_to_bits(7, 3), vec![1, 1, 1]);
    }

    #[test]
    fn test_day_14_mask_apply() {
        let mask = Mask::from("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");

        assert_eq!(73, mask.apply(11));
        assert_eq!(101, mask.apply(101));
        assert_eq!(64, mask.apply(0));
    }

    #[test]
    fn test_day_14_mask_apply_float() {
        let mask = Mask::from("000000000000000000000000000000X1001X");

        assert_eq!(vec![26, 58, 27, 59], mask.apply_float(42));

        let mask = Mask::from("00000000000000000000000000000000X0XX");

        assert_eq!(vec![16, 24, 18, 26, 17, 25, 19, 27], mask.apply_float(26));
    }
}
