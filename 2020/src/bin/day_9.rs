use anyhow::Result;

use aoc2020::read;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = read("./09.input")?;

    println!("part A: {}", day_9_a(&input));

    Ok(())
}

fn day_9_a(input: &str) -> u64 {
    let numbers = numbers(input);

    incorrect(25, numbers).unwrap()
}

fn incorrect(range_max: usize, numbers: Vec<u64>) -> Option<u64> {
    for max in range_max..numbers.len() {
        if valid(range_max, numbers[0..max + 1].to_vec()) {
            continue;
        }

        return Some(numbers[max]);
    }

    None
}

fn valid(range_max: usize, numbers: Vec<u64>) -> bool {
    let target = *numbers.last().unwrap();

    let max = range_max.max(numbers.len()) - 1;
    let min = max - range_max;

    numbers[min..max]
        .iter()
        .combinations(2)
        .any(|pair| pair.iter().map(|n| *n).sum::<u64>() == target)
}

fn numbers(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|n: &str| n.parse::<u64>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_9_incorrect() {
        let numbers: Vec<u64> = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        assert_eq!(Some(127), incorrect(5, numbers));
    }

    #[test]
    fn test_day_9_valid() {
        let numbers: Vec<u64> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26,
        ];

        assert_eq!(true, valid(25, numbers));

        let numbers: Vec<u64> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 49,
        ];

        assert_eq!(true, valid(25, numbers));

        let numbers: Vec<u64> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 100,
        ];

        assert_eq!(false, valid(25, numbers));

        let numbers: Vec<u64> = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 50,
        ];

        assert_eq!(false, valid(25, numbers));

        let numbers: Vec<u64> = vec![
            20, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1, 21, 22, 23, 24,
            25, 45, 26,
        ];

        assert_eq!(true, valid(25, numbers));

        let numbers: Vec<u64> = vec![
            20, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1, 21, 22, 23, 24,
            25, 45, 64,
        ];

        assert_eq!(true, valid(25, numbers));

        let numbers: Vec<u64> = vec![
            20, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1, 21, 22, 23, 24,
            25, 45, 65,
        ];

        assert_eq!(false, valid(25, numbers));

        let numbers: Vec<u64> = vec![
            20, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 1, 21, 22, 23, 24,
            25, 45, 66,
        ];

        assert_eq!(true, valid(25, numbers));
    }
}
