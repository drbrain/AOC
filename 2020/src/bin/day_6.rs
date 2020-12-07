use anyhow::Result;

use aoc2020::read;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = read("./06.input")?;

    println!("part A: {}", day_6_a(&input));
    println!("part B: {}", day_6_b(&input));

    Ok(())
}

fn day_6_a(answers: &str) -> usize {
    answers
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| char::is_alphabetic(*c))
                .unique()
                .count()
        })
        .sum()
}

fn day_6_b(answers: &str) -> usize {
    let groups: Vec<&str> = answers.split("\n\n").collect();

    groups
        .iter()
        .map(|group| {
            let size = group.lines().count();

            let groups = group
                .chars()
                .filter(|c| char::is_alphabetic(*c))
                .sorted()
                .group_by(|c| *c);

            let mut count = 0;

            for (_, group) in &groups {
                if group.count() == size {
                    count += 1;
                }
            }

            count
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_6_a() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(11, day_6_a(input));
    }

    #[test]
    fn test_day_6_b() {
        let input = "abc

a
b
c

ab
ac

a
a
a
a

b";

        assert_eq!(6, day_6_b(input));
    }
}
