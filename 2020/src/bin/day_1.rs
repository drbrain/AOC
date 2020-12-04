use anyhow::Result;

use aoc2020::read;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = read("./01.input")?;

    println!("part A: {}", day_1(&input, 2)?);
    println!("part B: {}", day_1(&input, 3)?);

    Ok(())
}

fn day_1(input: &str, entries: usize) -> Result<u32> {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let entries: Vec<u32> = numbers
        .into_iter()
        .combinations(entries)
        .find(|pair| pair.iter().sum::<u32>() == 2020)
        .unwrap();

    let answer = entries.iter().product();

    Ok(answer)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_1_a() {
        let input = String::from("1721\n979\n366\n299\n675\n1456\n");

        assert_eq!(514579, day_1(&input, 2).unwrap());
    }

    #[test]
    fn test_day_1_b() {
        let input = String::from("1721\n979\n366\n299\n675\n1456\n");

        assert_eq!(241861950, day_1(&input, 3).unwrap());
    }
}
