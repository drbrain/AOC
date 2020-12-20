use anyhow::Result;

use aoc2020::read;

use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use earlgrey::EarleyParser;
use earlgrey::GrammarBuilder;

fn main() -> Result<()> {
    let input = read("./19.input")?;

    println!("part A: {}", day_19_a(&input));
    println!("part B: {}", day_19_b(&input));

    Ok(())
}

fn day_19_a(input: &str) -> usize {
    let (rules, messages) = parse(input).unwrap().1;

    messages.iter().filter(|m| match_message(m, &rules)).count()
}

fn day_19_b(input: &str) -> usize {
    let messages = parse(input).unwrap().1 .1;

    let grammar = GrammarBuilder::default()
        .terminal("20", |c| "a" == c)
        .terminal("91", |c| "b" == c)
        .nonterm("0")
        .nonterm("1")
        .nonterm("2")
        .nonterm("3")
        .nonterm("4")
        .nonterm("5")
        .nonterm("6")
        .nonterm("7")
        .nonterm("8")
        .nonterm("9")
        .nonterm("10")
        .nonterm("11")
        .nonterm("12")
        .nonterm("13")
        .nonterm("14")
        .nonterm("15")
        .nonterm("16")
        .nonterm("17")
        .nonterm("18")
        .nonterm("19")
        .nonterm("21")
        .nonterm("22")
        .nonterm("23")
        .nonterm("24")
        .nonterm("25")
        .nonterm("26")
        .nonterm("27")
        .nonterm("28")
        .nonterm("29")
        .nonterm("30")
        .nonterm("31")
        .nonterm("32")
        .nonterm("33")
        .nonterm("34")
        .nonterm("35")
        .nonterm("36")
        .nonterm("37")
        .nonterm("38")
        .nonterm("39")
        .nonterm("40")
        .nonterm("41")
        .nonterm("42")
        .nonterm("43")
        .nonterm("44")
        .nonterm("45")
        .nonterm("46")
        .nonterm("47")
        .nonterm("48")
        .nonterm("49")
        .nonterm("50")
        .nonterm("51")
        .nonterm("52")
        .nonterm("53")
        .nonterm("54")
        .nonterm("55")
        .nonterm("56")
        .nonterm("57")
        .nonterm("58")
        .nonterm("59")
        .nonterm("60")
        .nonterm("61")
        .nonterm("62")
        .nonterm("63")
        .nonterm("64")
        .nonterm("65")
        .nonterm("66")
        .nonterm("67")
        .nonterm("68")
        .nonterm("69")
        .nonterm("70")
        .nonterm("71")
        .nonterm("72")
        .nonterm("73")
        .nonterm("74")
        .nonterm("75")
        .nonterm("76")
        .nonterm("77")
        .nonterm("78")
        .nonterm("79")
        .nonterm("80")
        .nonterm("81")
        .nonterm("82")
        .nonterm("83")
        .nonterm("84")
        .nonterm("85")
        .nonterm("86")
        .nonterm("87")
        .nonterm("88")
        .nonterm("89")
        .nonterm("90")
        .nonterm("92")
        .nonterm("93")
        .nonterm("94")
        .nonterm("95")
        .nonterm("96")
        .nonterm("97")
        .nonterm("98")
        .nonterm("99")
        .nonterm("100")
        .nonterm("101")
        .nonterm("102")
        .nonterm("103")
        .nonterm("104")
        .nonterm("105")
        .nonterm("106")
        .nonterm("107")
        .nonterm("108")
        .nonterm("109")
        .nonterm("110")
        .nonterm("111")
        .nonterm("112")
        .nonterm("113")
        .nonterm("114")
        .nonterm("115")
        .nonterm("116")
        .nonterm("117")
        .nonterm("118")
        .nonterm("119")
        .nonterm("120")
        .nonterm("121")
        .nonterm("122")
        .nonterm("123")
        .nonterm("124")
        .nonterm("125")
        .nonterm("126")
        .nonterm("127")
        .nonterm("128")
        .nonterm("129")
        .nonterm("130")
        .nonterm("131")
        .rule("0", &["8", "11"])
        .rule("1", &["91", "20"])
        .rule("2", &["106", "60"])
        .rule("3", &["91", "92"])
        .rule("3", &["20", "63"])
        .rule("4", &["91", "50"])
        .rule("4", &["20", "115"])
        .rule("5", &["91", "91"])
        .rule("6", &["117", "91"])
        .rule("6", &["58", "20"])
        .rule("7", &["20", "91"])
        .rule("7", &["91", "20"])
        .rule("8", &["42"])
        .rule("8", &["42", "8"])
        .rule("9", &["37", "91"])
        .rule("9", &["53", "20"])
        .rule("10", &["91", "44"])
        .rule("10", &["20", "84"])
        .rule("11", &["42", "31"])
        .rule("11", &["42", "11", "31"])
        .rule("12", &["91", "62"])
        .rule("12", &["20", "129"])
        .rule("13", &["109", "91"])
        .rule("13", &["129", "20"])
        .rule("14", &["20", "1"])
        .rule("14", &["91", "62"])
        .rule("15", &["120", "91"])
        .rule("15", &["14", "20"])
        .rule("16", &["103", "91"])
        .rule("16", &["22", "20"])
        .rule("17", &["20", "1"])
        .rule("17", &["91", "89"])
        .rule("18", &["84", "91"])
        .rule("18", &["89", "20"])
        .rule("19", &["5", "20"])
        .rule("19", &["109", "91"])
        .rule("21", &["89", "20"])
        .rule("21", &["62", "91"])
        .rule("22", &["91", "55"])
        .rule("22", &["20", "88"])
        .rule("23", &["91", "91"])
        .rule("23", &["91", "20"])
        .rule("24", &["20", "83"])
        .rule("24", &["91", "64"])
        .rule("25", &["91", "54"])
        .rule("25", &["20", "49"])
        .rule("26", &["109", "91"])
        .rule("26", &["89", "20"])
        .rule("27", &["91", "129"])
        .rule("27", &["20", "60"])
        .rule("28", &["106", "129"])
        .rule("29", &["82", "91"])
        .rule("29", &["30", "20"])
        .rule("30", &["20", "5"])
        .rule("30", &["91", "40"])
        .rule("31", &["91", "69"])
        .rule("31", &["20", "47"])
        .rule("32", &["91", "59"])
        .rule("32", &["20", "119"])
        .rule("33", &["100", "20"])
        .rule("33", &["122", "91"])
        .rule("34", &["91", "17"])
        .rule("35", &["20", "13"])
        .rule("35", &["7", "91"])
        .rule("35", &["23", "20"])
        .rule("36", &["20", "102"])
        .rule("36", &["91", "23"])
        .rule("37", &["20", "113"])
        .rule("37", &["91", "67"])
        .rule("38", &["27", "91"])
        .rule("38", &["36", "20"])
        .rule("39", &["114", "20"])
        .rule("39", &["102", "91"])
        .rule("40", &["20", "91"])
        .rule("41", &["23", "91"])
        .rule("42", &["20", "86"])
        .rule("42", &["91", "56"])
        .rule("43", &["91", "12"])
        .rule("43", &["20", "65"])
        .rule("44", &["91", "106"])
        .rule("44", &["20", "20"])
        .rule("45", &["7", "91"])
        .rule("45", &["44", "20"])
        .rule("46", &["5", "91"])
        .rule("46", &["5", "20"])
        .rule("47", &["20", "3"])
        .rule("47", &["91", "57"])
        .rule("48", &["91", "5"])
        .rule("48", &["20", "5"])
        .rule("49", &["7", "20"])
        .rule("49", &["5", "91"])
        .rule("50", &["91", "107"])
        .rule("50", &["20", "2"])
        .rule("51", &["62", "20"])
        .rule("51", &["114", "91"])
        .rule("52", &["129", "20"])
        .rule("52", &["102", "91"])
        .rule("53", &["105", "20"])
        .rule("53", &["68", "91"])
        .rule("54", &["89", "91"])
        .rule("54", &["114", "20"])
        .rule("55", &["35", "20"])
        .rule("55", &["72", "91"])
        .rule("56", &["91", "121"])
        .rule("56", &["20", "61"])
        .rule("57", &["112", "91"])
        .rule("57", &["94", "20"])
        .rule("58", &["89", "20"])
        .rule("58", &["1", "91"])
        .rule("59", &["91", "83"])
        .rule("59", &["20", "46"])
        .rule("60", &["91", "20"])
        .rule("60", &["20", "106"])
        .rule("61", &["33", "91"])
        .rule("61", &["131", "20"])
        .rule("62", &["106", "106"])
        .rule("63", &["6", "20"])
        .rule("63", &["75", "91"])
        .rule("64", &["91", "1"])
        .rule("64", &["20", "5"])
        .rule("65", &["20", "109"])
        .rule("66", &["91", "1"])
        .rule("67", &["93", "91"])
        .rule("67", &["65", "20"])
        .rule("68", &["18", "91"])
        .rule("68", &["99", "20"])
        .rule("69", &["118", "91"])
        .rule("69", &["16", "20"])
        .rule("70", &["91", "24"])
        .rule("70", &["20", "34"])
        .rule("71", &["110", "91"])
        .rule("71", &["128", "20"])
        .rule("72", &["20", "23"])
        .rule("72", &["91", "44"])
        .rule("73", &["44", "106"])
        .rule("74", &["91", "78"])
        .rule("74", &["20", "48"])
        .rule("75", &["17", "106"])
        .rule("76", &["20", "109"])
        .rule("76", &["91", "23"])
        .rule("77", &["127", "91"])
        .rule("77", &["90", "20"])
        .rule("78", &["20", "5"])
        .rule("79", &["5", "20"])
        .rule("79", &["1", "91"])
        .rule("80", &["91", "39"])
        .rule("80", &["20", "76"])
        .rule("81", &["7", "91"])
        .rule("81", &["60", "20"])
        .rule("82", &["106", "102"])
        .rule("83", &["1", "20"])
        .rule("83", &["60", "91"])
        .rule("84", &["20", "20"])
        .rule("84", &["91", "20"])
        .rule("85", &["91", "1"])
        .rule("85", &["20", "44"])
        .rule("86", &["104", "20"])
        .rule("86", &["9", "91"])
        .rule("87", &["73", "91"])
        .rule("87", &["13", "20"])
        .rule("88", &["20", "21"])
        .rule("88", &["91", "26"])
        .rule("89", &["20", "20"])
        .rule("90", &["20", "129"])
        .rule("90", &["91", "23"])
        .rule("92", &["126", "91"])
        .rule("92", &["95", "20"])
        .rule("93", &["60", "91"])
        .rule("93", &["114", "20"])
        .rule("94", &["77", "91"])
        .rule("94", &["124", "20"])
        .rule("95", &["79", "91"])
        .rule("95", &["85", "20"])
        .rule("96", &["20", "48"])
        .rule("96", &["91", "52"])
        .rule("97", &["91", "7"])
        .rule("97", &["20", "102"])
        .rule("98", &["20", "81"])
        .rule("98", &["91", "45"])
        .rule("99", &["60", "91"])
        .rule("99", &["89", "20"])
        .rule("100", &["91", "111"])
        .rule("100", &["20", "2"])
        .rule("101", &["91", "25"])
        .rule("101", &["20", "96"])
        .rule("102", &["20", "20"])
        .rule("102", &["91", "91"])
        .rule("103", &["91", "80"])
        .rule("103", &["20", "29"])
        .rule("104", &["91", "70"])
        .rule("104", &["20", "108"])
        .rule("105", &["97", "20"])
        .rule("105", &["83", "91"])
        .rule("106", &["91"])
        .rule("106", &["20"])
        .rule("107", &["62", "20"])
        .rule("107", &["44", "91"])
        .rule("108", &["20", "87"])
        .rule("108", &["91", "71"])
        .rule("109", &["20", "20"])
        .rule("109", &["106", "91"])
        .rule("110", &["109", "91"])
        .rule("110", &["60", "20"])
        .rule("111", &["7", "91"])
        .rule("111", &["89", "20"])
        .rule("112", &["15", "91"])
        .rule("112", &["43", "20"])
        .rule("113", &["82", "20"])
        .rule("113", &["130", "91"])
        .rule("114", &["20", "91"])
        .rule("114", &["20", "20"])
        .rule("115", &["51", "91"])
        .rule("115", &["85", "20"])
        .rule("116", &["38", "20"])
        .rule("116", &["74", "91"])
        .rule("117", &["20", "23"])
        .rule("118", &["4", "91"])
        .rule("118", &["101", "20"])
        .rule("119", &["20", "123"])
        .rule("119", &["91", "65"])
        .rule("120", &["23", "20"])
        .rule("120", &["5", "91"])
        .rule("121", &["32", "20"])
        .rule("121", &["116", "91"])
        .rule("122", &["128", "20"])
        .rule("122", &["41", "91"])
        .rule("123", &["20", "84"])
        .rule("123", &["91", "60"])
        .rule("124", &["10", "20"])
        .rule("124", &["35", "91"])
        .rule("125", &["28", "20"])
        .rule("125", &["66", "91"])
        .rule("126", &["20", "123"])
        .rule("126", &["91", "19"])
        .rule("127", &["129", "91"])
        .rule("127", &["40", "20"])
        .rule("128", &["1", "20"])
        .rule("128", &["44", "91"])
        .rule("129", &["91", "20"])
        .rule("129", &["106", "91"])
        .rule("130", &["60", "106"])
        .rule("131", &["125", "20"])
        .rule("131", &["98", "91"])
        .into_grammar("0")
        .unwrap();

    //rules[8] = Rule::Alt((vec![42], vec![42, 8]));
    //rules[11] = Rule::Alt((vec![42, 31], vec![42, 11, 31]));

    messages
        .iter()
        .filter(|m| {
            let message = m.chars().into_iter().map(|c| c.to_string());

            let result = EarleyParser::new(grammar.clone()).parse(message);

            //dbg!(&result);

            match result {
                Ok(_) => true,
                Err(_) => false,
            }
        })
        .count()
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
                //eprintln!(
                //    "message: {:>15} ok {:?} stack: {:?}",
                //    message, ok, rule_stack
                //);
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

        //eprintln!(
        //    "message: {:>15} ok {:?} stack: {:?}",
        //    message, ok, rule_stack
        //);
    }

    //eprintln!(
    //    "ok: {:?}, message: {:?}, stack: {:?}",
    //    ok, message, rule_stack
    //);

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

// fn early_grammar(rules: &'static Vec<Rule>) -> Grammar {
//     let mut builder = GrammarBuilder::default();
//
//     for (i, rule) in rules.iter().enumerate() {
//         let i = &i.to_string()[..];
//
//         match rule {
//             Rule::Alt((a, b)) => {
//                 builder = builder.rule(i.clone(), &vec_to_string(a));
//                 builder = builder.rule(i, &vec_to_string(b));
//             }
//             Rule::Lit(t) => {
//                 builder = builder.terminal(t.clone(), move |c| c == t);
//             }
//             Rule::Seq(s) => {
//                 builder = builder.rule(i, &vec_to_string(s));
//             }
//         }
//     }
//
//     builder.into_grammar("0").unwrap()
// }
//
// fn vec_to_string(vec: &Vec<usize>) -> Vec<String> {
//     vec.iter().map(|v| v.to_string()).collect()
// }

#[derive(Debug)]
enum Rule {
    Alt((Vec<usize>, Vec<usize>)),
    Lit(String),
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
    delimited(
        char('"'),
        map(anychar, |c| Rule::Lit(c.to_string())),
        char('"'),
    )(input)
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
    fn test_day_19_grammar() {
        let grammar = GrammarBuilder::default()
            .nonterm("0")
            .nonterm("2")
            .nonterm("3")
            .nonterm("4")
            .nonterm("5")
            .nonterm("6")
            .nonterm("7")
            .nonterm("8")
            .nonterm("9")
            .nonterm("10")
            .nonterm("11")
            .nonterm("12")
            .nonterm("13")
            .nonterm("15")
            .nonterm("16")
            .nonterm("17")
            .nonterm("18")
            .nonterm("19")
            .nonterm("20")
            .nonterm("21")
            .nonterm("22")
            .nonterm("23")
            .nonterm("24")
            .nonterm("25")
            .nonterm("26")
            .nonterm("27")
            .nonterm("28")
            .nonterm("31")
            .nonterm("42")
            .terminal("1", |c| c == "a")
            .terminal("14", |c| c == "b")
            .rule("0", &["8", "11"])
            .rule("2", &["1", "24"])
            .rule("2", &["14", "4"])
            .rule("3", &["5", "14"])
            .rule("3", &["16", "1"])
            .rule("4", &["1", "1"])
            .rule("5", &["1", "14"])
            .rule("5", &["15", "1"])
            .rule("6", &["14", "14"])
            .rule("6", &["1", "14"])
            .rule("7", &["14", "5"])
            .rule("7", &["1", "21"])
            .rule("8", &["42"])
            .rule("8", &["42", "8"])
            .rule("9", &["14", "27"])
            .rule("9", &["1", "26"])
            .rule("10", &["23", "14"])
            .rule("10", &["28", "1"])
            .rule("11", &["42", "31"])
            .rule("11", &["42", "11", "31"])
            .rule("12", &["24", "14"])
            .rule("12", &["19", "1"])
            .rule("13", &["14", "3"])
            .rule("13", &["1", "12"])
            .rule("15", &["1"])
            .rule("15", &["14"])
            .rule("16", &["15", "1"])
            .rule("16", &["14", "14"])
            .rule("17", &["14", "2"])
            .rule("17", &["1", "7"])
            .rule("18", &["15", "15"])
            .rule("19", &["14", "1"])
            .rule("19", &["14", "14"])
            .rule("20", &["14", "14"])
            .rule("20", &["1", "15"])
            .rule("21", &["14", "1"])
            .rule("21", &["1", "14"])
            .rule("22", &["14", "14"])
            .rule("23", &["25", "1"])
            .rule("23", &["22", "14"])
            .rule("24", &["14", "1"])
            .rule("25", &["1", "1"])
            .rule("25", &["1", "14"])
            .rule("26", &["14", "22"])
            .rule("26", &["1", "20"])
            .rule("27", &["1", "6"])
            .rule("27", &["14", "18"])
            .rule("28", &["16", "1"])
            .rule("31", &["14", "17"])
            .rule("31", &["1", "13"])
            .rule("42", &["9", "14"])
            .rule("42", &["10", "1"])
            .into_grammar("0")
            .unwrap();

        let messages = vec![
            "aaaabbaaaabbaaa",
            "aaaaabbaabaaaaababaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "baabbaaaabbaaaababbaababb",
            "babaaabbbaaabaababbaabababaaab",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "bbabbbbaabaabba",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "bbbbbbbaaaabbbbaaabbabaaa",
        ];

        let count = messages
            .iter()
            .filter(|m| {
                let message = m.chars().into_iter().map(|c| c.to_string());

                let result = EarleyParser::new(grammar.clone()).parse(message);

                //dbg!(&result);

                match result {
                    Ok(_) => true,
                    Err(_) => false,
                }
            })
            .count();

        //assert_eq!(3, day_19_a(input));
        assert_eq!(12, count);
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
