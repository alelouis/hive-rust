use crate::logic::bugs;
use crate::logic::bugs::bug::{Bug, BugKind, Color};
use crate::logic::hive::Hive;
use crate::logic::r#move::Move;
use crate::logic::tile::{Direction, Tile};
use std::collections::HashSet;
use std::ops::Not;
use std::str::FromStr;

#[derive(Clone)]
pub struct Player {
    inactive_pieces: Vec<Bug>,
    active_pieces: Vec<Bug>,
}

const PIECE_SET: [&str; 11] = [
    "Q", "S1", "S2", "B1", "B2", "G1", "G2", "G3", "A1", "A2", "A3",
];

impl Player {
    pub fn new(color: Color) -> Self {
        let mut inactive_pieces = vec![];
        let color_str = if color == Color::White { 'w' } else { 'b' };
        for piece in PIECE_SET {
            let bug = Bug::from_str(format!("{color_str}{piece}").as_str())
                .expect("Couldn't create bug from string.");
            inactive_pieces.push(bug)
        }
        Player {
            inactive_pieces,
            active_pieces: vec![],
        }
    }

    pub fn set_piece_active(&mut self, bug: Bug) {
        self.inactive_pieces.retain(|&x| x != bug);
        self.active_pieces.push(bug);
    }

    pub fn is_piece_inactive(&self, bug: Bug) -> bool {
        self.inactive_pieces.contains(&bug)
    }

    pub fn find_bugs_dir_from_tiles(
        &self,
        hive: &Hive,
        tiles: HashSet<Tile>,
    ) -> Vec<(Option<Bug>, Option<Direction>)> {
        // Manage beetle
        let mut c = vec![];
        for tile in tiles {
            if let Some(t) = hive.get_bugs().get(&tile) {
                c.push((
                    Some(t.last().expect("Couldn't get last bug of tile").clone()),
                    None,
                ));
            } else {
                let nearby = hive.get_nearby_bugs(tile);
                // Duplicate moves
                // for (bug, dir) in nearby {
                //     c.push((Some(bug.clone()), Some(dir.clone())));
                // }
                let (b, d) = nearby.first().unwrap();
                c.push((Some(b.clone()), Some(d.clone())));
            }
        }
        c
    }

    // Takes as input a set of tiles and output tiles which have only neighbors of turn_color.
    pub fn filter_tiles_by_neighbors_color(
        &self,
        hive: &Hive,
        tiles: HashSet<Tile>,
        turn_color: Color,
    ) -> HashSet<Tile> {
        let opposite_color = if turn_color == Color::Black {
            Color::White
        } else {
            Color::Black
        };
        let mut same_color_tiles: HashSet<Tile> = Default::default();
        for tile in tiles.iter() {
            let mut neigh_colors = vec![];
            for neigh_tile in tile.neighbors() {
                if let Some(bugs) = hive.get_bugs_on_tile(neigh_tile) {
                    let tile_color = bugs.last().expect("Couldn't get last bug of tile").color;
                    neigh_colors.push(tile_color);
                }
            }
            if neigh_colors
                .iter()
                .any(|color| color == &opposite_color)
                .not()
            {
                same_color_tiles.insert(*tile);
            }
        }
        same_color_tiles
    }

    pub fn is_queen_played(&self, turn_color: Color) -> bool {
        self.active_pieces.contains(&match turn_color {
            Color::Black => Bug::from_str("bQ").unwrap(),
            Color::White => Bug::from_str("wQ").unwrap(),
        })
    }

    pub fn is_forced_queen_play(&self, turn_color: Color, turn_color_number: u32) -> bool {
        let mut forced = false;
        match turn_color {
            Color::Black => {
                let black_queen = Bug::from_str("bQ").unwrap();
                if !self.active_pieces.contains(&black_queen) & (turn_color_number == 3) {
                    forced = true
                }
            }
            Color::White => {
                let white_queen = Bug::from_str("wQ").unwrap();
                if !self.active_pieces.contains(&white_queen) & (turn_color_number == 3) {
                    forced = true;
                }
            }
        }
        forced
    }

    pub fn placing(&self, hive: &Hive, turn_color: Color, turn_color_number: u32) -> Vec<Move> {
        let mut moves = vec![];
        let mut added_spider = 0;
        let mut added_beetle = 0;
        let mut added_grasshopper = 0;
        let mut added_ant = 0;

        let active_bugs = hive.get_bugs().keys();
        let tiles_with_bugs: HashSet<&Tile> = active_bugs.collect::<HashSet<&Tile>>();
        let mut neighbors_tiles_of_bugs: HashSet<Tile> = Default::default();
        for tile in tiles_with_bugs {
            for neigh in tile.neighbors() {
                if hive.get_bugs_on_tile(neigh).is_none() {
                    neighbors_tiles_of_bugs.insert(neigh);
                }
            }
        }

        // Force queen play if fourth turn
        let mut check_pieces = if self.is_forced_queen_play(turn_color, turn_color_number) {
            match turn_color {
                Color::Black => vec![Bug::from_str("bQ").unwrap()],
                Color::White => vec![Bug::from_str("wQ").unwrap()],
            }
        } else {
            self.inactive_pieces.clone()
        };

        // Disable queen on first turns
        if turn_color_number == 0 {
            check_pieces.retain(|b| b.kind != BugKind::Queen)
        }

        for piece in &check_pieces {
            let candidates: Vec<(Option<Bug>, Option<Direction>)>;
            candidates = match hive.get_n_tiles() {
                0 => {
                    // Place on tile (0, 0, 0)
                    vec![(None, None)]
                }
                1 => {
                    // Can place anywhere around first piece
                    let mut tiles: HashSet<Tile> = Default::default();
                    for tile in Tile::new(0, 0, 0).neighbors() {
                        tiles.insert(tile);
                    }
                    self.find_bugs_dir_from_tiles(hive, tiles)
                }
                _ => {
                    // Place only on neighbors of same color
                    let same_color_tiles = self.filter_tiles_by_neighbors_color(
                        hive,
                        neighbors_tiles_of_bugs.clone(),
                        turn_color,
                    );
                    self.find_bugs_dir_from_tiles(hive, same_color_tiles)
                }
            };
            for (other, direction) in candidates {
                let m = Move::new(*piece, other, direction);
                match piece.kind {
                    BugKind::Spider => {
                        if added_spider == 0 {
                            added_spider = piece.index
                        }
                        if piece.index == added_spider {
                            moves.push(m);
                        }
                    }
                    BugKind::Beetle => {
                        if added_beetle == 0 {
                            added_beetle = piece.index
                        }
                        if piece.index == added_beetle {
                            moves.push(m);
                        }
                    }
                    BugKind::Grasshopper => {
                        if added_grasshopper == 0 {
                            added_grasshopper = piece.index
                        }
                        if piece.index == added_grasshopper {
                            moves.push(m);
                        }
                    }
                    BugKind::Ant => {
                        if added_ant == 0 {
                            added_ant = piece.index
                        }
                        if piece.index == added_ant {
                            moves.push(m);
                        }
                    }
                    _ => moves.push(m),
                };
            }
        }
        moves
    }

    pub fn movement(&self, hive: &Hive) -> Vec<Move> {
        let mut moves = vec![];

        let active_bugs = hive.get_bugs();
        let mut hive_without_current_bug: Hive = hive.clone();

        for bug in &self.active_pieces {
            let tile = hive
                .find_bug(&bug)
                .expect("Couldn't find tile of active bug.");
            hive_without_current_bug.remove_bug(*bug);

            let bug_z_index = hive
                .get_bugs_on_tile(tile)
                .unwrap()
                .iter()
                .position(|&b| b == *bug)
                .unwrap();
            let is_under_other_bug = (bug_z_index + 1) < hive.get_bugs_on_tile(tile).unwrap().len();

            if hive_without_current_bug.is_connected() & !is_under_other_bug {
                let candidate_tiles = match bug.kind {
                    BugKind::Queen => bugs::queen::moves(tile, active_bugs),
                    BugKind::Beetle => bugs::beetle::moves(tile, &hive_without_current_bug),
                    BugKind::Grasshopper => {
                        bugs::grasshopper::moves(tile, &hive_without_current_bug)
                    }
                    BugKind::Spider => {
                        bugs::spider::moves(tile, hive_without_current_bug.get_bugs())
                    }
                    BugKind::Ant => bugs::ant::moves(tile, active_bugs),
                };
                let bug_dir = self.find_bugs_dir_from_tiles(hive, candidate_tiles);
                let mut current_moves: Vec<Move> = bug_dir
                    .iter()
                    .cloned()
                    //.filter(|(other, _)| *bug != other.unwrap())
                    .map(|(other, dir)| Move::new(*bug, other, dir))
                    .collect();
                moves.append(&mut current_moves)
            }
            hive_without_current_bug.add_bug(tile, *bug);
        }
        moves
    }

    pub fn valid_moves(&self, hive: &Hive, turn_number: u32, turn_color: Color) -> Vec<Move> {
        let mut moves = vec![];

        // Placing
        let turn_color_number = match turn_color {
            Color::White => turn_number / 2,
            Color::Black => (turn_number - 1) / 2,
        };

        let mut placing_moves = self.placing(hive, turn_color, turn_color_number);
        moves.append(&mut placing_moves);

        // Movement
        if self.is_queen_played(turn_color) {
            let mut motion_move = self.movement(hive);
            moves.append(&mut motion_move);
        }

        moves
    }
}
