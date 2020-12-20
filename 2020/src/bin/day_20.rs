use anyhow::Result;

use aoc2020::read;

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
    //println!("part B: {}", day_20_b(&input));

    Ok(())
}

fn day_20_a(input: &str) -> u64 {
    let tiles = tiles(input).unwrap().1;

    let corners = fit(&tiles);

    corners.iter().product()
}

fn fit(tiles: &Vec<Tile>) -> Vec<u64> {
    // map a tile to its edges
    let mut tile_edges = HashMap::new();
    // map an edge to the tiles that have that edge
    let mut edge_tiles: HashMap<Vec<bool>, Vec<&Tile>> = HashMap::new();

    for tile in tiles {
        for edge in tile.edges() {
            //eprintln!("tile {} edge {:?}", tile.id, edge);
            tile_edges.insert(tile, edge.clone());

            if let Some(v) = edge_tiles.get_mut(&edge) {
                v.push(tile);
            } else {
                edge_tiles.insert(edge, vec![tile]);
            }
        }
    }

    //edge_tiles
    //    .iter()
    //    .for_each(|(e, ts)| eprintln!("edge {:?} matches {} tiles", e, ts.len()));

    // match a tile to its adjacent tiles
    let mut matches: HashMap<&Tile, Vec<&Tile>> = HashMap::new();

    for tile in tiles {
        for edge in tile.edges() {
            match edge_tiles.get(&edge) {
                Some(m) => match m.iter().find(|t| *t != &tile) {
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

    matches
        .iter()
        .filter(|(_, ts)| ts.len() == 4)
        .map(|(t, _)| t.id)
        .collect()
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
            2 => image.last().unwrap().to_vec(),
            3 => image.last().unwrap().iter().rev().copied().collect(),
            4 => self.tile.left_edge(),
            5 => self.tile.left_edge().iter().rev().copied().collect(),
            6 => self.tile.right_edge(),
            7 => self.tile.right_edge().iter().rev().copied().collect(),
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
}
