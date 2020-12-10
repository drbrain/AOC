use anyhow::anyhow;
use anyhow::Result;

use aoc2020::read;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = read("./10.input")?;

    println!("part A: {}", day_10_a(&input));
    //println!("part B: {}", day_10_b(&input));

    Ok(())
}

fn day_10_a(input: &str) -> u64 {
    let adapters = numbers(input);

    let (ones, threes) = joltage_differences(&adapters).unwrap();

    ones * threes
}

fn joltage_differences(adapters: &[u64]) -> Result<(u64, u64)> {
    let mut ones = 0;
    let mut threes = 1;
    let mut joltage = 0;

    for adapter in adapters {
        if (1..4).contains(&(adapter - joltage)) {
            match adapter - joltage {
                1 => {
                    ones += 1;
                }
                3 => {
                    threes += 1;
                }
                _ => {
                    anyhow!("adapter {} out of range for joltage {}", adapter, joltage);
                }
            }

            joltage = *adapter;
        } else {
            continue;
        }
    }

    Ok((ones, threes))
}

fn numbers(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|n: &str| n.parse::<u64>().unwrap())
        .sorted()
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_10() {
        let input = "16
10
15
5
1
11
7
19
6
12
4";

        assert_eq!(35, day_10_a(input));

        let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

        assert_eq!(220, day_10_a(input));
    }

    #[test]
    fn test_day_10_joltage_differences() {
        let adapters = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];

        assert_eq!((7, 5), joltage_differences(&adapters).unwrap());

        let adapters = vec![
            1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35,
            38, 39, 42, 45, 46, 47, 48, 49,
        ];

        assert_eq!((22, 10), joltage_differences(&adapters).unwrap());
    }
}
