use anyhow::Result;

use aoc2020::read;

use std::convert::From;

fn main() -> Result<()> {
    let input = read("./03.input")?;

    println!("part A: {}", day_3_a(&input, &Slope::new(3, 1))?);

    println!("part B: {}", day_3_b(&input)?);

    Ok(())
}

fn day_3_a(input: &String, slope: &Slope) -> Result<u64> {
    let field = Field::from(input);

    Ok(field.toboggan(slope))
}

fn day_3_b(input: &String) -> Result<u64> {
    let field = Field::from(input);

    let slopes = vec![
        Slope::new(1, 1),
        Slope::new(3, 1),
        Slope::new(5, 1),
        Slope::new(7, 1),
        Slope::new(1, 2),
    ];

    let answer = slopes
        .into_iter()
        .map(|slope| field.toboggan(&slope))
        .fold(1, |a, b| a * b);

    Ok(answer)
}

#[derive(Debug)]
struct Slope {
    pub run: usize,
    pub drop: usize,
}

impl Slope {
    fn new(run: usize, drop: usize) -> Self {
        Slope { run, drop }
    }
}

#[derive(Debug)]
struct Field {
    rows: Vec<Vec<bool>>,
}

impl Field {
    fn toboggan(&self, slope: &Slope) -> u64 {
        self.trees(slope).filter(|t| *t).count() as u64
    }

    fn trees(&self, slope: &Slope) -> FieldIter {
        FieldIter::new(self, slope)
    }
}

impl From<&String> for Field {
    fn from(string: &String) -> Self {
        let rows = string
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        Field { rows }
    }
}

struct FieldIter<'a> {
    field: &'a Field,
    run: usize,
    drop: usize,
    width: usize,
    height: usize,
    row: usize,
    col: usize,
}

impl FieldIter<'_> {
    fn new<'a>(field: &'a Field, slope: &Slope) -> FieldIter<'a> {
        let drop = slope.drop;
        let run = slope.run;
        let row = slope.drop;
        let col = slope.run;
        let width = field.rows[0].len();
        let height = field.rows.len();

        FieldIter {
            field,
            run,
            drop,
            width,
            height,
            row,
            col,
        }
    }
}

impl Iterator for FieldIter<'_> {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= self.height {
            return None;
        }

        let curr = self.field.rows[self.row][self.col];

        self.row += self.drop;
        self.col += self.run;
        self.col %= self.width;

        Some(curr)
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

        assert_eq!(7, day_3_a(&input, 3, 1).unwrap());
    }

    #[test]
    fn test_day_3_b() {
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

        assert_eq!(336, day_3_b(&input).unwrap());
    }
}
