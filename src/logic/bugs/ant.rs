use crate::logic::bugs::bug::Bug;
use crate::logic::tile::Tile;
use std::collections::{HashMap, HashSet};

use crate::logic::bugs;
use pathfinding::prelude::bfs_reach;

fn successors(tile: &Tile, active_bugs: &HashMap<Tile, Vec<Bug>>) -> Vec<Tile> {
    bugs::queen::moves(*tile, active_bugs)
        .iter()
        .cloned()
        .collect()
}

pub fn moves(tile: Tile, active_bugs: &HashMap<Tile, Vec<Bug>>) -> HashSet<Tile> {
    let mut candidates =
        bfs_reach(tile, |t| successors(&t, active_bugs)).collect::<HashSet<Tile>>();
    candidates.remove(&tile);

    let mut connected_moves = vec![];
    for m in candidates.iter() {
        let occupied_neighbors: Vec<Tile> = m
            .neighbors()
            .iter()
            .filter(|t| active_bugs.get(t).is_some())
            .cloned()
            .collect();

        if occupied_neighbors.len() == 1 {
            if *occupied_neighbors.first().unwrap() != tile {
                connected_moves.push(*m);
            }
        } else {
            connected_moves.push(*m);
        }
    }
    connected_moves.iter().cloned().collect()
}
