use anyhow::Result;

use aoc2020::read;

use std::convert::From;
use std::fmt;

fn main() -> Result<()> {
    let input = read("./11.input")?;

    println!("part B: {}", day_11_b(&input));

    Ok(())
}

fn day_11_b(input: &str) -> usize {
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
        let mut seats = Vec::with_capacity(8);

        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }

                seats.push(self.find_seat(row, col, dr, dc))
            }
        }

        Adjacent::new(seats)
    }

    fn find_seat(&self, row: usize, col: usize, dr: isize, dc: isize) -> Seat {
        let max_row = self.seats.len() as isize;
        let max_col = self.seats[0].len() as isize;

        let mut r = row as isize + dr;
        let mut c = col as isize + dc;

        loop {
            if !((0..max_row).contains(&r) && (0..max_col).contains(&c)) {
                break Seat::Floor;
            }

            match self.seats[r as usize][c as usize] {
                Seat::Floor => {
                    r += dr;
                    c += dc;
                }
                Seat::Empty => {
                    break Seat::Empty;
                }
                Seat::Occupied => {
                    break Seat::Occupied;
                }
            }
        }
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
    fn new(layout: &Layout) -> SeatIter {
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

    fn five_or_more_occupied(&self) -> bool {
        self.seats.iter().filter(|s| Seat::Occupied == **s).count() >= 5
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
                if self.five_or_more_occupied() {
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
            '#' => Seat::Occupied,
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
    fn test_day_11b_b() {
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

        assert_eq!(26, day_11_b(input));
    }

    #[test]
    fn test_day_11b_adjacent() {
        let input = ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";

        let layout = Layout::from(input);
        dbg!(&layout);
        let adjacent = layout.adjacent(4, 3);

        assert_eq!(8, adjacent.seats.len());
        assert_eq!(Seat::Occupied, adjacent.seats[0]);
        assert_eq!(Seat::Occupied, adjacent.seats[1]);
        assert_eq!(Seat::Occupied, adjacent.seats[2]);
        assert_eq!(Seat::Occupied, adjacent.seats[3]);
        assert_eq!(Seat::Occupied, adjacent.seats[4]);
        assert_eq!(Seat::Occupied, adjacent.seats[5]);
        assert_eq!(Seat::Occupied, adjacent.seats[6]);
        assert_eq!(Seat::Occupied, adjacent.seats[7]);

        let input = ".............
.L.L.#.#.#.#.
.............";

        let layout = Layout::from(input);
        let adjacent = layout.adjacent(1, 1);

        assert_eq!(Seat::Floor, adjacent.seats[0]);
        assert_eq!(Seat::Empty, adjacent.seats[4]);

        let adjacent = layout.adjacent(1, 3);

        assert_eq!(Seat::Floor, adjacent.seats[0]);
        assert_eq!(Seat::Empty, adjacent.seats[3]);
        assert_eq!(Seat::Occupied, adjacent.seats[4]);

        let input = ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.";

        let layout = Layout::from(input);
        let adjacent = layout.adjacent(3, 3);

        assert_eq!(true, adjacent.all_empty());
    }

    #[test]
    fn test_day_11b_adjacent_five_or_more_occupied() {
        let adjacent = Adjacent::new(vec![
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
        ]);

        assert_eq!(false, adjacent.five_or_more_occupied());

        let adjacent = Adjacent::new(vec![
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
        ]);

        assert_eq!(true, adjacent.five_or_more_occupied());
    }

    #[test]
    fn test_day_11b_adjacent_all_empty() {
        let adjacent = Adjacent::new(vec![Seat::Floor, Seat::Empty, Seat::Empty]);

        assert_eq!(true, adjacent.all_empty());

        let adjacent = Adjacent::new(vec![Seat::Occupied, Seat::Empty, Seat::Empty]);

        assert_eq!(false, adjacent.all_empty());
    }

    #[test]
    fn test_day_11b_adjacent_step() {
        let all_empty = Adjacent::new(vec![Seat::Floor, Seat::Empty]);
        let one_occupied = Adjacent::new(vec![Seat::Floor, Seat::Occupied, Seat::Empty]);
        let five_occupied = Adjacent::new(vec![
            Seat::Floor,
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
            Seat::Occupied,
        ]);

        assert_eq!(Seat::Occupied, all_empty.step(Seat::Empty));
        assert_eq!(Seat::Empty, one_occupied.step(Seat::Empty));
        assert_eq!(Seat::Empty, five_occupied.step(Seat::Empty));

        assert_eq!(Seat::Occupied, all_empty.step(Seat::Occupied));
        assert_eq!(Seat::Empty, five_occupied.step(Seat::Occupied));

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
    fn test_day_11b_seat_iter() {
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
    fn test_day_11b_step() {
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
