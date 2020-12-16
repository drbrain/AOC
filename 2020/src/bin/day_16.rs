use anyhow::Result;

use aoc2020::read;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::ops::Range;

fn main() -> Result<()> {
    let input = read("./16.input")?;

    println!("part A: {}", day_16_a(&input));
    //println!("part B: {}", day_15_b(&input));

    Ok(())
}

fn day_16_a(input: &str) -> u64 {
    let (notes, _, nearby) = parse(input).unwrap().1;

    let invalid = find_all_invalid(&nearby, &notes);

    invalid.iter().sum()
}

fn find_all_invalid(tickets: &Vec<Ticket>, notes: &Vec<Note>) -> Vec<u64> {
    tickets
        .iter()
        .filter_map(|ticket| find_invalid(ticket, notes))
        .flatten()
        .collect()
}

fn find_invalid(ticket: &Ticket, notes: &Vec<Note>) -> Option<Vec<u64>> {
    let invalid: Vec<u64> = ticket
        .fields
        .iter()
        .filter(|field| !validate(field, notes))
        .copied()
        .collect();

    if invalid.is_empty() {
        None
    } else {
        Some(invalid)
    }
}

fn validate(field: &u64, notes: &Vec<Note>) -> bool {
    notes.iter().any(|n| n.validate(field))
}

#[derive(Debug)]
enum Note {
    DepartureLocation((Range<u64>, Range<u64>)),
    DepartureStation((Range<u64>, Range<u64>)),
    DeparturePlatform((Range<u64>, Range<u64>)),
    DepartureTrack((Range<u64>, Range<u64>)),
    DepartureDate((Range<u64>, Range<u64>)),
    DepartureTime((Range<u64>, Range<u64>)),
    ArrivalLocation((Range<u64>, Range<u64>)),
    ArrivalStation((Range<u64>, Range<u64>)),
    ArrivalPlatform((Range<u64>, Range<u64>)),
    ArrivalTrack((Range<u64>, Range<u64>)),
    Class((Range<u64>, Range<u64>)),
    Duration((Range<u64>, Range<u64>)),
    Price((Range<u64>, Range<u64>)),
    Route((Range<u64>, Range<u64>)),
    Row((Range<u64>, Range<u64>)),
    Seat((Range<u64>, Range<u64>)),
    Train((Range<u64>, Range<u64>)),
    Type((Range<u64>, Range<u64>)),
    Wagon((Range<u64>, Range<u64>)),
    Zone((Range<u64>, Range<u64>)),
}

impl Note {
    fn validate(&self, field: &u64) -> bool {
        let (r1, r2) = match self {
            Note::DepartureLocation((r1, r2)) => (r1, r2),
            Note::DepartureStation((r1, r2)) => (r1, r2),
            Note::DeparturePlatform((r1, r2)) => (r1, r2),
            Note::DepartureTrack((r1, r2)) => (r1, r2),
            Note::DepartureDate((r1, r2)) => (r1, r2),
            Note::DepartureTime((r1, r2)) => (r1, r2),
            Note::ArrivalLocation((r1, r2)) => (r1, r2),
            Note::ArrivalStation((r1, r2)) => (r1, r2),
            Note::ArrivalPlatform((r1, r2)) => (r1, r2),
            Note::ArrivalTrack((r1, r2)) => (r1, r2),
            Note::Class((r1, r2)) => (r1, r2),
            Note::Duration((r1, r2)) => (r1, r2),
            Note::Price((r1, r2)) => (r1, r2),
            Note::Route((r1, r2)) => (r1, r2),
            Note::Row((r1, r2)) => (r1, r2),
            Note::Seat((r1, r2)) => (r1, r2),
            Note::Train((r1, r2)) => (r1, r2),
            Note::Type((r1, r2)) => (r1, r2),
            Note::Wagon((r1, r2)) => (r1, r2),
            Note::Zone((r1, r2)) => (r1, r2),
        };

        r1.contains(field) || r2.contains(field)
    }
}

#[derive(Debug)]
struct Ticket {
    fields: Vec<u64>,
}

fn parse(input: &str) -> IResult<&str, (Vec<Note>, Ticket, Vec<Ticket>)> {
    tuple((
        notes,
        preceded(tag("\n\n"), your_ticket),
        preceded(tag("\n\n"), nearby_tickets),
    ))(input)
}

fn notes(input: &str) -> IResult<&str, Vec<Note>> {
    separated_list1(tag("\n"), note)(input)
}

fn your_ticket(input: &str) -> IResult<&str, Ticket> {
    preceded(tag("your ticket:\n"), ticket)(input)
}

fn nearby_tickets(input: &str) -> IResult<&str, Vec<Ticket>> {
    preceded(tag("nearby tickets:\n"), separated_list1(tag("\n"), ticket))(input)
}

fn ticket(input: &str) -> IResult<&str, Ticket> {
    map(nums_u64, |fields| Ticket { fields })(input)
}

fn note(input: &str) -> IResult<&str, Note> {
    alt((
        map(preceded(tag("departure location: "), ranges), |rs| {
            Note::DepartureLocation(rs)
        }),
        map(preceded(tag("departure station: "), ranges), |rs| {
            Note::DepartureStation(rs)
        }),
        map(preceded(tag("departure platform: "), ranges), |rs| {
            Note::DeparturePlatform(rs)
        }),
        map(preceded(tag("departure track: "), ranges), |rs| {
            Note::DepartureTrack(rs)
        }),
        map(preceded(tag("departure date: "), ranges), |rs| {
            Note::DepartureDate(rs)
        }),
        map(preceded(tag("departure time: "), ranges), |rs| {
            Note::DepartureTime(rs)
        }),
        map(preceded(tag("arrival location: "), ranges), |rs| {
            Note::ArrivalLocation(rs)
        }),
        map(preceded(tag("arrival station: "), ranges), |rs| {
            Note::ArrivalStation(rs)
        }),
        map(preceded(tag("arrival platform: "), ranges), |rs| {
            Note::ArrivalPlatform(rs)
        }),
        map(preceded(tag("arrival track: "), ranges), |rs| Note::ArrivalTrack(rs)),
        map(preceded(tag("class: "), ranges), |rs| Note::Class(rs)),
        map(preceded(tag("duration: "), ranges), |rs| Note::Duration(rs)),
        map(preceded(tag("price: "), ranges), |rs| Note::Price(rs)),
        map(preceded(tag("route: "), ranges), |rs| Note::Route(rs)),
        map(preceded(tag("row: "), ranges), |rs| Note::Row(rs)),
        map(preceded(tag("seat: "), ranges), |rs| Note::Seat(rs)),
        map(preceded(tag("train: "), ranges), |rs| Note::Train(rs)),
        map(preceded(tag("type: "), ranges), |rs| Note::Type(rs)),
        map(preceded(tag("wagon: "), ranges), |rs| Note::Wagon(rs)),
        map(preceded(tag("zone: "), ranges), |rs| Note::Zone(rs)),
    ))(input)
}

fn ranges(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    tuple((range, preceded(tag(" or "), range)))(input)
}

fn range(input: &str) -> IResult<&str, Range<u64>> {
    map(tuple((num_u64, preceded(tag("-"), num_u64))), |(r1, r2)| {
        Range {
            start: r1,
            end: r2 + 1,
        }
    })(input)
}

fn nums_u64(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(","), num_u64)(input)
}
fn num_u64(input: &str) -> IResult<&str, u64> {
    map(recognize(digit1), |s: &str| s.parse::<u64>().unwrap())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_16_a() {
        let input = "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

        assert_eq!(71, day_16_a(input));
    }
}
