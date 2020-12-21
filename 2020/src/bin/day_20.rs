use anyhow::Result;

use aoc2020::read;

use itertools::Itertools;

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::collections::HashMap;
use std::hash::Hash;

fn main() -> Result<()> {
    let input = read("./20.input")?;

    println!("part A: {}", day_20_a(&input));
    println!("part B: {}", day_20_b(&input));

    Ok(())
}

fn day_20_a(input: &str) -> u64 {
    let tiles = tiles(input).unwrap().1;

    let mut builder = ImageBuilder::new(&tiles);

    builder.fit();

    builder.corners().iter().product()
}

fn day_20_b(input: &str) -> u64 {
    let tiles = tiles(input).unwrap().1;

    let mut builder = ImageBuilder::new(&tiles);
    builder.fit();
    builder.build();
    builder.count_sea_monsters()
}

type Matches<'a> = HashMap<&'a Tile, Vec<&'a Tile>>;

struct ImageBuilder<'a> {
    tiles: HashMap<u64, &'a Tile>,
    // map an edge to the tiles that have that edge
    edge_tiles: HashMap<Vec<bool>, Vec<&'a Tile>>,
    matches: Option<Matches<'a>>,
    //image: Option<Vec<Vec<bool>>>,
}

impl ImageBuilder<'_> {
    fn new<'a>(tiles: &'a Vec<Tile>) -> ImageBuilder {
        let mut tiles_map: HashMap<u64, &Tile> = HashMap::new();
        let mut edge_tiles: HashMap<Vec<bool>, Vec<&Tile>> = HashMap::new();

        for tile in tiles {
            tiles_map.insert(tile.id, tile);

            for edge in tile.edges() {
                //eprintln!("tile {} edge {:?}", tile.id, edge);
                if let Some(v) = edge_tiles.get_mut(&edge) {
                    v.push(tile);
                } else {
                    edge_tiles.insert(edge, vec![tile]);
                }
            }
        }

        ImageBuilder {
            tiles: tiles_map,
            edge_tiles,
            matches: None,
            //image: None,
        }
    }

    fn build(&self) {
        let curr = self.tiles.get(&self.corners()[0]).unwrap();
        let neighbors = self.neighbors(curr);

        let edges: Vec<(usize, Vec<bool>)> = curr
            .edges()
            .enumerate()
            .filter(|(_, e)| neighbors.iter().any(|n| n.edges().any(|ne| ne == *e)))
            .collect();

        // find pair that are one apart
        dbg!(edges);

        loop {
            // if no placed neighbor { break; }
            break;
        }
    }

    fn corners(&self) -> Vec<u64> {
        if let Some(m) = &self.matches {
            m.iter()
                .filter(|(_, ts)| ts.len() == 4)
                .map(|(t, _)| t.id)
                .collect()
        } else {
            panic!("fit() before finding corners");
        }
    }

    fn count_sea_monsters(&self) -> u64 {
        0
    }

    fn fit(&mut self) {
        //edge_tiles
        //    .iter()
        //    .for_each(|(e, ts)| eprintln!("edge {:?} matches {} tiles", e, ts.len()));

        // match a tile to its adjacent tiles
        let mut matches: HashMap<&Tile, Vec<&Tile>> = HashMap::new();

        for tile in self.tiles.values() {
            for edge in tile.edges() {
                match self.edge_tiles.get(&edge) {
                    Some(m) => match m.iter().find(|t| *t != tile) {
                        Some(t) => {
                            if let Some(v) = matches.get_mut(tile) {
                                v.push(t);
                            } else {
                                matches.insert(tile, vec![t]);
                            }
                        }
                        None => (),
                    },
                    None => panic!("could not find edge for tile {}", tile.id),
                }
            }
        }

        //matches
        //    .iter()
        //    .for_each(|(t, ts)| eprintln!("tile {} matches {} tiles", t.id, ts.len()));

        self.matches = Some(matches);
    }

    fn neighbors(&self, tile: &Tile) -> Vec<&Tile> {
        let matches = if let Some(m) = &self.matches {
            m
        } else {
            panic!("fit() before fetching neighbors");
        };

        matches
            .get(tile)
            .unwrap()
            .iter()
            .copied()
            .unique_by(|t| t.id)
            .collect()
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Tile {
    id: u64,
    image: Vec<Vec<bool>>,
}

impl Tile {
    fn new((id, image): (u64, Vec<Vec<bool>>)) -> Tile {
        Tile { id, image }
    }

    #[allow(dead_code)]
    fn trim(&self) -> Vec<Vec<bool>> {
        let col_len = self.image.len();
        let row_len = self.image[0].len();

        self.image[1..col_len - 1]
            .iter()
            .map(|row| row[1..row_len - 1].to_vec())
            .collect()
    }

    fn edges(&self) -> EdgeIterator {
        EdgeIterator::new(self)
    }

    fn right_edge(&self) -> Vec<bool> {
        self.image
            .iter()
            .map(|r| r.last().unwrap())
            .copied()
            .collect()
    }

    fn left_edge(&self) -> Vec<bool> {
        self.image.iter().map(|r| r[0]).collect()
    }
}

#[derive(Debug)]
struct EdgeIterator<'a> {
    tile: &'a Tile,
    edge: usize,
}

impl EdgeIterator<'_> {
    fn new<'a>(tile: &'a Tile) -> EdgeIterator {
        EdgeIterator { tile, edge: 0 }
    }
}

impl Iterator for EdgeIterator<'_> {
    type Item = Vec<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.edge > 7 {
            return None;
        }

        let image = &self.tile.image;

        let curr = match self.edge {
            0 => image[0].clone(),
            1 => image[0].iter().rev().copied().collect(),
            2 => self.tile.right_edge(),
            3 => self.tile.right_edge().iter().rev().copied().collect(),
            4 => image.last().unwrap().to_vec(),
            5 => image.last().unwrap().iter().rev().copied().collect(),
            6 => self.tile.left_edge(),
            7 => self.tile.left_edge().iter().rev().copied().collect(),
            _ => unreachable!(),
        };

        self.edge += 1;

        Some(curr)
    }
}

fn tiles(input: &str) -> IResult<&str, Vec<Tile>> {
    separated_list1(tag("\n\n"), tile)(input)
}

fn tile(input: &str) -> IResult<&str, Tile> {
    map(pair(tile_id, tile_image), Tile::new)(input)
}

fn tile_id(input: &str) -> IResult<&str, u64> {
    delimited(tag("Tile "), num_u64, tag(":\n"))(input)
}

fn tile_image(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
    separated_list1(tag("\n"), tile_row)(input)
}

fn tile_row(input: &str) -> IResult<&str, Vec<bool>> {
    fold_many1(
        map(one_of("#."), |c| c == '#'),
        Vec::new(),
        |mut acc: Vec<_>, item| {
            acc.push(item);
            acc
        },
    )(input)
}

fn num_u64(input: &str) -> IResult<&str, u64> {
    map(recognize(digit1), |s: &str| s.parse::<u64>().unwrap())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_20_a() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

        assert_eq!(20899048083289, day_20_a(input));
    }

    #[test]
    fn test_day_20_b() {
        let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

        assert_eq!(273, day_20_b(input));
    }

    #[test]
    fn test_day_20_tile_trim() {
        let t = tile(
            "Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...",
        )
        .unwrap()
        .1;

        let expected = vec![
            vec![true, false, false, true, true, true, true, true],
            vec![false, true, false, false, false, false, false, false],
            vec![true, true, true, true, true, false, false, false],
            vec![true, true, true, false, true, false, false, true],
            vec![true, false, false, false, true, false, true, true],
            vec![false, true, true, true, true, true, false, true],
            vec![false, true, false, true, true, true, false, false],
            vec![false, true, false, false, false, false, false, false],
        ];

        assert_eq!(expected, t.trim());
    }
}
