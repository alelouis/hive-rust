use crate::bug::{Bug, BugKind, Color};
use crate::hive::Hive;
use crate::r#move::Move;
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

    pub fn valid_moves(&self, hive: &Hive) -> Vec<Move> {
        let mut moves = vec![];
        let mut added_spider = false;
        let mut added_beetle = false;
        let mut added_grasshopper = false;
        let mut added_ant = false;
        for piece in &self.inactive_pieces {
            let m = Move::new(*piece, None, None);
            match piece.kind {
                BugKind::Spider => {
                    if !added_spider {
                        moves.push(m);
                        added_spider = true
                    }
                }
                BugKind::Beetle => {
                    if !added_beetle {
                        moves.push(m);
                        added_beetle = true
                    }
                }
                BugKind::Grasshopper => {
                    if !added_grasshopper {
                        moves.push(m);
                        added_grasshopper = true
                    }
                }
                BugKind::Ant => {
                    if !added_ant {
                        moves.push(m);
                        added_ant = true;
                    }
                }
                _ => moves.push(m),
            };
        }
        moves
    }
}
