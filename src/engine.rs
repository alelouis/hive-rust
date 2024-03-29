use crate::logic::game::{Game, GameState};
use crate::logic::r#move::Move;
use log::{error, info, LevelFilter};
use std::str::FromStr;

pub struct Engine {
    version: &'static str,
    pub game: Option<Game>,
}

const VERSION: &str = "0.1";

impl Engine {
    pub fn new() -> Self {
        simple_logging::log_to_file("test.log", LevelFilter::Error)
            .expect("Couldn't initialize logger");
        Engine {
            version: VERSION,
            game: None,
        }
    }

    fn info(&self) -> String {
        format!("{}", self.version)
    }

    pub fn new_game(&mut self) -> String {
        info!("starting new game");
        let mut game = Game::new();
        game.set_state(GameState::InProgress);
        self.game = Some(game);
        info!("turn number: {}", self.game.as_ref().unwrap().turn_number);
        info!("turn color: {:?}", self.game.as_ref().unwrap().turn_color);
        self.game.as_ref().expect("No game found.").game_string()
    }

    pub fn play(&mut self, move_str: String) -> Result<String, String> {
        info!("new move requested: {move_str}");
        let m = Move::from_str(move_str.as_str()).expect("Couldn't read move");
        info!("{m}");
        let valid_moves = self.game.as_mut().unwrap().compute_valid_moves();
        if valid_moves.contains(&m) {
            self.game.as_mut().unwrap().play_move(m);
            self.game.as_mut().unwrap().update_game_state();

            let game_string = self
                .game
                .as_ref()
                .expect("Couldn't find game.")
                .game_string();
            let moves_string = self
                .game
                .as_ref()
                .expect("Couldn't find game.")
                .moves_string();
            info!("move {move_str} played");
            info!("turn number: {}", self.game.as_ref().unwrap().turn_number);
            info!("turn color: {:?}", self.game.as_ref().unwrap().turn_color);
            Ok(format!("{game_string};{moves_string}"))
        } else {
            error!("{}", format!("invalid move {move_str}"));
            Err(format!("invalid move {move_str}"))
        }
    }

    pub fn pass(&self) -> String {
        "pass not implemented yet.".to_string()
    }

    pub fn valid_moves(&mut self) -> String {
        info!("requesting valid moves");
        let mut moves_str = vec![];
        for m in self.game.as_mut().unwrap().compute_valid_moves() {
            moves_str.push(format!("{m}"))
        }
        moves_str.join(";")
    }

    pub fn best_move(&self) -> String {
        let best_move = self.game.as_ref().unwrap().get_best_move();
        if let Some(m) = best_move {
            format!("{m}")
        } else {
            "".to_string()
        }
    }

    pub fn options(&self) -> String {
        "DummyOption;bool;False;False".to_string()
    }

    pub fn process_command(&mut self, command: String) -> Result<String, String> {
        let command_stripped = command.strip_suffix('\n').unwrap().to_string();
        let keyword: String;
        let args: String;
        info!("new command received: {command_stripped}");
        if command.contains(' ') {
            let split_index = command_stripped.find(' ').unwrap();
            (keyword, args) = (
                command.get(0..split_index).unwrap().to_string(),
                command
                    .get(split_index + 1..)
                    .unwrap()
                    .strip_suffix("\n")
                    .unwrap()
                    .to_string(),
            );
            match keyword.as_str() {
                "play" => self.play(args),
                "newgame" => Ok(self.new_game()),
                _ => {
                    error!("Unknown command!");
                    Err("Unknown command.".to_string())
                }
            }
        } else {
            keyword = command_stripped;
            match keyword.as_str() {
                "info" => Ok(self.info()),
                "newgame" => Ok(self.new_game()),
                "pass" => Err(self.pass()),
                "validmoves" => Ok(self.valid_moves()),
                "bestmove" => Ok(self.best_move()),
                "options" => Ok(self.options()),
                _ => {
                    error!("Unknown command!");
                    Err("Unknown command.".to_string())
                }
            }
        }
    }
}
