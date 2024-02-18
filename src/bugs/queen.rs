use crate::bugs::bug::Bug;
use crate::logic::tile::{Direction, Tile};
use std::collections::{HashMap, HashSet};

fn is_gate(source_tile: Tile, target_tile: Tile, active_bugs: &HashMap<Tile, Vec<Bug>>) -> bool {
    let delta = target_tile - source_tile;
    let direction = match delta {
        Tile { q: -1, r: 0, s: 1 } => Direction::W,
        Tile { q: 1, r: 0, s: -1 } => Direction::E,
        Tile { q: 0, r: -1, s: 1 } => Direction::NW,
        Tile { q: 1, r: -1, s: 0 } => Direction::NE,
        Tile { q: -1, r: 1, s: 0 } => Direction::SW,
        Tile { q: 0, r: 1, s: -1 } => Direction::SE,
        _ => {
            panic!("Delta is not one step away.")
        }
    };

    let (tile_a, tile_b) = match direction {
        Direction::E => (
            source_tile.move_towards(Direction::NE, 1),
            source_tile.move_towards(Direction::SE, 1),
        ),
        Direction::W => (
            source_tile.move_towards(Direction::NW, 1),
            source_tile.move_towards(Direction::SW, 1),
        ),
        Direction::NW => (
            source_tile.move_towards(Direction::W, 1),
            source_tile.move_towards(Direction::NE, 1),
        ),
        Direction::NE => (
            source_tile.move_towards(Direction::NW, 1),
            source_tile.move_towards(Direction::E, 1),
        ),
        Direction::SW => (
            source_tile.move_towards(Direction::W, 1),
            source_tile.move_towards(Direction::SE, 1),
        ),
        Direction::SE => (
            source_tile.move_towards(Direction::SW, 1),
            source_tile.move_towards(Direction::E, 1),
        ),
    };

    active_bugs.get(&tile_a).is_some() & active_bugs.get(&tile_b).is_some()
}

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

    let candidate_no_gates = neighbors_of_neighbors_set
        .intersection(&free_neighbors_set)
        .filter(|target| !is_gate(tile, **target, active_bugs))
        .cloned()
        .collect();

    candidate_no_gates
}
