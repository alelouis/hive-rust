use crate::hive::Hive;
use crate::tile::{Tile, ALL_DIRECTIONS};
use std::collections::HashSet;

pub fn moves(tile: Tile, hive_without_current_bug: &Hive) -> HashSet<Tile> {
    let mut candidates = HashSet::new();
    for dir in ALL_DIRECTIONS {
        let mut next_tile = tile.clone();
        let mut hopped = 0;
        loop {
            next_tile = next_tile.move_towards(dir, 1);
            if hive_without_current_bug
                .get_bugs()
                .get(&next_tile)
                .is_none()
            {
                break;
            } else {
                hopped += 1
            }
        }
        if hopped > 0 {
            candidates.insert(next_tile);
        }
    }
    candidates
}
