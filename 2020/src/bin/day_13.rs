use anyhow::Result;

use aoc2020::read;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::convert::From;

fn main() -> Result<()> {
    let input = read("./13.input")?;

    println!("part A: {}", day_13_a(&input));
    println!("part B: {}", day_13_b(&input));

    Ok(())
}

fn day_13_a(input: &str) -> i64 {
    let schedule = Schedule::from(input);

    let (arrival_timestamp, bus_id) = schedule.next_bus();

    let offset = arrival_timestamp - schedule.timestamp;

    offset * bus_id
}

fn day_13_b(input: &str) -> i64 {
    let schedule = Schedule::from(input);

    schedule.gold_coin()
}

#[derive(Debug)]
struct Schedule {
    timestamp: i64,
    buses: Vec<Option<i64>>,
}

impl Schedule {
    fn next_bus(&self) -> (i64, i64) {
        let in_service = self.in_service();

        for ts in self.timestamp.. {
            if let Some(bus_id) = in_service.iter().find(|v| ts % *v == 0) {
                return (ts, *bus_id);
            }
        }

        unreachable!();
    }

    // ts = ts % 7 == ts + 0
    // ts = ts % 13 == ts + 1
    // ts = ts % 59 == ts + 4
    // ts = ts % 31 == ts + 6
    // ts = ts % 19 == ts + 7

    // ts = ts / 7
    // ts = (ts + 1) / 13
    // ts = ts % 59 == ts + 4
    // ts = ts % 31 == ts + 6
    // ts = ts % 19 == ts + 7
    fn gold_coin(&self) -> i64 {
        chinese_remainder(&self.residues(), &self.in_service()).unwrap()
    }

    fn in_service(&self) -> Vec<i64> {
        self.buses.iter().filter_map(|id| *id).collect()
    }

    fn residues(&self) -> Vec<i64> {
        self.buses
            .iter()
            .enumerate()
            .filter_map(|(i, id)| match id {
                Some(id) => Some(id - i as i64),
                None => None,
            })
            .collect::<Vec<i64>>()
    }
}

impl From<&str> for Schedule {
    fn from(input: &str) -> Schedule {
        let (timestamp, buses) = parse(input).unwrap().1;

        Schedule { timestamp, buses }
    }
}

// https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

fn parse(input: &str) -> IResult<&str, (i64, Vec<Option<i64>>)> {
    tuple((number, (preceded(tag("\n"), buses))))(input)
}

fn buses(input: &str) -> IResult<&str, Vec<Option<i64>>> {
    separated_list1(tag(","), alt((opt_number, x)))(input)
}

fn x(input: &str) -> IResult<&str, Option<i64>> {
    map(tag("x"), |_| None)(input)
}

fn opt_number(input: &str) -> IResult<&str, Option<i64>> {
    map(recognize(digit1), |s: &str| s.parse::<i64>().ok())(input)
}

fn number(input: &str) -> IResult<&str, i64> {
    map(recognize(digit1), |s: &str| s.parse::<i64>().unwrap())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_13_a() {
        let input = "939
7,13,x,x,59,x,31,19";

        assert_eq!(295, day_13_a(input));
    }

    #[test]
    fn test_day_13_b() {
        assert_eq!(day_13_b("0\n17,x,13,19"), 3417);
        assert_eq!(day_13_b("0\n67,7,59,61"), 754018);
        assert_eq!(day_13_b("0\n67,x,7,59,61"), 779210);
        assert_eq!(day_13_b("0\n7,13,x,x,59,x,31,19"), 1068781);
        assert_eq!(day_13_b("0\n67,7,x,59,61"), 1261476);
        assert_eq!(day_13_b("0\n1789,37,47,1889"), 1202161486);
    }

    #[test]
    fn test_day_13_schedule_from() {
        let input = "939
7,13,x,x,59,x,31,19";

        let schedule: Schedule = input.into();

        assert_eq!(939, schedule.timestamp);

        let expected = vec![
            Some(7),
            Some(13),
            None,
            None,
            Some(59),
            None,
            Some(31),
            Some(19),
        ];
        assert_eq!(expected, schedule.buses);
    }
}
