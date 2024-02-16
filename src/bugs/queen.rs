use crate::bug::Bug;
use crate::hive::Hive;
use crate::tile::Tile;
use std::collections::{HashMap, HashSet};

pub fn moves(
    tile: Tile,
    active_bugs: &HashMap<Tile, Vec<Bug>>,
    hive_without_current_bug: &Hive,
) -> HashSet<Tile> {
    let neighbors = tile.neighbors();
    let free_neighbors: Vec<Tile> = neighbors
        .iter()
        .filter(|tile| active_bugs.get(tile).is_none())
        .cloned()
        .collect();
    let slide_neighbors: Vec<Tile> = free_neighbors
        .iter()
        .cloned()
        .filter(|tile| hive_without_current_bug.get_nearby_bugs(*tile).len() > 0)
        .collect();
    HashSet::from_iter(slide_neighbors.iter().cloned())
}
