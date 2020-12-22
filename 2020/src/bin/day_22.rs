use anyhow::Result;

use aoc2020::read;

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::cmp::Ordering;

fn main() -> Result<()> {
    let input = read("./22.input")?;

    println!("part A: {}", day_22_a(&input));
    //println!("part B: {}", day_22_b(&input));

    Ok(())
}

fn day_22_a(input: &str) -> u64 {
    let mut decks = parse(input).unwrap().1;

    let winner = play(&mut decks);

    winner.score()
}

fn play(decks: &mut Vec<Deck>) -> Deck {
    let mut deck_1 = decks.remove(0);
    let mut deck_2 = decks.remove(0);

    while !deck_1.is_empty() && !deck_2.is_empty() {
        let card_1 = deck_1.draw();
        let card_2 = deck_2.draw();

        match card_1.cmp(&card_2) {
            Ordering::Less => deck_2.add(card_2, card_1),
            Ordering::Greater => deck_1.add(card_1, card_2),
            Ordering::Equal => panic!("cards must not be equal"),
        }
    }

    if deck_1.is_empty() {
        deck_2
    } else {
        deck_1
    }
}

#[derive(Debug)]
struct Deck {
    cards: Vec<u64>,
}

impl Deck {
    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn draw(&mut self) -> u64 {
        self.cards.remove(0)
    }

    fn add(&mut self, high: u64, low: u64) {
        self.cards.push(high);
        self.cards.push(low);
    }

    fn score(&self) -> u64 {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .fold(0, |a, (i, v)| a + (i as u64 + 1) * v)
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Deck>> {
    separated_list1(tag("\n\n"), deck)(input)
}

fn deck(input: &str) -> IResult<&str, Deck> {
    preceded(player, cards)(input)
}

fn player(input: &str) -> IResult<&str, u64> {
    delimited(tag("Player "), num_u64, tag(":\n"))(input)
}

fn cards(input: &str) -> IResult<&str, Deck> {
    map(separated_list1(tag("\n"), num_u64), |cards| Deck { cards })(input)
}

fn num_u64(input: &str) -> IResult<&str, u64> {
    map(recognize(digit1), |s: &str| s.parse::<u64>().unwrap())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_22_a() {
        let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

        assert_eq!(306, day_22_a(input));
    }
}
