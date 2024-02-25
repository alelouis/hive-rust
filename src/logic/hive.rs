use crate::logic::bugs::bug::{Bug, Color};
use crate::logic::r#move::Move;
use crate::logic::tile::{Direction, Tile, REVERSE_DIRECTION};
use log::debug;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Display, Formatter};
use std::ops::Not;

#[derive(Clone)]
pub struct Hive {
    bugs: HashMap<Tile, Vec<Bug>>,
    turn: Color,
}

impl Hive {
    pub fn new() -> Self {
        Hive {
            bugs: HashMap::new(),
            turn: Color::White,
        }
    }

    pub fn get_n_tiles(&self) -> usize {
        self.bugs.len()
    }

    pub fn get_bugs(&self) -> &HashMap<Tile, Vec<Bug>> {
        &self.bugs
    }

    pub fn get_nearby_bugs(&self, tile: Tile) -> Vec<(Bug, Direction)> {
        let mut bugs_directions: Vec<(Bug, Direction)> = vec![];
        let tile_neighbors = tile.neighbors();
        for (t, direction) in tile_neighbors.iter().zip(REVERSE_DIRECTION) {
            match self.bugs.get(&t) {
                Some(bugs_on_tile) => {
                    for bug in bugs_on_tile {
                        bugs_directions.push((*bug, direction))
                    }
                }
                None => {}
            };
        }
        bugs_directions
    }

    pub fn is_surrounded(&self, tile: Tile) -> bool {
        let mut n_surround = 0;
        let tile_neighbors = tile.neighbors();
        for t in tile_neighbors {
            if self.bugs.get(&t).is_some() {
                n_surround += 1
            }
        }
        n_surround == 6
    }

    // Play a given move
    pub fn play_move(&mut self, m: Move) {
        if m.is_first_piece() {
            let tile = Tile::new(0, 0, 0);
            self.add_bug(tile, m.source);
        } else {
            let target = m.target.expect("Couldn't find target.");
            self.place_bug_relative(m.source, target, m.direction)
        }
    }

    // Add a bug to the hive at specified tile
    pub fn add_bug(&mut self, tile: Tile, bug: Bug) {
        let bugs = self.bugs.get_mut(&tile);
        if let Some(vec) = bugs {
            debug!("stacking {bug} on tile {tile}");
            vec.push(bug);
        } else {
            debug!("placing {bug} on tile {tile}");
            self.bugs.insert(tile, vec![bug]);
        }
    }

    // Removes a bug from the hive
    pub fn remove_bug(&mut self, bug: Bug) {
        let tile = self.find_bug(&bug).expect("Couldn't find bug.");
        let bugs = self.bugs.get_mut(&tile);
        debug!("removing {bug} from tile {tile}");
        if let Some(vec) = bugs {
            if vec.len() > 1 {
                vec.retain(|&x| x != bug);
            } else {
                self.bugs.remove(&tile);
            }
        }
    }

    // Returns the tile a bug is on
    pub fn find_bug(&self, bug: &Bug) -> Option<Tile> {
        self.bugs.iter().find_map(|(key, &ref val)| {
            if val.contains(bug) {
                Some(key.clone())
            } else {
                None
            }
        })
    }

    pub fn get_bugs_on_tile(&self, tile: Tile) -> Option<Vec<Bug>> {
        self.bugs.get(&tile).cloned()
    }

    pub fn count_bugs_of_color(&self, color: Color) -> i32 {
        let mut score = 0;
        for (_, bugs) in &self.bugs {
            for bug in bugs {
                if bug.color == color {
                    score += 1;
                }
            }
        }
        score
    }

    // Place an other bug relative to a bug in a given direction
    pub fn place_bug_relative(&mut self, other: Bug, bug: Bug, direction: Option<Direction>) {
        let source_tile = self
            .find_bug(&bug)
            .expect("Couldn't find target bug in relative placement.");

        let target_tile = match direction {
            Some(d) => source_tile.move_towards(d, 1),
            None => source_tile.clone(),
        };

        // If already on the board, delete if from previous tile
        if self.find_bug(&other).is_some() {
            self.remove_bug(other);
        }

        // Add to new tile
        self.add_bug(target_tile, other);
    }

    pub fn is_connected(&self) -> bool {
        let start = self.bugs.keys().next().expect("Hive has so tile");
        let mut stack: VecDeque<Tile> = VecDeque::new();
        let mut visited: HashSet<Tile> = HashSet::new();
        stack.push_back(*start);

        while stack.len() > 0 {
            let node = stack.pop_back().expect("Empty stack");
            visited.insert(node);
            let neighbors = node.neighbors();
            let occupied_neighbors = neighbors
                .iter()
                .filter(|tile| self.bugs.get(tile).is_some());
            for neigh in occupied_neighbors {
                if visited.contains(&neigh).not() {
                    stack.push_back(*neigh)
                }
            }
        }

        visited.len() == self.bugs.len()
    }
}

impl Display for Hive {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut description = "".to_string();
        for (tile, bug) in &self.bugs {
            description = description + &format!("{tile}: {:?}\n", bug);
        }
        write!(f, "{description}")
    }
}
