use anyhow::Result;

use aoc2020::read;

use std::convert::From;

fn main() -> Result<()> {
    let input = read("./03.input")?;

    println!("part A: {}", day_3_a(&input, 3, 1)?);

    Ok(())
}

fn day_3_a(input: &String, run: usize, drop: usize) -> Result<u32> {
    let slope = Slope::from(input);

    Ok(slope.toboggan(run, drop)?)
}

#[derive(Debug)]
struct Slope {
    rows: Vec<Vec<bool>>,
}

impl Slope {
    fn toboggan(&self, run: usize, drop: usize) -> Result<u32> {
        let mut row = drop;
        let mut col = run;
        let width = self.rows[0].len();
        let mut trees = 0;

        while row < self.rows.len() {
            col %= width;

            if self.rows[row][col] {
                trees += 1;
            }

            row += drop;
            col += run;
        }

        Ok(trees)
    }
}

impl From<&String> for Slope {
    fn from(string: &String) -> Self {
        let rows = string
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        Slope { rows }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_3_a() {
        let input = String::from(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        );

        assert_eq!(7, day_3_a(input, 3, 1).unwrap());
    }
}
