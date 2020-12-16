use anyhow::Result;

use aoc2020::read;

use itertools::Itertools;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::collections::HashSet;
use std::fmt;
use std::ops::Range;

fn main() -> Result<()> {
    let input = read("./16.input")?;

    println!("part A: {}", day_16_a(&input));
    println!("part B: {}", day_16_b(&input));

    Ok(())
}

fn day_16_a(input: &str) -> u64 {
    let (notes, _, nearby) = parse(input).unwrap().1;

    let invalid = find_all_invalid(&nearby, &notes);

    invalid.iter().sum()
}

fn find_all_invalid(tickets: &[Ticket], notes: &[Note]) -> Vec<u64> {
    tickets
        .iter()
        .filter_map(|ticket| find_invalid(ticket, notes))
        .flatten()
        .collect()
}

fn find_invalid(ticket: &Ticket, notes: &[Note]) -> Option<Vec<u64>> {
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

fn validate(field: &u64, notes: &[Note]) -> bool {
    notes.iter().any(|n| n.validate(field))
}

fn day_16_b(input: &str) -> u64 {
    let (notes, ticket, nearby) = parse(input).unwrap().1;

    let valid = valid(&nearby, &notes);

    let positions = determine_positions(&valid, &notes);

    departure_positions(&ticket, &positions).iter().product()
}

fn determine_positions(tickets: &[Ticket], notes: &[Note]) -> Vec<(usize, Note)> {
    let mut possibilities: Vec<Vec<&Note>> = Vec::with_capacity(notes.len());

    for field in 0..tickets[0].fields.len() {
        let ticket_fields = nth_field(tickets, field);

        let matching: Vec<&Note> = notes
            .iter()
            .filter(|n| ticket_fields.iter().all(|f| n.validate(f)))
            .collect();

        possibilities.push(matching);
    }

    let possibilities: Vec<(usize, &Vec<&Note>)> = possibilities
        .iter()
        .enumerate()
        .sorted_by(|(_, a), (_, b)| Ord::cmp(&a.len(), &b.len()))
        .collect();

    let mut seen: HashSet<&Note> = HashSet::new();
    let mut positions: Vec<(usize, Note)> = Vec::with_capacity(notes.len());

    for (i, possibility) in possibilities {
        let remain: &Note = possibility
            .iter()
            .find(|p| !seen.contains(*p))
            .unwrap();

        seen.insert(remain);

        positions.push((i, remain.clone()));
    }

    positions
}

fn nth_field(tickets: &[Ticket], field: usize) -> Vec<u64> {
    tickets.iter().map(|t| t.fields[field]).collect()
}

fn departure_positions(ticket: &Ticket, positions: &[(usize, Note)]) -> Vec<u64> {
    positions
        .iter()
        .filter_map(|(i, note)| match note {
            Note::DepartureLocation(_) => Some(ticket.fields[*i]),
            Note::DepartureStation(_) => Some(ticket.fields[*i]),
            Note::DeparturePlatform(_) => Some(ticket.fields[*i]),
            Note::DepartureTrack(_) => Some(ticket.fields[*i]),
            Note::DepartureDate(_) => Some(ticket.fields[*i]),
            Note::DepartureTime(_) => Some(ticket.fields[*i]),
            _ => None,
        })
        .collect()
}

fn valid(tickets: &[Ticket], notes: &[Note]) -> Vec<Ticket> {
    tickets
        .iter()
        .filter_map(|ticket| find_valid(ticket, notes))
        .collect()
}

fn find_valid(ticket: &Ticket, notes: &[Note]) -> Option<Ticket> {
    match ticket.fields.iter().all(|field| validate(field, notes)) {
        true => Some(ticket.clone()),
        false => None,
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
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

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            Note::DepartureLocation(_) => "DLo",
            Note::DepartureStation(_) => "DSt",
            Note::DeparturePlatform(_) => "DPl",
            Note::DepartureTrack(_) => "DTr",
            Note::DepartureDate(_) => "DDa",
            Note::DepartureTime(_) => "DTi",
            Note::ArrivalLocation(_) => "ALo",
            Note::ArrivalStation(_) => "ASt",
            Note::ArrivalPlatform(_) => "APl",
            Note::ArrivalTrack(_) => "ATr",
            Note::Class(_) => "Cls",
            Note::Duration(_) => "Dur",
            Note::Price(_) => "Pri",
            Note::Route(_) => "Rou",
            Note::Row(_) => "Row",
            Note::Seat(_) => "Sea",
            Note::Train(_) => "Tra",
            Note::Type(_) => "Typ",
            Note::Wagon(_) => "Wag",
            Note::Zone(_) => "Zon",
        };

        write!(f, "{}", symbol)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
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
        map(
            preceded(tag("departure location: "), ranges),
            Note::DepartureLocation,
        ),
        map(
            preceded(tag("departure station: "), ranges),
            Note::DepartureStation,
        ),
        map(
            preceded(tag("departure platform: "), ranges),
            Note::DeparturePlatform,
        ),
        map(
            preceded(tag("departure track: "), ranges),
            Note::DepartureTrack,
        ),
        map(
            preceded(tag("departure date: "), ranges),
            Note::DepartureDate,
        ),
        map(
            preceded(tag("departure time: "), ranges),
            Note::DepartureTime,
        ),
        map(
            preceded(tag("arrival location: "), ranges),
            Note::ArrivalLocation,
        ),
        map(
            preceded(tag("arrival station: "), ranges),
            Note::ArrivalStation,
        ),
        map(
            preceded(tag("arrival platform: "), ranges),
            Note::ArrivalPlatform,
        ),
        map(preceded(tag("arrival track: "), ranges), Note::ArrivalTrack),
        map(preceded(tag("class: "), ranges), Note::Class),
        map(preceded(tag("duration: "), ranges), Note::Duration),
        map(preceded(tag("price: "), ranges), Note::Price),
        map(preceded(tag("route: "), ranges), Note::Route),
        map(preceded(tag("row: "), ranges), Note::Row),
        map(preceded(tag("seat: "), ranges), Note::Seat),
        map(preceded(tag("train: "), ranges), Note::Train),
        map(preceded(tag("type: "), ranges), Note::Type),
        map(preceded(tag("wagon: "), ranges), Note::Wagon),
        map(preceded(tag("zone: "), ranges), Note::Zone),
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

    #[test]
    fn test_day_16_valid() {
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

        let (notes, _, nearby) = parse(input).unwrap().1;

        assert_eq!(
            vec![Ticket {
                fields: vec![7, 3, 47]
            }],
            valid(&nearby, &notes)
        );
    }
}
