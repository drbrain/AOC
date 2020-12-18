use anyhow::Result;

use aoc2020::read;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::str::FromStr;

fn main() -> Result<()> {
    let input = read("./18.input")?;

    println!("part B: {}", day_18_b(&input));

    Ok(())
}

fn day_18_b(input: &str) -> u64 {
    let exprs = exprs(input).unwrap().1;

    exprs.iter().sum()
}

fn exprs(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag("\n"), expr)(input)
}

// adapted from https://github.com/Geal/nom/blob/master/tests/arithmetic.rs, used under MIT License
fn parens(i: &str) -> IResult<&str, u64> {
    delimited(space0, delimited(tag("("), expr, tag(")")), space0)(i)
}

fn factor(i: &str) -> IResult<&str, u64> {
    alt((
        map_res(delimited(space0, digit1, space0), FromStr::from_str),
        parens,
    ))(i)
}

// Read an initial factor and for each time we find a + operator followed by another factor, we do
// the math by folding everything
fn term(i: &str) -> IResult<&str, u64> {
    let (i, init) = factor(i)?;

    fold_many0(preceded(char('+'), factor), init, |acc, val| acc + val)(i)
}

// Again with *
fn expr(i: &str) -> IResult<&str, u64> {
    let (i, init) = term(i)?;

    fold_many0(preceded(char('*'), term), init, |acc, val| acc * val)(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_18_b_expr() {
        assert_eq!(7, day_18_b("3 + 4"));
        assert_eq!(231, day_18_b("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(46, day_18_b("2 * 3 + (4 * 5)"));
        assert_eq!(1445, day_18_b("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(
            669060,
            day_18_b("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
        );
        assert_eq!(
            23340,
            day_18_b("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );
    }
}
