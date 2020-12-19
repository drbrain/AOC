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
    let input = read("./19.input")?;

    println!("part A: {}", day_19_a(&input));
    //println!("part B: {}", day_19_b(&input));

    Ok(())
}

fn day_19_a(input: &str) -> usize {
    let (rules, messages) = parse(input).unwrap().1;

    messages.iter().filter(|m| match_message(m, &rules)).count()
}

#[derive(Debug)]
enum StackEntry<'a> {
    Index(usize),
    Unwind((usize, &'a str)),
}

fn match_message<'a>(mut message: &'a str, rules: &Vec<Rule>) -> bool {
    let mut rule_stack: Vec<StackEntry> = Vec::new();
    let mut ok = None;
    let mut curr = 0;

    rule_stack.push(StackEntry::Index(curr));

    while !message.is_empty() && !rule_stack.is_empty() {
        curr = match rule_stack.pop().unwrap() {
            StackEntry::Index(c) => c,
            StackEntry::Unwind((c, m)) => {
                if ok.unwrap_or(false) {
                } else {
                    ok = None;
                    message = m;

                    match &rules[c] {
                        Rule::Alt((_, b)) => {
                            for rule in b.iter().rev() {
                                rule_stack.push(StackEntry::Index(*rule));
                            }
                        }
                        _ => panic!("Only an alt can be in an unwind"),
                    }
                }
                continue;
            }
        };

        match &rules[curr] {
            Rule::Alt((a, _)) => {
                rule_stack.push(StackEntry::Unwind((curr, message.clone())));

                for rule in a.iter().rev() {
                    rule_stack.push(StackEntry::Index(*rule));
                }
            }
            Rule::Lit(c) => {
                if message[0..1] == c.to_string() {
                    ok = Some(true);
                    message = &message[1..];
                } else {
                    while let Some(StackEntry::Index(_)) = rule_stack.last() {
                        rule_stack.pop();
                    }

                    ok = Some(false);
                }
            }
            Rule::Seq(s) => {
                for rule in s.iter().rev() {
                    rule_stack.push(StackEntry::Index(*rule));
                }
            }
        }
    }

    ok.unwrap_or(false) && message.is_empty()
}

// recursive solution
// fn match_message(message: &str, rules: &Vec<Rule>) -> bool {
//     let (unmatched, did_match) = match_rule(message, rules, 0);
//
//     did_match && unmatched.is_empty()
// }
//
// fn match_rule<'a>(message: &'a str, rules: &Vec<Rule>, index: usize) -> (&'a str, bool) {
//     match &rules[index] {
//         Rule::Alt((a, b)) => match match_sequence(message, rules, a) {
//             (m, true) => (m, true),
//             (_, false) => match_sequence(message, rules, b),
//         },
//         Rule::Lit(c) => {
//             if message[0..1] == c.to_string() {
//                 (&message[1..], true)
//             } else {
//                 (message, false)
//             }
//         }
//         Rule::Seq(s) => match_sequence(message, rules, s),
//     }
// }
//
// fn match_sequence<'a>(
//     mut message: &'a str,
//     rules: &Vec<Rule>,
//     sequence: &Vec<usize>,
// ) -> (&'a str, bool) {
//     for index in sequence {
//         match match_rule(message, rules, *index) {
//             (m, true) => message = m,
//             (m, false) => {
//                 return (m, false);
//             }
//         }
//     }
//
//     (message, true)
// }

#[derive(Debug)]
enum Rule {
    Alt((Vec<usize>, Vec<usize>)),
    Lit(char),
    Seq(Vec<usize>),
}

fn parse(input: &str) -> IResult<&str, (Vec<Rule>, Vec<&str>)> {
    pair(rules, preceded(tag("\n\n"), messages))(input)
}

fn rules(input: &str) -> IResult<&str, Vec<Rule>> {
    separated_list1(tag("\n"), rule)(input)
}

fn rule(input: &str) -> IResult<&str, Rule> {
    preceded(rule_num, alt((rule_lit, rule_alt, rule_seq)))(input)
}

fn rule_num(input: &str) -> IResult<&str, usize> {
    terminated(num_usize, tag(": "))(input)
}

fn rule_alt(input: &str) -> IResult<&str, Rule> {
    map(pair(numbers, preceded(tag(" | "), numbers)), Rule::Alt)(input)
}

fn rule_lit(input: &str) -> IResult<&str, Rule> {
    delimited(char('"'), map(anychar, Rule::Lit), char('"'))(input)
}

fn rule_seq(input: &str) -> IResult<&str, Rule> {
    map(numbers, Rule::Seq)(input)
}

fn numbers(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(char(' '), num_usize)(input)
}

fn num_usize(input: &str) -> IResult<&str, usize> {
    map(recognize(digit1), |s: &str| s.parse::<usize>().unwrap())(input)
}

fn messages(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(char('\n'), alpha1)(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_19_a() {
        let input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

        assert_eq!(2, day_19_a(input));
    }

    #[test]
    fn test_day_19_match_message() {
        let rs = rules(
            "0: 1 1 2
1: \"a\"
2: \"b\"",
        )
        .unwrap()
        .1;

        assert_eq!(true, match_message("aab", &rs));
        assert_eq!(false, match_message("abb", &rs));

        let rs = rules(
            "0: 1 | 2
1: \"a\"
2: \"b\"",
        )
        .unwrap()
        .1;

        assert_eq!(true, match_message("a", &rs));
        assert_eq!(true, match_message("b", &rs));
        assert_eq!(false, match_message("ab", &rs));

        let rs = rules(
            "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"",
        )
        .unwrap()
        .1;

        assert_eq!(true, match_message("ababbb", &rs));
        assert_eq!(true, match_message("abbbab", &rs));
        assert_eq!(false, match_message("bababa", &rs));
        assert_eq!(false, match_message("aaabbb", &rs));
        assert_eq!(false, match_message("aaaabbb", &rs));
    }
}
