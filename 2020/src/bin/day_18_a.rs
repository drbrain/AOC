use anyhow::Result;

use aoc2020::read;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

fn main() -> Result<()> {
    let input = read("./18.input")?;

    println!("part A: {}", day_18_a(&input));
    //println!("part B: {}", day_18_b(&input));

    Ok(())
}

fn day_18_a(input: &str) -> u64 {
    let exprs = exprs(input).unwrap().1;

    exprs.iter().map(|expr| expr.eval()).sum()
}

#[derive(Clone, Debug)]
enum Expr {
    Apply(Apply),
    Num(u64),
    Paren(Box<Expr>),
}

impl Expr {
    fn eval(&self) -> u64 {
        match self {
            Expr::Apply(a) => a.eval(),
            Expr::Num(n) => *n,
            Expr::Paren(e) => e.eval(),
        }
    }
}

#[derive(Clone, Debug)]
struct Apply {
    lhs: Box<Expr>,
    op: Op,
    rhs: Box<Expr>,
}

impl Apply {
    fn eval(&self) -> u64 {
        let lhs = match &*self.lhs {
            Expr::Apply(_) => unreachable!("apply can't be left of another apply"),
            Expr::Num(n) => *n,
            Expr::Paren(e) => e.eval(),
        };

        match &*self.rhs {
            Expr::Apply(a) => {
                let evaled_lhs = Box::new(Expr::Num(self.op.apply(lhs, a.lhs.eval())));

                Apply {
                    lhs: evaled_lhs,
                    op: a.op.clone(),
                    rhs: a.rhs.clone(),
                }
                .eval()
            }
            Expr::Num(n) => self.op.apply(lhs, *n),
            Expr::Paren(e) => self.op.apply(lhs, e.eval()),
        }
    }
}

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mul,
}

impl Op {
    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Op::Add => a + b,
            Op::Mul => a * b,
        }
    }
}

fn exprs(input: &str) -> IResult<&str, Vec<Expr>> {
    separated_list1(tag("\n"), expr)(input)
}

fn expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(tuple((lhs, op, boxed_expr)), |(lhs, op, rhs)| {
            Expr::Apply(Apply { lhs, op, rhs })
        }),
        parens,
        number,
    ))(input)
}

fn lhs(input: &str) -> IResult<&str, Box<Expr>> {
    map(alt((number, parens)), Box::new)(input)
}

fn boxed_expr(input: &str) -> IResult<&str, Box<Expr>> {
    preceded(tag(" "), map(expr, Box::new))(input)
}

fn op(input: &str) -> IResult<&str, Op> {
    preceded(
        tag(" "),
        alt((map(tag("+"), |_| Op::Add), map(tag("*"), |_| Op::Mul))),
    )(input)
}

fn parens(input: &str) -> IResult<&str, Expr> {
    delimited(
        char('('),
        map(expr, |p| Expr::Paren(Box::new(p))),
        cut(char(')')),
    )(input)
}

fn number(input: &str) -> IResult<&str, Expr> {
    map(num_u64, Expr::Num)(input)
}

fn num_u64(input: &str) -> IResult<&str, u64> {
    map(recognize(digit1), |s: &str| s.parse::<u64>().unwrap())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_18_a() {
        assert_eq!(7, day_18_a("3 + 4"));
        assert_eq!(71, day_18_a("1 + 2 * 3 + 4 * 5 + 6"));
        assert_eq!(26, day_18_a("2 * 3 + (4 * 5)"));
        assert_eq!(437, day_18_a("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
        assert_eq!(12240, day_18_a("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
        assert_eq!(
            13632,
            day_18_a("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
        );

        assert_eq!(37, day_18_a("3 + 4\n5 * 6"));
    }
}
