use crate::hive::Hive;
use crate::tile::Tile;
use std::collections::HashSet;

pub mod ant;
pub mod beetle;
pub mod grasshopper;
pub mod queen;
pub mod spider;

pub fn filter_freedom_to_move(tiles: HashSet<Tile>, hive: &Hive) -> HashSet<Tile> {
    let filtered_tiles = tiles
        .iter()
        .cloned()
        .filter(|tile| hive.count_bugs_around(tile) < 5)
        .collect();
    filtered_tiles
}
