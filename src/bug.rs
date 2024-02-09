use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub enum Color {
    Black,
    White,
}

#[derive(Debug)]
pub enum BugKind {
    Queen,
    Beetle,
    Ant,
    Spider,
    Grasshopper,
}

pub struct Bug {
    kind: BugKind,
    index: u8,
    color: Color,
}

impl Bug {
    pub fn new(kind: BugKind, index: u8, color: Color) -> Self {
        Self { kind, index, color }
    }
}

impl Display for Bug {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBugError;

impl FromStr for Bug {
    type Err = ParseBugError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut index: Result<u8, ParseBugError> = Ok(0);

        let color = match s.chars().nth(0) {
            Some('w') => Ok(Color::White),
            Some('b') => Ok(Color::Black),
            _ => Err(ParseBugError),
        };

        let kind = match s.chars().nth(1) {
            Some('Q') => Ok(BugKind::Queen),
            Some('S') => Ok(BugKind::Spider),
            Some('B') => Ok(BugKind::Beetle),
            Some('G') => Ok(BugKind::Grasshopper),
            Some('A') => Ok(BugKind::Ant),
            _ => Err(ParseBugError),
        };

        if let Some(i) = s.chars().nth(2) {
            index = match i.to_digit(10) {
                Some(i) => Ok(i as u8),
                None => Err(ParseBugError),
            }
        }

        Ok(Bug::new(
            kind.expect("Couldn't parse kind."),
            index.expect("Couldn't parse index."),
            color.expect("Couldn't parse color."),
        ))
    }
}
