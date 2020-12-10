use anyhow::anyhow;
use anyhow::Result;

use aoc2020::read;

use itertools::Itertools;

fn main() -> Result<()> {
    let input = read("./10.input")?;

    println!("part A: {}", day_10_a(&input));
    println!("part B: {}", day_10_b(&input));

    Ok(())
}

fn day_10_a(input: &str) -> u64 {
    let adapters = numbers(input);

    let (ones, threes, _adapted) = joltage_differences(&adapters).unwrap();

    ones * threes
}
fn day_10_b(input: &str) -> usize {
    let adapters = numbers(input);

    let sequence = joltage_sequence(&adapters);

    valid_sequences(&sequence, 0)
}

fn joltage_differences(adapters: &[u64]) -> Result<(u64, u64, u64)> {
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

    Ok((ones, threes, joltage + 3))
}

// (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
// (0), 1, 4, 5, 6, 7, 10,     12, 15, 16, 19, (22)
// (0), 1, 4, 5,    7, 10, 11, 12, 15, 16, 19, (22)
// (0), 1, 4, 5,    7, 10,     12, 15, 16, 19, (22)
// (0), 1, 4,    6, 7, 10, 11, 12, 15, 16, 19, (22)
// (0), 1, 4,    6, 7, 10,     12, 15, 16, 19, (22)
// (0), 1, 4,       7, 10, 11, 12, 15, 16, 19, (22)
// (0), 1, 4,       7, 10,     12, 15, 16, 19, (22)
//
// (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)
// (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 47,     49, (52)
// (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46,     48, 49, (52)
// (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46,         49, (52)
// (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45,     47, 48, 49, (52)
// (0),       3, 4, 7,       10, 11, 14, 17,         20, 23,     25, 28, 31,         34, 35, 38, 39, 42, 45, 46,     48, 49, (52)
// (0),       3, 4, 7,       10, 11, 14, 17,         20, 23,     25, 28, 31,         34, 35, 38, 39, 42, 45, 46,         49, (52)
// (0),       3, 4, 7,       10, 11, 14, 17,         20, 23,     25, 28, 31,         34, 35, 38, 39, 42, 45,     47, 48, 49, (52)
// (0),       3, 4, 7,       10, 11, 14, 17,         20, 23,     25, 28, 31,         34, 35, 38, 39, 42, 45,     47,     49, (52)
// (0),       3, 4, 7,       10, 11, 14, 17,         20, 23,     25, 28, 31,         34, 35, 38, 39, 42, 45,         48, 49, (52)
fn valid_sequences(sequence: &Vec<u64>, first: usize) -> usize {
    let len = sequence.len();
    let mut valid = 1;

    for i in first..len - 1 {
        let a = sequence[i];

        if i + 2 < len && sequence[i + 2] - a == 2 {
            let mut alternate = sequence.clone();
            alternate.remove(i + 1);

            valid += valid_sequences(&alternate, i);
        }

        if i + 3 < len && sequence[i + 3] - a == 3 {
            let mut alternate = sequence.clone();

            alternate.remove(i + 1);
            alternate.remove(i + 1);

            valid += valid_sequences(&alternate, i);
        }
    }

    valid
}

fn joltage_sequence(adapters: &[u64]) -> Vec<u64> {
    let (_, _, target_joltage) = joltage_differences(&adapters).unwrap();
    let target_joltage = target_joltage;

    let mut sequence: Vec<u64> = vec![0];

    let mut joltage = 0;

    for adapter in adapters {
        let adapter = *adapter;

        if (1..4).contains(&(adapter - joltage)) {
            match adapter - joltage {
                1 => {
                    sequence.push(adapter);
                }
                3 => {
                    sequence.push(adapter);
                }
                _ => {
                    unreachable!();
                }
            }

            joltage = adapter;
        } else {
            continue;
        }
    }

    if joltage + 3 == target_joltage {
        sequence.push(target_joltage);

        sequence
    } else {
        unreachable!("needed {} jolts, reached {}", target_joltage, joltage + 3);
    }
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
    fn test_day_10_a() {
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
    fn test_day_10_valid_sequences() {
        let adapters = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];

        assert_eq!(8, valid_sequences(&adapters, 0));

        let adapters = vec![
            0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35,
            38, 39, 42, 45, 46, 47, 48, 49, 52,
        ];

        assert_eq!(19208, valid_sequences(&adapters, 0));
    }

    #[test]
    fn test_day_10_joltage_differences() {
        let adapters = vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19];

        assert_eq!((7, 5, 22), joltage_differences(&adapters).unwrap());

        let adapters = vec![
            1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35,
            38, 39, 42, 45, 46, 47, 48, 49,
        ];

        assert_eq!((22, 10, 52), joltage_differences(&adapters).unwrap());
    }
}
