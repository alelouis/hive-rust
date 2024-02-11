use crate::bug::Color;
use crate::hive::Hive;
use crate::player::Player;
use crate::r#move::Move;
use log::{debug, info};
use std::fmt::format;

#[derive(Debug)]
enum GameState {
    NotStarted,
    InProgress,
    Draw,
    WhiteWins,
    BlackWins,
}

#[derive(Debug)]
enum GameType {
    Base,
}

pub struct Game {
    state: GameState,
    gtype: GameType,
    pub(crate) turn_number: u32,
    pub(crate) turn_color: Color,
    players: [Player; 2],
    hive: Option<Hive>,
    moves_history: Vec<Move>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::NotStarted,
            gtype: GameType::Base,
            turn_number: 0,
            turn_color: Color::White,
            players: [Player::new(Color::White), Player::new(Color::Black)],
            hive: Some(Hive::new()),
            moves_history: vec![],
        }
    }

    pub fn play_move(&mut self, m: Move) {
        self.hive
            .as_mut()
            .expect("Couldn't find hive.")
            .play_move(m);

        let mut player = match self.turn_color {
            Color::White => self.players.get_mut(0),
            Color::Black => self.players.get_mut(1),
        }
        .expect("Couldn't get active player");

        if player.is_piece_inactive(m.source) {
            player.set_piece_active(m.source);
        }

        self.turn_number += 1;
        self.turn_color = if self.turn_color == Color::White {
            Color::Black
        } else {
            Color::White
        };
        self.moves_history.push(m);
    }

    pub fn turn_string(&self) -> String {
        let color = if self.turn_color == Color::White {
            "White"
        } else {
            "Black"
        };
        format!("{color}[{}]", self.turn_number)
    }

    pub fn game_string(&self) -> String {
        // GameTypeString;GameStateString;TurnString
        format!("{:?};{:?};{}", self.gtype, self.state, self.turn_string())
    }

    pub fn moves_string(&self) -> String {
        let mut moves_string = vec![];
        for m in &self.moves_history {
            moves_string.push(format!("{m}"));
        }
        moves_string.join(";")
    }

    pub fn compute_valid_moves(&self) -> Vec<Move> {
        let current_player = self.get_current_player();
        current_player.valid_moves(&self.hive.as_ref().expect("Couldn't get hive."))
    }

    pub fn get_current_player(&self) -> &Player {
        let current_player = match self.turn_color {
            Color::White => self.players.get(0),
            Color::Black => self.players.get(1),
        }
        .expect("Couldn't get player.");
        current_player
    }
}
