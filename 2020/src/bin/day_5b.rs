use anyhow::Result;

use aoc2020::read;

fn main() -> Result<()> {
    let input = read("./05.input")?;

    println!("part A: {}", day_5b_a(&input));
    println!("part B: {}", day_5b_b(&input));

    Ok(())
}

fn day_5b_a(input: &str) -> u32 {
    input.split('\n').map(|path| seat_id(path)).max().unwrap()
}

fn day_5b_b(input: &str) -> u32 {
    let mut seat_ids: Vec<u32> = input.split('\n').map(|path| seat_id(path)).collect();

    seat_ids.sort_unstable();

    let offset = seat_ids[0];

    let seat_id = seat_ids
        .iter()
        .enumerate()
        .find(|(index, id)| *index as u32 + offset != **id)
        .unwrap()
        .1;

    *seat_id - 1
}

fn seat_id(path: &str) -> u32 {
    let path = String::from(path);
    let path = path
        .replace("F", "0")
        .replace("B", "1")
        .replace("L", "0")
        .replace("R", "1");

    u32::from_str_radix(&path, 2).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_5b_seat_id() {
        assert_eq!(357, seat_id("FBFBBFFRLR"));
        assert_eq!(567, seat_id("BFFFBBFRRR"));
        assert_eq!(119, seat_id("FFFBBBFRRR"));
        assert_eq!(820, seat_id("BBFFBBFRLL"));
    }

    #[test]
    fn test_day_5b() {
        let input = "FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

        assert_eq!(820, day_5b_a(input));
    }
}
