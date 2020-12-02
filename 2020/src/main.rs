use anyhow::Result;

use itertools::Itertools;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

fn main() -> Result<()> {
    let input = read("./01.input")?;

    let answer = day_1_a(input)?;

    println!("answer: {}", answer);

    Ok(())
}

fn day_1_a(input: String) -> Result<u32> {
    let numbers: Vec<u32> = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let entries: Vec<u32> = numbers
        .into_iter()
        .combinations(2)
        .find(|pair| pair[0] + pair[1] == 2020)
        .unwrap();

    Ok(entries[0] * entries[1])
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
    fn test_day_1_a_example() {
        let input = String::from("1721\n979\n366\n299\n675\n1456\n");

        assert_eq!(514579, day_1_a(input).unwrap());
    }
}
