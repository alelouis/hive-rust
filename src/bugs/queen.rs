use crate::bug::Bug;
use crate::tile::Tile;
use std::collections::{HashMap, HashSet};

pub fn moves(tile: Tile, active_bugs: &HashMap<Tile, Vec<Bug>>) -> HashSet<Tile> {
    // Free neighbors of queen
    let neighbors = tile.neighbors();

    let free_neighbors_vec: Vec<Tile> = neighbors
        .iter()
        .filter(|tile| active_bugs.get(tile).is_none())
        .cloned()
        .collect();

    let occupied_neighbors: Vec<Tile> = neighbors
        .iter()
        .filter(|tile| active_bugs.get(tile).is_some())
        .cloned()
        .collect();

    let free_neighbors_set = HashSet::from_iter(free_neighbors_vec.iter().cloned());
    let mut neighbors_of_neighbors_set = HashSet::new();
    for neigh in occupied_neighbors {
        let neighbors = neigh.neighbors();
        for neigh in neighbors {
            neighbors_of_neighbors_set.insert(neigh);
        }
    }

    let candidates = neighbors_of_neighbors_set
        .intersection(&free_neighbors_set)
        .cloned()
        .collect();

    candidates
}
