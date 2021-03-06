use anyhow::Result;

use aoc2020::read;

use itertools::Itertools;
use itertools::MinMaxResult;

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::collections::HashMap;
use std::convert::From;
use std::fmt;
use std::ops::Range;

fn main() -> Result<()> {
    let input = read("./17.input")?;

    println!("part A: {}", day_17_a(&input));
    println!("part B: {}", day_17_b(&input));

    Ok(())
}

fn day_17_a(input: &str) -> usize {
    let pocket = Pocket3::from(input);

    (0..6).fold(pocket, |p, _| p.step()).active()
}

fn day_17_b(input: &str) -> usize {
    let pocket = Pocket4::from(input);

    (0..6).fold(pocket, |p, _| p.step()).active()
}

type Cubes3 = HashMap<Point3, bool>;

struct Pocket3 {
    cubes: Cubes3,
    x_range: Range<i32>,
    y_range: Range<i32>,
    z_range: Range<i32>,
}

impl Pocket3 {
    fn step(self) -> Pocket3 {
        let mut cubes: Cubes3 = HashMap::new();

        self.points().for_each(|p| {
            let active = *self.cubes.get(&p).unwrap_or(&false);
            let count = p
                .neighbors()
                .filter(|n| *self.cubes.get(n).unwrap_or(&false))
                .count();

            let new = if active {
                (2..4).contains(&count)
            } else {
                count == 3
            };

            cubes.insert(p, new);
        });

        let x_range = self.x_range.start - 1..self.x_range.end + 1;
        let y_range = self.y_range.start - 1..self.y_range.end + 1;
        let z_range = self.z_range.start - 1..self.z_range.end + 1;

        Pocket3 {
            cubes,
            x_range,
            y_range,
            z_range,
        }
    }

    fn points(&self) -> PocketPointIter3 {
        PocketPointIter3 {
            pocket: self,
            x: self.x_range.start,
            y: self.y_range.start,
            z: self.z_range.start,
        }
    }

    fn active(&self) -> usize {
        self.cubes.values().filter(|v| **v).count()
    }
}

impl From<&str> for Pocket3 {
    fn from(input: &str) -> Pocket3 {
        let cubes = pocket3(input).unwrap().1;

        let x_range = match cubes.keys().minmax_by(|a, b| a.x.cmp(&b.x)) {
            MinMaxResult::MinMax(min, max) => Range {
                start: min.x - 1,
                end: max.x + 2,
            },
            _ => unreachable!(),
        };
        let y_range = match cubes.keys().minmax_by(|a, b| a.y.cmp(&b.y)) {
            MinMaxResult::MinMax(min, max) => Range {
                start: min.y - 1,
                end: max.y + 2,
            },
            _ => unreachable!(),
        };
        let z_range = match cubes.keys().minmax_by(|a, b| a.z.cmp(&b.z)) {
            MinMaxResult::MinMax(min, max) => Range {
                start: min.z - 1,
                end: max.z + 2,
            },
            _ => unreachable!(),
        };

        Pocket3 {
            cubes,
            x_range,
            y_range,
            z_range,
        }
    }
}

struct PocketPointIter3<'a> {
    pocket: &'a Pocket3,

    x: i32,
    y: i32,
    z: i32,
}

impl Iterator for PocketPointIter3<'_> {
    type Item = Point3;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.pocket.x_range.contains(&self.x)
            && !self.pocket.y_range.contains(&self.y)
            && !self.pocket.z_range.contains(&self.z)
        {
            return None;
        }

        let curr = Point3 {
            x: self.x,
            y: self.y,
            z: self.z,
        };

        if !self.pocket.x_range.contains(&self.x) && !self.pocket.y_range.contains(&self.y) {
            self.x = self.pocket.x_range.start;
            self.y = self.pocket.y_range.start;
            self.z += 1;
        } else if !self.pocket.x_range.contains(&self.x) {
            self.x = self.pocket.x_range.start;
            self.y += 1;
        } else {
            self.x += 1;
        }

        Some(curr)
    }
}

type Cubes4 = HashMap<Point4, bool>;

struct Pocket4 {
    cubes: Cubes4,
    w_range: Range<i32>,
    x_range: Range<i32>,
    y_range: Range<i32>,
    z_range: Range<i32>,
}

impl Pocket4 {
    fn step(self) -> Pocket4 {
        let mut cubes: Cubes4 = HashMap::new();

        self.points().for_each(|p| {
            let active = *self.cubes.get(&p).unwrap_or(&false);
            let count = p
                .neighbors()
                .filter(|n| *self.cubes.get(n).unwrap_or(&false))
                .count();

            let new = if active {
                (2..4).contains(&count)
            } else {
                count == 3
            };

            cubes.insert(p, new);
        });

        let w_range = self.w_range.start - 1..self.w_range.end + 1;
        let x_range = self.x_range.start - 1..self.x_range.end + 1;
        let y_range = self.y_range.start - 1..self.y_range.end + 1;
        let z_range = self.z_range.start - 1..self.z_range.end + 1;

        Pocket4 {
            cubes,
            w_range,
            x_range,
            y_range,
            z_range,
        }
    }

    fn points(&self) -> PocketPointIter4 {
        PocketPointIter4 {
            pocket: self,
            w: self.x_range.start,
            x: self.x_range.start,
            y: self.y_range.start,
            z: self.z_range.start,
        }
    }

    fn active(&self) -> usize {
        self.cubes.values().filter(|v| **v).count()
    }
}

impl fmt::Debug for Pocket3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for z in self.z_range.clone() {
            writeln!(f, "z: {}", z)?;

            for y in self.x_range.clone() {
                for x in self.y_range.clone() {
                    let p = Point3 { x, y, z };

                    let symbol = if *self.cubes.get(&p).unwrap_or(&false) {
                        "#"
                    } else {
                        "."
                    };

                    write!(f, "{}", symbol)?;
                }
                writeln!(f,)?;
            }
            writeln!(f,)?;
        }

        Ok(())
    }
}

struct PocketPointIter4<'a> {
    pocket: &'a Pocket4,

    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl Iterator for PocketPointIter4<'_> {
    type Item = Point4;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.pocket.w_range.contains(&self.w)
            && !self.pocket.x_range.contains(&self.x)
            && !self.pocket.y_range.contains(&self.y)
            && !self.pocket.z_range.contains(&self.z)
        {
            return None;
        }

        let curr = Point4 {
            w: self.w,
            x: self.x,
            y: self.y,
            z: self.z,
        };

        if !self.pocket.w_range.contains(&self.w)
            && !self.pocket.x_range.contains(&self.x)
            && !self.pocket.y_range.contains(&self.y)
        {
            self.w = self.pocket.w_range.start;
            self.x = self.pocket.x_range.start;
            self.y = self.pocket.y_range.start;
            self.z += 1;
        } else if !self.pocket.x_range.contains(&self.w) && !self.pocket.y_range.contains(&self.x) {
            self.w = self.pocket.w_range.start;
            self.x = self.pocket.x_range.start;
            self.y += 1;
        } else if !self.pocket.x_range.contains(&self.w) {
            self.w = self.pocket.x_range.start;
            self.x += 1;
        } else {
            self.w += 1;
        }

        Some(curr)
    }
}

impl From<&str> for Pocket4 {
    fn from(input: &str) -> Pocket4 {
        let cubes = pocket4(input).unwrap().1;

        let w_range = match cubes.keys().minmax_by(|a, b| a.w.cmp(&b.w)) {
            MinMaxResult::MinMax(min, max) => Range {
                start: min.w - 1,
                end: max.w + 2,
            },
            _ => unreachable!(),
        };
        let x_range = match cubes.keys().minmax_by(|a, b| a.x.cmp(&b.x)) {
            MinMaxResult::MinMax(min, max) => Range {
                start: min.x - 1,
                end: max.x + 2,
            },
            _ => unreachable!(),
        };
        let y_range = match cubes.keys().minmax_by(|a, b| a.y.cmp(&b.y)) {
            MinMaxResult::MinMax(min, max) => Range {
                start: min.y - 1,
                end: max.y + 2,
            },
            _ => unreachable!(),
        };
        let z_range = match cubes.keys().minmax_by(|a, b| a.z.cmp(&b.z)) {
            MinMaxResult::MinMax(min, max) => Range {
                start: min.z - 1,
                end: max.z + 2,
            },
            _ => unreachable!(),
        };

        Pocket4 {
            cubes,
            w_range,
            x_range,
            y_range,
            z_range,
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3 {
    fn neighbors(&self) -> Point3Neighbors {
        Point3Neighbors {
            point: self,
            dx: -1,
            dy: -1,
            dz: -1,
        }
    }
}

struct Point3Neighbors<'a> {
    point: &'a Point3,
    dx: i32,
    dy: i32,
    dz: i32,
}

impl Iterator for Point3Neighbors<'_> {
    type Item = Point3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dx == -1 && self.dy == -1 && self.dz == 2 {
            return None;
        }

        let curr = Point3 {
            x: self.point.x + self.dx,
            y: self.point.y + self.dy,
            z: self.point.z + self.dz,
        };

        if self.dx == 1 && self.dy == 1 {
            self.dx = -1;
            self.dy = -1;
            self.dz += 1;
        } else if self.dx == 1 {
            self.dx = -1;
            self.dy += 1;
        } else {
            self.dx += 1;
        }

        if let (0, 0, 0) = (self.dx, self.dy, self.dz) {
            self.dx += 1;
        }

        Some(curr)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Point4 {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl Point4 {
    fn neighbors(&self) -> Point4Neighbors {
        Point4Neighbors {
            point: self,
            dw: -1,
            dx: -1,
            dy: -1,
            dz: -1,
        }
    }
}

struct Point4Neighbors<'a> {
    point: &'a Point4,
    dw: i32,
    dx: i32,
    dy: i32,
    dz: i32,
}

impl Iterator for Point4Neighbors<'_> {
    type Item = Point4;

    fn next(&mut self) -> Option<Self::Item> {
        if self.dw == -1 && self.dx == -1 && self.dy == -1 && self.dz == 2 {
            return None;
        }

        let curr = Point4 {
            w: self.point.w + self.dw,
            x: self.point.x + self.dx,
            y: self.point.y + self.dy,
            z: self.point.z + self.dz,
        };

        if self.dw == 1 && self.dx == 1 && self.dy == 1 {
            self.dw = -1;
            self.dx = -1;
            self.dy = -1;
            self.dz += 1;
        } else if self.dw == 1 && self.dx == 1 {
            self.dw = -1;
            self.dx = -1;
            self.dy += 1;
        } else if self.dw == 1 {
            self.dw = -1;
            self.dx += 1;
        } else {
            self.dw += 1;
        }

        if let (0, 0, 0, 0) = (self.dw, self.dx, self.dy, self.dz) {
            self.dw += 1;
        }

        Some(curr)
    }
}

fn pocket3(input: &str) -> IResult<&str, Cubes3> {
    plane3(input)
}

fn plane3(input: &str) -> IResult<&str, Cubes3> {
    let mut cubes: Cubes3 = HashMap::new();

    let z = 0;

    fold_many1(terminated(row, opt(tag("\n"))), 0, |y: i32, row| {
        row.iter().enumerate().for_each(|(x, active)| {
            let x = x as i32;
            let point = Point3 { x, y, z };
            cubes.insert(point, *active);
        });

        y + 1
    })(input)?;

    Ok((input, cubes))
}

fn pocket4(input: &str) -> IResult<&str, Cubes4> {
    plane4(input)
}

fn plane4(input: &str) -> IResult<&str, Cubes4> {
    let mut cubes: Cubes4 = HashMap::new();

    let z = 0;
    let w = 0;

    fold_many1(terminated(row, opt(tag("\n"))), 0, |y: i32, row| {
        row.iter().enumerate().for_each(|(x, active)| {
            let x = x as i32;
            let point = Point4 { w, x, y, z };
            cubes.insert(point, *active);
        });

        y + 1
    })(input)?;

    Ok((input, cubes))
}

fn row(input: &str) -> IResult<&str, Vec<bool>> {
    many1(map(one_of("#."), |c| c == '#'))(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_17_a() {
        let input = ".#.
..#
###";

        assert_eq!(112, day_17_a(input));
    }

    #[test]
    fn test_day_17_pocket_from() {
        let pocket = Pocket3::from(".#.\n..#\n###");
        let cubes = pocket.cubes;

        assert_eq!(cubes.get(&Point3 { x: 0, y: 0, z: 0 }).unwrap(), &false);
        assert_eq!(cubes.get(&Point3 { x: 1, y: 0, z: 0 }).unwrap(), &true);
        assert_eq!(cubes.get(&Point3 { x: 2, y: 0, z: 0 }).unwrap(), &false);

        assert_eq!(cubes.get(&Point3 { x: 0, y: 1, z: 0 }).unwrap(), &false);
        assert_eq!(cubes.get(&Point3 { x: 1, y: 1, z: 0 }).unwrap(), &false);
        assert_eq!(cubes.get(&Point3 { x: 2, y: 1, z: 0 }).unwrap(), &true);

        assert_eq!(cubes.get(&Point3 { x: 0, y: 2, z: 0 }).unwrap(), &true);
        assert_eq!(cubes.get(&Point3 { x: 1, y: 2, z: 0 }).unwrap(), &true);
        assert_eq!(cubes.get(&Point3 { x: 2, y: 2, z: 0 }).unwrap(), &true);

        assert_eq!(pocket.x_range, (-1..4));
        assert_eq!(pocket.y_range, (-1..4));
        assert_eq!(pocket.z_range, (-1..2));
    }

    #[test]
    fn test_day_17_point3_neighbors() {
        let point = Point3 { x: 0, y: 0, z: 0 };

        let neighbors = point.neighbors().collect::<Vec<Point3>>();

        assert_eq!(
            Point3 {
                x: -1,
                y: -1,
                z: -1
            },
            neighbors[0]
        );
        assert_eq!(Point3 { x: 0, y: -1, z: -1 }, neighbors[1]);
        assert_eq!(Point3 { x: 1, y: -1, z: -1 }, neighbors[2]);

        assert_eq!(Point3 { x: -1, y: 0, z: -1 }, neighbors[3]);
        assert_eq!(Point3 { x: 0, y: 0, z: -1 }, neighbors[4]);
        assert_eq!(Point3 { x: 1, y: 0, z: -1 }, neighbors[5]);

        assert_eq!(Point3 { x: -1, y: 1, z: -1 }, neighbors[6]);
        assert_eq!(Point3 { x: 0, y: 1, z: -1 }, neighbors[7]);
        assert_eq!(Point3 { x: 1, y: 1, z: -1 }, neighbors[8]);

        assert_eq!(Point3 { x: -1, y: -1, z: 0 }, neighbors[9]);
        assert_eq!(Point3 { x: 0, y: -1, z: 0 }, neighbors[10]);
        assert_eq!(Point3 { x: 1, y: -1, z: 0 }, neighbors[11]);

        assert_eq!(Point3 { x: -1, y: 0, z: 0 }, neighbors[12]);
        assert_eq!(Point3 { x: 1, y: 0, z: 0 }, neighbors[13]);

        assert_eq!(Point3 { x: -1, y: 1, z: 0 }, neighbors[14]);
        assert_eq!(Point3 { x: 0, y: 1, z: 0 }, neighbors[15]);
        assert_eq!(Point3 { x: 1, y: 1, z: 0 }, neighbors[16]);

        assert_eq!(Point3 { x: -1, y: -1, z: 1 }, neighbors[17]);
        assert_eq!(Point3 { x: 0, y: -1, z: 1 }, neighbors[18]);
        assert_eq!(Point3 { x: 1, y: -1, z: 1 }, neighbors[19]);

        assert_eq!(Point3 { x: -1, y: 0, z: 1 }, neighbors[20]);
        assert_eq!(Point3 { x: 0, y: 0, z: 1 }, neighbors[21]);
        assert_eq!(Point3 { x: 1, y: 0, z: 1 }, neighbors[22]);

        assert_eq!(Point3 { x: -1, y: 1, z: 1 }, neighbors[23]);
        assert_eq!(Point3 { x: 0, y: 1, z: 1 }, neighbors[24]);
        assert_eq!(Point3 { x: 1, y: 1, z: 1 }, neighbors[25]);

        assert_eq!(26, neighbors.len());
    }
}
