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
    candidates
}
