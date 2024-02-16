use crate::hive::Hive;
use crate::tile::Tile;
use std::collections::HashSet;

pub fn moves(tile: Tile, hive_without_current_bug: &Hive) -> HashSet<Tile> {
    let neighbors = tile.neighbors();
    let slide_neighbors: Vec<Tile> = neighbors
        .iter()
        .cloned()
        .filter(|tile| hive_without_current_bug.get_nearby_bugs(*tile).len() > 0)
        .collect();
    HashSet::from_iter(slide_neighbors.iter().cloned())
}
