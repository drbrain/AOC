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
    //println!("part B: {}", day_13_b(&input));

    Ok(())
}

fn day_13_a(input: &str) -> u32 {
    let schedule = Schedule::from(input);

    let (arrival_timestamp, bus_id) = schedule.next_bus();

    let offset = arrival_timestamp - schedule.timestamp;

    offset * bus_id
}

//fn day_13_b(input: &str) -> u32 {
//}

#[derive(Debug)]
struct Schedule {
    timestamp: u32,
    buses: Vec<Option<u32>>,
}

impl Schedule {
    fn next_bus(&self) -> (u32, u32) {
        let in_service = self.in_service();

        for ts in self.timestamp.. {
            if let Some(bus_id) = in_service.iter().find(|v| ts % *v == 0) {
                return (ts, *bus_id);
            }
        }

        unreachable!();
    }

    fn in_service(&self) -> Vec<u32> {
        self.buses.iter().filter_map(|id| *id).collect()
    }
}

impl From<&str> for Schedule {
    fn from(input: &str) -> Schedule {
        let (timestamp, buses) = parse(input).unwrap().1;

        Schedule { timestamp, buses }
    }
}

fn parse(input: &str) -> IResult<&str, (u32, Vec<Option<u32>>)> {
    tuple((number, (preceded(tag("\n"), buses))))(input)
}

fn buses(input: &str) -> IResult<&str, Vec<Option<u32>>> {
    separated_list1(tag(","), alt((opt_number, x)))(input)
}

fn x(input: &str) -> IResult<&str, Option<u32>> {
    map(tag("x"), |_| None)(input)
}

fn opt_number(input: &str) -> IResult<&str, Option<u32>> {
    map(recognize(digit1), |s: &str| s.parse::<u32>().ok())(input)
}

fn number(input: &str) -> IResult<&str, u32> {
    map(recognize(digit1), |s: &str| s.parse::<u32>().unwrap())(input)
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
