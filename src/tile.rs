use std::fmt::{Display, Formatter, Result};
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub struct Tile {
    pub q: i8,
    pub r: i8,
    pub s: i8,
}

#[macro_export]
macro_rules! tile {
    ($q:expr, $r:expr, $s:expr) => {
        Tile::new($q, $r, $s)
    };
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    E,
    W,
    NW,
    NE,
    SW,
    SE,
}

pub const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::E,
    Direction::W,
    Direction::NW,
    Direction::NE,
    Direction::SW,
    Direction::SE,
];

pub const REVERSE_DIRECTION: [Direction; 6] = [
    Direction::W,
    Direction::E,
    Direction::SE,
    Direction::SW,
    Direction::NE,
    Direction::NW,
];

impl Tile {
    pub fn new(q: i8, r: i8, s: i8) -> Self {
        Tile { q, r, s }
    }

    pub fn move_towards(&self, direction: Direction, steps: i8) -> Tile {
        let delta = match direction {
            Direction::W => tile!(-steps, 0, steps),
            Direction::E => tile!(steps, 0, -steps),
            Direction::NW => tile!(0, -steps, steps),
            Direction::NE => tile!(steps, -steps, 0),
            Direction::SW => tile!(-steps, steps, 0),
            Direction::SE => tile!(0, steps, -steps),
        };
        *self + delta
    }

    pub fn neighbors(&self) -> Vec<Tile> {
        let mut output = vec![];
        for direction in ALL_DIRECTIONS {
            output.push(self.move_towards(direction, 1))
        }
        output
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(q: {}, r: {}, s: {})", self.q, self.r, self.s)
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let char = match self {
            Direction::E | Direction::W => '-',
            Direction::NW | Direction::SE => '\\',
            Direction::NE | Direction::SW => '/',
        };
        let prefix = match self {
            Direction::E | Direction::SE | Direction::NE => '>',
            Direction::W | Direction::SW | Direction::NW => '<',
        };
        write!(f, "{prefix}{char}")
    }
}

impl Add for Tile {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.q + other.q, self.r + other.r, self.s + other.s)
    }
}

impl Sub for Tile {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.q - other.q, self.r - other.r, self.s - other.s)
    }
}
