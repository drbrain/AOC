use anyhow::Result;

use aoc2020::read;

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::IResult;

use std::collections::HashMap;
use std::convert::From;

fn main() -> Result<()> {
    let input = read("./15.input")?;

    println!("part A: {}", day_15_a(&input));
    //println!("part B: {}", day_15_b(&input));

    Ok(())
}

fn day_15_a(input: &str) -> u64 {
    let mut game = MemoryGame::from(input);

    game.play(2020);

    *game.numbers.last().unwrap()
}

//fn day_15_b(input: &str) -> u64 {
//}

#[derive(Debug)]
struct MemoryGame {
    numbers: Vec<u64>,
    last_seen: HashMap<u64, (u64, usize, usize)>,
}

impl MemoryGame {
    fn play(&mut self, until: usize) {
        for turn in 0..until {
            if turn < self.numbers.len() {
                let number = self.numbers[turn];

                self.speak(number, turn);
            } else {
                let last = self.numbers.last().unwrap();

                let (count, a, b) = self.last_spoken(last);

                if count == 1 {
                    self.speak(0, turn);
                } else {
                    let number = a - b;

                    self.speak(number as u64, turn);
                }
            }
        }
    }

    fn speak(&mut self, number: u64, turn: usize) {
        let (count, previous, _) = self.last_spoken(&number);

        self.last_seen.insert(number, (count + 1, turn, previous));

        if turn >= self.numbers.len() {
            self.numbers.push(number);
        }
    }

    fn last_spoken(&self, number: &u64) -> (u64, usize, usize) {
        match self.last_seen.get(number) {
            Some((c, a, b)) => (*c, *a, *b),
            None => (0, 0, 0),
        }
    }
}

impl From<&str> for MemoryGame {
    fn from(input: &str) -> MemoryGame {
        let numbers = numbers(input).unwrap().1;
        let last_seen = HashMap::new();

        MemoryGame { numbers, last_seen }
    }
}

fn numbers(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), num_u64)(input)
}

fn num_u64(input: &str) -> IResult<&str, u64> {
    map(recognize(digit1), |s: &str| s.parse::<u64>().unwrap())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_15_a() {
        let input = "0,3,6";

        assert_eq!(436, day_15_a(input));
    }

    #[test]
    fn test_day_15_game_play() {
        let mut game = MemoryGame::from("0,3,6");

        game.play(10);

        assert_eq!(0, game.numbers[3]);
        assert_eq!(3, game.numbers[4]);
        assert_eq!(3, game.numbers[5]);
        assert_eq!(1, game.numbers[6]);
        assert_eq!(0, game.numbers[7]);
        assert_eq!(4, game.numbers[8]);
        assert_eq!(0, game.numbers[9]);
    }
}
