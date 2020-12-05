use anyhow::Result;

use aoc2020::read;

use std::cmp::Ordering::Less;
use std::cmp::Ordering::Equal;
use std::cmp::Ordering::Greater;

fn main() -> Result<()> {
    let input = read("./05.input")?;

    println!("part A: {}", day_5_a(&input));
    //println!("part B: {}", day_5_b(&input)?);

    Ok(())
}

fn day_5_a(input: &str) -> usize {
    input.split('\n').map(|path| seat_id(path)).max().unwrap()
}

fn seat_id(path: &str) -> usize {
    let row_path = &path[0..7];
    let seat_path = &path[7..10];

    find_row(row_path) * 8 + find_seat(seat_path)
}

fn collection(size: usize) -> Vec<usize> {
    let mut collection = Vec::with_capacity(size);

    for item in 0..size {
        collection.push(item);
    }

    collection

}

fn find(collection: Vec<usize>, path: &str, less: char) -> usize {
    let mut by = path.chars().map(|c| if c == less { Less } else { Greater });

    collection.binary_search_by(|_| by.next().unwrap_or(Equal)).unwrap()
}

fn rows() -> Vec<usize> {
    collection(128)
}

fn find_row(path: &str) -> usize {
    find(rows(), path, 'B')
}

fn seats() -> Vec<usize> {
    collection(8)
}

fn find_seat(path: &str) -> usize {
    find(seats(), path, 'R')
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_5_find_row() {
        assert_eq!(44, find_row("FBFBBFF"));
        assert_eq!(70, find_row("BFFFBBF"));
        assert_eq!(14, find_row("FFFBBBF"));
        assert_eq!(102, find_row("BBFFBBF"));
    }

    #[test]
    fn test_day_5_find_seat() {
        assert_eq!(5, find_seat("RLR"));
        assert_eq!(7, find_seat("RRR"));
        assert_eq!(7, find_seat("RRR"));
        assert_eq!(4, find_seat("RLL"));
    }

    #[test]
    fn test_day_5_seat_id() {
        assert_eq!(357, seat_id("FBFBBFFRLR"));
        assert_eq!(567, seat_id("BFFFBBFRRR"));
        assert_eq!(119, seat_id("FFFBBBFRRR"));
        assert_eq!(820, seat_id("BBFFBBFRLL"));
    }

    #[test]
    fn test_day_5() {
        let input = "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

        assert_eq!(820, day_5_a(input));
    }

}
