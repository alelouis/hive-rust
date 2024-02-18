use crate::bug::{Bug, Color};
use crate::r#move::Move;
use crate::tile::{Direction, Tile, REVERSE_DIRECTION};
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

    // Play a given move
    pub fn play_move(&mut self, m: Move) {
        if m.is_first_piece() {
            let tile = Tile::new(0, 0, 0);
            self.add_bug(tile, m.source);
        } else {
            let target = m.target.expect("Couldn't find target.");
            self.place_bug_relative(m.source, target, m.direction)
        }
        self.turn += 1
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

#[cfg(test)]
mod tests {
    use crate::bug::Bug;
    use crate::hive::Hive;
    use crate::tile::{Direction, Tile, ALL_DIRECTIONS};
    use std::str::FromStr;

    #[test]
    fn add_bug() {
        let mut hive = Hive::new();
        let bug = Bug::from_str("wQ").expect("Couldn't parse bug");
        let tile = Tile::new(0, 0, 0);
        hive.add_bug(tile, bug);
        assert_eq!(hive.bugs.len(), 1);
    }

    #[test]
    fn stacked_bugs() {
        let mut hive = Hive::new();
        let queen = Bug::from_str("wQ").expect("Couldn't parse bug");
        let beetle = Bug::from_str("wB").expect("Couldn't parse bug");
        let tile = Tile::new(0, 0, 0);
        hive.add_bug(tile, queen);
        hive.add_bug(tile, beetle);
        assert_eq!(hive.bugs.get(&tile).expect("Couldn't get tile").len(), 2);
        hive.remove_bug(beetle);
        assert_eq!(hive.bugs.get(&tile).expect("Couldn't get tile").len(), 1);
        hive.remove_bug(queen);
        assert_eq!(hive.bugs.len(), 0);
    }

    #[test]
    fn place_relative_bug() {
        let mut hive = Hive::new();
        let bug_0 = Bug::from_str("wQ").expect("Couldn't parse bug");
        let bug_1 = Bug::from_str("wS1").expect("Couldn't parse bug");
        let tile = Tile::new(0, 0, 0);
        hive.add_bug(tile, bug_0);
        for direction in ALL_DIRECTIONS {
            hive.place_bug_relative(bug_1, bug_0, Some(direction));
            let tile_bug_1 = hive.find_bug(&bug_1).expect("Couldn't find bug");
            assert_eq!(tile_bug_1, tile.move_towards(direction, 1));
            hive.remove_bug(bug_1);
        }
    }

    #[test]
    fn moving_bug() {
        let mut hive = Hive::new();
        let queen = Bug::from_str("wQ").expect("Couldn't parse bug");
        let ant = Bug::from_str("wA1").expect("Couldn't parse bug");
        let tile = Tile::new(0, 0, 0);
        hive.add_bug(tile, queen);
        hive.place_bug_relative(ant, queen, Some(Direction::E));
        let first_tile = Tile::new(1, 0, -1);
        assert_eq!(hive.find_bug(&ant).expect("Couldn't find ant"), first_tile);
        hive.place_bug_relative(ant, queen, Some(Direction::W));
        let second_tile = Tile::new(-1, 0, 1);
        assert_eq!(hive.bugs.get(&first_tile).is_none(), true);
        assert_eq!(hive.find_bug(&ant).expect("Couldn't find ant"), second_tile);
    }
}
