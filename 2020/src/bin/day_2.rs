use anyhow::Result;

use aoc2020::read;

use std::ops::Range;
use std::str::FromStr;

fn main() -> Result<()> {
    let input = read("./02.input")?;

    println!("part A: {}", day_2_a(&input)?);
    println!("part B: {}", day_2_b(&input)?);

    Ok(())
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

    match (
        &password[first..first + 1] == letter,
        &password[second..second + 1] == letter,
    ) {
        (true, false) => true,
        (false, true) => true,

        (false, false) => false,
        (true, true) => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

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
