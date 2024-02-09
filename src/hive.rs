use crate::bug::{Bug, Color};
use crate::tile::Tile;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct Hive {
    bugs: HashMap<Tile, Bug>,
    turn: Color,
}

impl Hive {
    pub fn new() -> Self {
        Hive {
            bugs: HashMap::new(),
            turn: Color::White,
        }
    }

    pub fn add_bug(&mut self, tile: Tile, bug: Bug) {
        self.bugs.insert(tile, bug);
    }
}

impl Display for Hive {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut description = "".to_string();
        for (tile, bug) in &self.bugs {
            description = description + &format!("{tile}: {bug}\n");
        }
        write!(f, "{description}")
    }
}
