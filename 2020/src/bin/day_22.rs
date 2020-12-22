use anyhow::Result;

use aoc2020::read;

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::cmp::Ordering;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = read("./22.input")?;

    println!("part A: {}", day_22_a(&input));
    println!("part B: {}", day_22_b(&input));

    Ok(())
}

fn day_22_a(input: &str) -> usize {
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

fn day_22_b(input: &str) -> usize {
    let mut decks = parse(input).unwrap().1;

    let deck_1 = decks.remove(0);
    let deck_2 = decks.remove(0);

    let mut previous: HashSet<Round> = HashSet::new();

    let winner = play_recursive(deck_1, deck_2, &mut previous);

    winner.score()
}

fn play_recursive(mut deck_1: Deck, mut deck_2: Deck, previous: &mut HashSet<Round>) -> Deck {
    while !deck_1.is_empty() && !deck_2.is_empty() {
        let round = Round::new(&deck_1, &deck_2);

        if previous.contains(&round) {
            return deck_1;
        }

        let card_1 = deck_1.draw();
        let card_2 = deck_2.draw();

        if deck_1.len() >= card_1 && deck_2.len() >= card_2 {
            let sd_1 = deck_1.sub_deck(card_1);
            let sd_2 = deck_2.sub_deck(card_2);

            let mut sub_previous: HashSet<Round> = HashSet::new();

            match play_recursive(sd_1, sd_2, &mut sub_previous).player {
                1 => deck_1.add(card_1, card_2),
                2 => deck_2.add(card_2, card_1),
                player => unreachable!("Where did player {} come from", player),
            }
        } else {
            match card_1.cmp(&card_2) {
                Ordering::Less => deck_2.add(card_2, card_1),
                Ordering::Greater => deck_1.add(card_1, card_2),
                Ordering::Equal => panic!("cards must not be equal"),
            }
        }

        previous.insert(round);
    }

    if deck_1.is_empty() {
        deck_2
    } else {
        deck_1
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Round {
    deck_1: Deck,
    deck_2: Deck,
}

impl Round {
    fn new(deck_1: &Deck, deck_2: &Deck) -> Round {
        let deck_1 = deck_1.clone();
        let deck_2 = deck_2.clone();

        Round { deck_1, deck_2 }
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Deck {
    player: usize,
    cards: Vec<usize>,
}

impl Deck {
    fn add(&mut self, high: usize, low: usize) {
        self.cards.push(high);
        self.cards.push(low);
    }

    fn draw(&mut self) -> usize {
        self.cards.remove(0)
    }

    fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    fn len(&self) -> usize {
        self.cards.len()
    }

    fn score(&self) -> usize {
        self.cards
            .iter()
            .rev()
            .enumerate()
            .fold(0, |a, (i, v)| a + (i + 1) * v)
    }

    fn sub_deck(&self, count: usize) -> Deck {
        let cards = self.cards[0..count].to_vec();

        Deck {
            player: self.player,
            cards,
        }
    }
}

fn parse(input: &str) -> IResult<&str, Vec<Deck>> {
    separated_list1(tag("\n\n"), deck)(input)
}

fn deck(input: &str) -> IResult<&str, Deck> {
    map(pair(player, cards), |(player, cards)| Deck {
        player,
        cards,
    })(input)
}

fn player(input: &str) -> IResult<&str, usize> {
    delimited(tag("Player "), num_usize, tag(":\n"))(input)
}

fn cards(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag("\n"), num_usize)(input)
}

fn num_usize(input: &str) -> IResult<&str, usize> {
    map(recognize(digit1), |s: &str| s.parse::<usize>().unwrap())(input)
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

    #[test]
    fn test_day_22_b() {
        let input = "Player 1:
43
19

Player 2:
2
29
14";

        assert_eq!(105, day_22_b(input));

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

        assert_eq!(291, day_22_b(input));
    }
}
