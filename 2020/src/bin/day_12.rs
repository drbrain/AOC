use anyhow::Result;

use aoc2020::read;

use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use std::convert::From;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;

fn main() -> Result<()> {
    let input = read("./12.input")?;

    println!("part A: {}", day_12_a(&input));
    println!("part B: {}", day_12_b(&input));

    Ok(())
}

fn day_12_a(input: &str) -> i32 {
    let actions = actions(input).unwrap().1;

    let mut ship = Ship::default();

    ship.navigate_wrong(&actions);

    ship.manhattan()
}

fn day_12_b(input: &str) -> i32 {
    let actions = actions(input).unwrap().1;

    let mut ship = Ship::default();

    ship.navigate_waypoint(&actions);

    ship.manhattan()
}

#[derive(Clone, Eq, PartialEq)]
struct Ship {
    facing: Direction,
    waypoint: Waypoint,
    n: i32,
    e: i32,
}

impl Ship {
    fn default() -> Self {
        Ship {
            facing: Direction::E,
            waypoint: Waypoint::default(),
            n: 0,
            e: 0,
        }
    }

    #[allow(dead_code)]
    fn new(facing: Direction, waypoint: Waypoint, n: i32, e: i32) -> Self {
        Ship {
            facing,
            waypoint,
            n,
            e,
        }
    }

    fn manhattan(&self) -> i32 {
        self.n.abs() + self.e.abs()
    }

    fn navigate_waypoint(&mut self, actions: &[Action]) {
        for action in actions {
            match action {
                Action::N(v) => self.move_waypoint(action.into(), *v),
                Action::S(v) => self.move_waypoint(action.into(), *v),
                Action::E(v) => self.move_waypoint(action.into(), *v),
                Action::W(v) => self.move_waypoint(action.into(), *v),
                Action::F(v) => self.move_to_waypoint(*v),
                Action::L(v) => self.waypoint.rotate(action.into(), *v),
                Action::R(v) => self.waypoint.rotate(action.into(), *v),
            }
        }
    }

    fn navigate_wrong(&mut self, actions: &[Action]) {
        for action in actions {
            match action {
                Action::N(v) => self.move_ship(action.into(), *v),
                Action::S(v) => self.move_ship(action.into(), *v),
                Action::E(v) => self.move_ship(action.into(), *v),
                Action::W(v) => self.move_ship(action.into(), *v),
                Action::F(v) => self.move_ship(self.facing.clone(), *v),
                Action::L(v) => self.turn_ship(action.into(), *v),
                Action::R(v) => self.turn_ship(action.into(), *v),
            }
        }
    }

    fn move_ship(&mut self, direction: Direction, units: i32) {
        match direction {
            Direction::N => self.n += units,
            Direction::S => self.n -= units,
            Direction::E => self.e += units,
            Direction::W => self.e -= units,
        }
    }

    fn move_to_waypoint(&mut self, times: i32) {
        self.n += self.waypoint.n * times;
        self.e += self.waypoint.e * times;
    }

    fn move_waypoint(&mut self, direction: Direction, units: i32) {
        match direction {
            Direction::N => self.waypoint.n += units,
            Direction::S => self.waypoint.n -= units,
            Direction::E => self.waypoint.e += units,
            Direction::W => self.waypoint.e -= units,
        }
    }

    fn turn_ship(&mut self, rotation: Rotation, degrees: i32) {
        self.facing = match rotation {
            Rotation::R => self.facing.clone() + degrees,
            Rotation::L => self.facing.clone() - degrees,
        };
    }
}

impl fmt::Debug for Ship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let facing = match self.facing {
            Direction::N => "N",
            Direction::S => "S",
            Direction::E => "E",
            Direction::W => "W",
        };

        f.write_fmt(format_args!(
            "facing {} at {}N, {}E",
            facing, self.n, self.e
        ))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct Waypoint {
    n: i32,
    e: i32,
}

impl Waypoint {
    fn default() -> Waypoint {
        Waypoint { n: 1, e: 10 }
    }

    #[allow(dead_code)]
    fn new(n: i32, e: i32) -> Waypoint {
        Waypoint { n, e }
    }

    fn rotate(&mut self, rotation: Rotation, degrees: i32) {
        let degrees = match rotation {
            Rotation::R => degrees,
            Rotation::L => -degrees,
        };

        let turns = (4 + degrees / 90) % 4;

        let n = self.n;
        let e = self.e;

        match turns {
            1 => {
                self.e = n;
                self.n = -e
            }
            2 => {
                self.e = -e;
                self.n = -n
            }
            3 => {
                self.e = -n;
                self.n = e
            }
            _ => unreachable!("unhandled rotation {} turns from {:?}", turns, self),
        }
    }
}

impl fmt::Debug for Waypoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}N, {}E", self.n, self.e))
    }
}

enum Rotation {
    R,
    L,
}

impl From<&Action> for Rotation {
    fn from(action: &Action) -> Rotation {
        match action {
            Action::R(_) => Rotation::R,
            Action::L(_) => Rotation::L,
            _ => panic!("Unable to convert {:?} into a rotation", action),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Add<i32> for Direction {
    type Output = Direction;

    fn add(self, rhs: i32) -> Direction {
        let dir = match self {
            Direction::E => 0,
            Direction::S => 1,
            Direction::W => 2,
            Direction::N => 3,
        };

        match (dir + rhs / 90) % 4 {
            0 => Direction::E,
            1 => Direction::S,
            2 => Direction::W,
            3 => Direction::N,
            _ => unreachable!("error rotating {} degrees around {:?}", rhs, self),
        }
    }
}

impl Sub<i32> for Direction {
    type Output = Direction;

    fn sub(self, rhs: i32) -> Direction {
        let units = rhs / 90;

        let dir = match self {
            Direction::E => 0,
            Direction::S => 1,
            Direction::W => 2,
            Direction::N => 3,
        };

        match (dir - rhs / 90) % 4 {
            -3 => Direction::S,
            -2 => Direction::W,
            -1 => Direction::N,
            0 => Direction::E,
            1 => Direction::S,
            2 => Direction::W,
            3 => Direction::N,
            _ => unreachable!(
                "unhandled units {} rotating {} degrees around {:?}",
                units, rhs, self
            ),
        }
    }
}

impl From<&Action> for Direction {
    fn from(action: &Action) -> Direction {
        match action {
            Action::N(_) => Direction::N,
            Action::S(_) => Direction::S,
            Action::E(_) => Direction::E,
            Action::W(_) => Direction::W,
            _ => panic!("Unable to convert {:?} into a direction", action),
        }
    }
}

#[derive(Eq, PartialEq)]
enum Action {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl From<(char, i32)> for Action {
    fn from(input: (char, i32)) -> Action {
        let (action, value) = input;

        match action {
            'N' => Action::N(value),
            'S' => Action::S(value),
            'E' => Action::E(value),
            'W' => Action::W(value),
            'L' => Action::L(value),
            'R' => Action::R(value),
            'F' => Action::F(value),
            _ => unreachable!("unknown action {} ({})", action, value),
        }
    }
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::N(n) => f.write_fmt(format_args!("N{:<3}", n)),
            Action::S(n) => f.write_fmt(format_args!("S{:<3}", n)),
            Action::E(n) => f.write_fmt(format_args!("E{:<3}", n)),
            Action::W(n) => f.write_fmt(format_args!("W{:<3}", n)),
            Action::L(n) => f.write_fmt(format_args!("L{:<3}", n)),
            Action::R(n) => f.write_fmt(format_args!("R{:<3}", n)),
            Action::F(n) => f.write_fmt(format_args!("F{:<3}", n)),
        }
    }
}

fn actions(input: &str) -> IResult<&str, Vec<Action>> {
    separated_list1(tag("\n"), action)(input)
}

fn action(input: &str) -> IResult<&str, Action> {
    map(tuple((one_of("NSEWLRF"), number)), Action::from)(input)
}

fn number(input: &str) -> IResult<&str, i32> {
    map(recognize(digit1), |s: &str| s.parse::<i32>().unwrap())(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day_12_a() {
        let input = "F10
N3
F7
R90
F11";

        assert_eq!(25, day_12_a(input));
    }

    #[test]
    fn test_day_12_actions() {
        let input = "F10
N3
F7
R90
F11";

        let expected = vec![
            Action::F(10),
            Action::N(3),
            Action::F(7),
            Action::R(90),
            Action::F(11),
        ];

        assert_eq!(expected, actions(input).unwrap().1);
    }

    #[test]
    fn test_day_12_navigate_waypoint() {
        let mut ship = Ship::default();

        ship.navigate_waypoint(&vec![Action::F(10)]);
        assert_eq!(Ship::new(Direction::E, Waypoint::new(1, 10), 10, 100), ship);

        ship.navigate_waypoint(&vec![Action::N(3)]);
        assert_eq!(Ship::new(Direction::E, Waypoint::new(4, 10), 10, 100), ship);

        ship.navigate_waypoint(&vec![Action::F(7)]);
        assert_eq!(Ship::new(Direction::E, Waypoint::new(4, 10), 38, 170), ship);

        ship.navigate_waypoint(&vec![Action::R(90)]);
        assert_eq!(
            Ship::new(Direction::E, Waypoint::new(-10, 4), 38, 170),
            ship
        );

        ship.navigate_waypoint(&vec![Action::F(11)]);
        assert_eq!(
            Ship::new(Direction::E, Waypoint::new(-10, 4), -72, 214),
            ship
        );
    }

    #[test]
    fn test_day_12_navigate_wrong() {
        let mut ship = Ship::default();

        ship.navigate_wrong(&vec![Action::E(5)]);
        assert_eq!(Ship::new(Direction::E, Waypoint::default(), 0, 5), ship);

        ship.navigate_wrong(&vec![Action::R(90)]);
        assert_eq!(Ship::new(Direction::S, Waypoint::default(), 0, 5), ship);

        ship.navigate_wrong(&vec![Action::E(5)]);
        assert_eq!(Ship::new(Direction::S, Waypoint::default(), 0, 10), ship);
    }

    #[test]
    fn test_day_12_rotation() {
        assert_eq!(Direction::S, Direction::E + 90_i32);
        assert_eq!(Direction::W, Direction::S + 90_i32);
        assert_eq!(Direction::N, Direction::W + 90_i32);
        assert_eq!(Direction::E, Direction::N + 90_i32);

        assert_eq!(Direction::W, Direction::E + 180_i32);
        assert_eq!(Direction::N, Direction::E + 270_i32);
        assert_eq!(Direction::E, Direction::E + 360_i32);
        assert_eq!(Direction::S, Direction::E + 450_i32);

        assert_eq!(Direction::N, Direction::E - 90_i32);
        assert_eq!(Direction::W, Direction::N - 90_i32);
        assert_eq!(Direction::S, Direction::W - 90_i32);
        assert_eq!(Direction::E, Direction::S - 90_i32);

        assert_eq!(Direction::W, Direction::E - 180_i32);
        assert_eq!(Direction::S, Direction::E - 270_i32);
        assert_eq!(Direction::E, Direction::E - 360_i32);
        assert_eq!(Direction::N, Direction::E - 450_i32);
    }
}
