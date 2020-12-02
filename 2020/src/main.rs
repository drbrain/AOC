use anyhow::Result;

use itertools::Itertools;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() -> Result<()> {
    let input = read("./01.input")?;

    println!("part A: {}", day_1(&input, 2)?);
    println!("part B: {}", day_1(&input, 3)?);

    Ok(())
}

fn day_1(input: &String, entries: usize) -> Result<u32> {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let entries: Vec<u32> = numbers
        .into_iter()
        .combinations(entries)
        .find(|pair| pair.iter().fold(0, |a, b| a + b) == 2020)
        .unwrap();

    let answer = entries.iter().fold(1, |a, b| a * b);

    Ok(answer)
}

fn read<P>(filename: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut result = String::new();
    let mut file = File::open(filename)?;

    file.read_to_string(&mut result)?;

    Ok(result)
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
