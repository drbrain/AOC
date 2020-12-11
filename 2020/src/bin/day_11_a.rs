use anyhow::Result;

use aoc2020::read;

use std::convert::From;
use std::fmt;

fn main() -> Result<()> {
    let input = read("./11.input")?;

    println!("part A: {}", day_11_a(&input));
    //println!("part B: {}", day_11_b(&input));

    Ok(())
}

fn day_11_a(input: &str) -> usize {
    let layout = Layout::from(input);

    let stable = find_stable(layout);

    stable.occupied()
}

//fn day_11_b(input: &str) -> usize {
//    0
//}

fn find_stable(start: Layout) -> Layout {
    let mut curr = start;
    let mut prev = Layout::empty();

    while curr != prev {
        prev = curr.clone();
        curr = curr.step();
    }

    curr
}

#[derive(Clone, Eq, PartialEq)]
struct Layout {
    seats: Vec<Vec<Seat>>,
}

impl Layout {
    fn new(seats: Vec<Vec<Seat>>) -> Self {
        Layout { seats }
    }

    fn empty() -> Self {
        let seats = vec![vec![]];

        Layout { seats }
    }

    fn adjacent(&self, row: usize, col: usize) -> Adjacent {
        let max_row = self.seats.len() - 1;
        let max_col = self.seats[0].len() - 1;

        let row_min = if row == 0 { 0 } else { row - 1 };
        let row_max = if row == max_row { max_row } else { row + 1 };
        let col_min = if col == 0 { 0 } else { col - 1 };
        let col_max = if col == max_col { max_col } else { col + 1 };

        let mut seats = vec![];

        for r in row_min..=row_max {
            for c in col_min..=col_max {
                if r == row && c == col {
                    continue;
                }

                seats.push(self.seats[r][c]);
            }
        }

        Adjacent::new(seats)
    }

    fn occupied(&self) -> usize {
        self.seats()
            .filter(|(_, _, s)| Seat::Occupied == *s)
            .count()
    }

    fn seats(&self) -> SeatIter {
        SeatIter::new(self)
    }

    fn step(&self) -> Self {
        let len = self.seats.len();
        let mut new = Vec::with_capacity(len);

        for _ in 0..len {
            new.push(Vec::with_capacity(len));
        }

        for (r, c, seat) in self.seats() {
            let adjacent = self.adjacent(r, c);
            let updated = adjacent.step(seat);

            new[r].push(updated);
        }

        Layout::new(new)
    }
}

impl fmt::Debug for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("\n")?;

        for row in &self.seats {
            for seat in row {
                seat.fmt(f)?;
            }

            f.write_str("\n")?;
        }

        Ok(())
    }
}

impl From<&str> for Layout {
    fn from(string: &str) -> Self {
        let seats = string
            .lines()
            .map(|row| row.chars().map(Seat::from).collect())
            .collect();

        Layout { seats }
    }
}

struct SeatIter<'a> {
    layout: &'a Layout,
    row_max: usize,
    col_max: usize,
    row: usize,
    col: usize,
}

impl SeatIter<'_> {
    fn new<'a>(layout: &'a Layout) -> SeatIter<'a> {
        let row_max = layout.seats.len();
        let col_max = layout.seats[0].len();
        let row = 0;
        let col = 0;

        SeatIter {
            layout,
            row_max,
            col_max,
            row,
            col,
        }
    }
}

impl Iterator for SeatIter<'_> {
    type Item = (usize, usize, Seat);

    fn next(&mut self) -> Option<Self::Item> {
        if self.row == self.row_max {
            return None;
        }

        let curr = self.layout.seats[self.row][self.col];
        let row = self.row;
        let col = self.col;

        if self.col + 1 == self.col_max {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }

        Some((row, col, curr))
    }
}

#[derive(Debug)]
struct Adjacent {
    seats: Vec<Seat>,
}

impl Adjacent {
    fn new(seats: Vec<Seat>) -> Self {
        Adjacent { seats }
    }

    fn all_empty(&self) -> bool {
        self.seats
            .iter()
            .all(|s| Seat::Empty == *s || Seat::Floor == *s)
    }

    fn four_or_more_occupied(&self) -> bool {
        self.seats.iter().filter(|s| Seat::Occupied == **s).count() >= 4
    }

    fn step(&self, center: Seat) -> Seat {
        match center {
            Seat::Floor => Seat::Floor,
            Seat::Empty => {
                if self.all_empty() {
                    Seat::Occupied
                } else {
                    Seat::Empty
                }
            }
            Seat::Occupied => {
                if self.four_or_more_occupied() {
                    Seat::Empty
                } else {
                    Seat::Occupied
                }
            }
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Seat {
    Empty,
    Occupied,
    Floor,
}

impl From<char> for Seat {
    fn from(input: char) -> Self {
        match input {
            '.' => Seat::Floor,
            'L' => Seat::Empty,
            _ => panic!("unknown seat character {:?}", input),
        }
    }
}

impl fmt::Debug for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Seat::Empty => "L",
            Seat::Occupied => "#",
            Seat::Floor => ".",
        };

        f.write_str(c)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_11_a() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        assert_eq!(37, day_11_a(input));
    }

    #[test]
    fn test_day_11_adjacent() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let layout = Layout::from(input);
        let adjacent = layout.adjacent(0, 0);

        assert_eq!(3, adjacent.seats.len());
        assert_eq!(Seat::Floor, adjacent.seats[0]);
        assert_eq!(Seat::Empty, adjacent.seats[1]);
        assert_eq!(Seat::Empty, adjacent.seats[2]);

        let adjacent = layout.adjacent(9, 9);

        assert_eq!(3, adjacent.seats.len());
        assert_eq!(Seat::Floor, adjacent.seats[0]);
        assert_eq!(Seat::Empty, adjacent.seats[1]);
        assert_eq!(Seat::Empty, adjacent.seats[2]);
    }

    #[test]
    fn test_day_11_adjacent_four_or_more_occupied() {
        let adjacent = Adjacent::new(vec![Seat::Occupied, Seat::Occupied, Seat::Occupied]);

        assert_eq!(false, adjacent.four_or_more_occupied());

        let adjacent = Adjacent::new(vec![
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
        ]);

        assert_eq!(true, adjacent.four_or_more_occupied());
    }

    #[test]
    fn test_day_11_adjacent_all_empty() {
        let adjacent = Adjacent::new(vec![Seat::Floor, Seat::Empty, Seat::Empty]);

        assert_eq!(true, adjacent.all_empty());

        let adjacent = Adjacent::new(vec![Seat::Occupied, Seat::Empty, Seat::Empty]);

        assert_eq!(false, adjacent.all_empty());
    }

    #[test]
    fn test_day_11_adjacent_step() {
        let all_empty = Adjacent::new(vec![Seat::Floor, Seat::Empty]);
        let one_occupied = Adjacent::new(vec![Seat::Floor, Seat::Occupied, Seat::Empty]);
        let four_occupied = Adjacent::new(vec![
            Seat::Floor,
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
        ]);

        assert_eq!(Seat::Occupied, all_empty.step(Seat::Empty));
        assert_eq!(Seat::Empty, one_occupied.step(Seat::Empty));
        assert_eq!(Seat::Empty, four_occupied.step(Seat::Empty));

        assert_eq!(Seat::Occupied, all_empty.step(Seat::Occupied));
        assert_eq!(Seat::Empty, four_occupied.step(Seat::Occupied));

        let bug1 = Adjacent::new(vec![
            Seat::Occupied,
            Seat::Floor,
            Seat::Occupied,
            Seat::Occupied,
            Seat::Floor,
        ]);
        assert_eq!(Seat::Occupied, bug1.step(Seat::Occupied));
    }

    #[test]
    fn test_day_11_seat_iter() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let layout = Layout::from(input);
        let mut iter = SeatIter::new(&layout);

        assert_eq!((0, 0, Seat::Empty), iter.next().unwrap());
        assert_eq!((0, 1, Seat::Floor), iter.next().unwrap());
        assert_eq!((0, 2, Seat::Empty), iter.next().unwrap());
    }

    #[test]
    fn test_day_11_step() {
        let input = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

        let layout = Layout::from(input);
        let step = layout.step();

        let expected = "
#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
";

        assert_eq!(expected, format!("{:?}", step));

        let step = step.step();
        let expected = "
#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
";

        dbg!(&step);
        assert_eq!(expected, format!("{:?}", step));

        let step = step.step();
        let expected = "
#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
";

        assert_eq!(expected, format!("{:?}", step));

        let step = step.step();
        let expected = "
#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
";

        assert_eq!(expected, format!("{:?}", step));

        let step = step.step();
        let expected = "
#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
";

        assert_eq!(expected, format!("{:?}", step));
    }
}
