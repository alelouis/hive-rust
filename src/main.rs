mod bug;
mod hive;
mod tile;

use crate::bug::{Bug, BugKind, Color};
use crate::hive::Hive;
use crate::tile::Tile;
use std::str::FromStr;

fn main() {
    let mut hive = Hive::new();
    let tile = tile!(1, 0, 0);
    let bug = Bug::from_str("wQ").expect("Couldn't parse bug");
    hive.add_bug(tile, bug);
    println!("{hive}");
}
