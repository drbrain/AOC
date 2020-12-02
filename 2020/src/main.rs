use anyhow::Result;

use itertools::Itertools;

use std::fs::File;
use std::io;
use std::io::Read;
use std::ops::Range;
use std::path::Path;
use std::str::FromStr;

fn main() -> Result<()> {
    //let input = read("./01.input")?;

    //println!("part A: {}", day_1(&input, 2)?);
    //println!("part B: {}", day_1(&input, 3)?);

    let input = read("./02.input")?;

    //println!("part A: {}", day_2_a(&input)?);
    println!("part B: {}", day_2_b(&input)?);

    Ok(())
}

fn day_1(input: &String, entries: usize) -> Result<u32> {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let entries: Vec<u32> = numbers
        .into_iter()
        .combinations(entries)
        .find(|pair| pair.iter().fold(0, |a, b| a + b) == 2020)
        .unwrap();

    let answer = entries.iter().fold(1, |a, b| a * b);

    Ok(answer)
}

fn day_2_a(input: &String) -> Result<usize> {
    let valid = input
        .lines()
        .map(|line| parse_and_check_a(line))
        .filter(|ok| *ok)
        .count();

    Ok(valid)
}

fn day_2_b(input: &String) -> Result<usize> {
    let valid = input
        .lines()
        .map(|line| parse_and_check_b(line))
        .filter(|ok| *ok)
        .count();

    Ok(valid)
}

fn parse_and_check_a(line: &str) -> bool {
    let split: Vec<&str> = line.split(": ").collect();

    let policy: Vec<&str> = split[0].split(' ').collect();
    let password = split[1];

    let range: Vec<&str> = policy[0].split('-').collect();
    let letter = char::from_str(policy[1]).unwrap();

    let start = range[0].parse::<usize>().unwrap();
    let end = range[1].parse::<usize>().unwrap() + 1;

    let range = Range { start, end };

    range.contains(&password.chars().filter(|c| c == &letter).count())
}

fn parse_and_check_b(line: &str) -> bool {
    let split: Vec<&str> = line.split(": ").collect();

    let policy: Vec<&str> = split[0].split(' ').collect();
    let password = split[1];

    let positions: Vec<&str> = policy[0].split('-').collect();
    let letter = policy[1];

    let first = positions[0].parse::<usize>().unwrap() - 1;
    let second = positions[1].parse::<usize>().unwrap() - 1;

    match (&password[first..first + 1] == letter, &password[second..second + 1] == letter) {
        (true, false) => true,
        (false, true) => true,

        (false, false) => false,
        (true, true) => false,
    }
}

fn read<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut result = String::new();
    let mut file = File::open(filename)?;

    file.read_to_string(&mut result)?;

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_1_a() {
        let input = String::from("1721\n979\n366\n299\n675\n1456\n");

        assert_eq!(514579, day_1(&input, 2).unwrap());
    }

    #[test]
    fn test_day_1_b() {
        let input = String::from("1721\n979\n366\n299\n675\n1456\n");

        assert_eq!(241861950, day_1(&input, 3).unwrap());
    }

    #[test]
    fn test_day_2_a() {
        let input = String::from(
            "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
        );

        assert_eq!(2, day_2_a(&input).unwrap());
    }

    #[test]
    fn test_day_2_b() {
        let input = String::from(
            "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc",
        );

        assert_eq!(1, day_2_b(&input).unwrap());
    }
}
