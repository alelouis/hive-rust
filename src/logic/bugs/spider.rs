use crate::logic::bugs;
use crate::logic::bugs::bug::Bug;
use crate::logic::tile::Tile;
use std::collections::{HashMap, HashSet};

pub fn moves(tile: Tile, active_bugs: &HashMap<Tile, Vec<Bug>>) -> HashSet<Tile> {
    let mut tiles = HashSet::new();
    let mut iteration = HashSet::new();
    let mut visited = HashSet::new();
    visited.insert(tile);
    tiles.insert(tile);
    for idx in 0..3 {
        iteration = HashSet::new();
        for t in &tiles {
            let queen_moves = bugs::queen::moves(*t, active_bugs);
            for st in queen_moves {
                if visited.get(&st).is_none() {
                    iteration.insert(st);
                }
                if idx < 2 {
                    visited.insert(st);
                }
            }
        }
        tiles = iteration.clone();
    }
    let no_backtrack = iteration.difference(&visited).cloned().collect();
    no_backtrack
}
