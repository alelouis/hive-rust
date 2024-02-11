use crate::bug::{Bug, BugKind, Color};
use crate::hive::Hive;
use crate::r#move::Move;
use crate::tile::{Direction, Tile};
use std::str::FromStr;

pub struct Player {
    color: Color,
    inactive_pieces: Vec<Bug>,
    active_pieces: Vec<Bug>,
}

const PIECE_SET: [&str; 12] = [
    "Q", "S1", "S2", "B1", "B2", "B3", "G1", "G2", "G3", "A1", "A2", "A3",
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
            color,
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

    pub fn valid_moves(&self, hive: &Hive, turn_number: u32, turn_color: Color) -> Vec<Move> {
        let mut moves = vec![];
        let mut added_spider = 0;
        let mut added_beetle = 0;
        let mut added_grasshopper = 0;
        let mut added_ant = 0;

        // Placing
        // First round: Placed at 0, 0, 0
        // Second round: Placed around first piece
        // Else: Placed only with similar color

        for piece in &self.inactive_pieces {
            let mut candidates: Vec<(Option<Bug>, Option<Direction>)> = vec![];
            candidates = match hive.get_n_tiles() {
                0 => {
                    // Place on tile (0, 0, 0)
                    vec![(None, None)]},
                1 => {
                    // Can place anywhere around first piece
                    let mut c = vec![];
                    for neigh_tile in Tile::new(0, 0, 0).neighbors() {
                        let nearby = hive.get_nearby_bugs(neigh_tile);
                        let (bug, dir) = nearby.first().expect("Couldn't find bugs neighbors");
                        c.push((Some(bug.clone()), Some(dir.clone())));
                    }
                    c
                }
                _ => {
                    // Place only on neighbors of same color

                    vec![(None, None)]
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
}
