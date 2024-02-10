use crate::bug::{Bug, Color};
use crate::hive::Hive;
use crate::r#move::Move;
use std::fmt::format;
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

    pub fn valid_moves(&self, hive: &Hive) -> Vec<Move> {
        let mut moves = vec![];
        for piece in &self.inactive_pieces {
            let m = Move::new(*piece, None, None);
            moves.push(m)
        }
        moves
    }
}
