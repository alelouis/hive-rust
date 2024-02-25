use crate::logic::bugs::bug::{Bug, Color};
use crate::logic::eval;
use crate::logic::hive::Hive;
use crate::logic::player::Player;
use crate::logic::r#move::Move;
use minimax::{Strategy, Winner};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum GameState {
    NotStarted,
    InProgress,
    Draw,
    WhiteWins,
    BlackWins,
}

#[derive(Debug, Clone)]
enum GameType {
    Base,
}

#[derive(Clone)]
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

    pub fn set_state(&mut self, state: GameState) {
        self.state = state
    }

    pub fn get_hive(&self) -> &Option<Hive> {
        &self.hive
    }

    pub fn update_game_state(&mut self) {
        let white_queen = &Bug::from_str("wQ").expect("Couldn't create bug from string.");
        let black_queen = &Bug::from_str("bQ").expect("Couldn't create bug from string.");
        let in_game_white_queen = self.hive.as_ref().unwrap().find_bug(white_queen);
        let in_game_black_queen = self.hive.as_ref().unwrap().find_bug(black_queen);

        if let Some(tile) = in_game_white_queen {
            if self.hive.as_ref().unwrap().get_nearby_bugs(tile).len() == 6 {
                self.state = GameState::BlackWins;
            }
        } else if let Some(tile) = in_game_black_queen {
            if self.hive.as_ref().unwrap().get_nearby_bugs(tile).len() == 6 {
                self.state = GameState::BlackWins
            }
        } else {
            self.state = GameState::InProgress
        }
    }

    pub fn play_move(&mut self, m: Move) {
        self.hive
            .as_mut()
            .expect("Couldn't find hive.")
            .play_move(m);

        let player = match self.turn_color {
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

    pub fn get_best_move(&self) -> Move {
        let start = self;
        let mut strategy = minimax::Negamax::new(eval::Eval, 3);
        let best_move = strategy.choose_move(&start).unwrap();
        best_move
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
        current_player.valid_moves(
            &self.hive.as_ref().expect("Couldn't get hive."),
            self.turn_number,
            self.turn_color,
        )
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

impl minimax::Game for Game {
    type S = Game;
    type M = Move;

    fn generate_moves(state: &Game, moves: &mut Vec<Self::M>) {
        let current_player = state.get_current_player();
        let valid_moves = current_player.valid_moves(
            &state.hive.as_ref().expect("Couldn't get hive."),
            state.turn_number,
            state.turn_color,
        );
        for m in valid_moves {
            moves.push(m);
        }
    }

    fn apply(state: &mut Self::S, m: Self::M) -> Option<Self::S> {
        state.play_move(m);
        Some(state.clone())
    }

    fn get_winner(state: &Self::S) -> Option<Winner> {
        match state.state {
            GameState::WhiteWins => Some(Winner::PlayerJustMoved),
            GameState::BlackWins => Some(Winner::PlayerJustMoved),
            _ => None,
        }
    }
}
