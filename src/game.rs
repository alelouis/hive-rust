use crate::bug::Color;
use crate::player::Player;

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
    turn_number: u32,
    turn_color: Color,
    players: [Player; 2],
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::NotStarted,
            gtype: GameType::Base,
            turn_number: 0,
            turn_color: Color::White,
            players: [Player::new(Color::White), Player::new(Color::Black)],
        }
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
}
