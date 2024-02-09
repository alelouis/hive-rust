use std::fmt::{Display, Formatter, Result};
use std::ops::Add;
use std::ops::Sub;

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
pub struct Tile {
    q: i8,
    r: i8,
    s: i8,
}

#[macro_export]
macro_rules! tile {
    ($q:expr, $r:expr, $s:expr) => {
        Tile::new($q, $r, $s)
    };
}

pub enum Direction {
    E,
    W,
    NW,
    NE,
    SW,
    SE,
}

const ALL_DIRECTIONS: [Direction; 6] = [
    Direction::E,
    Direction::W,
    Direction::NW,
    Direction::NE,
    Direction::SW,
    Direction::SE,
];

impl Tile {
    pub fn new(q: i8, r: i8, s: i8) -> Self {
        Tile { q, r, s }
    }

    pub fn distance(&self, other: &Self) -> i32 {
        let sub: Self = *self - *other;
        (sub.q.abs() + sub.r.abs() + sub.s.abs()) as i32 / 2
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
