use crate::game::Game;
use crate::r#move::Move;
use std::fmt::format;
use std::str::FromStr;

pub struct Engine {
    version: &'static str,
    game: Option<Game>,
}

const VERSION: &str = "0.1";

impl Engine {
    pub fn new() -> Self {
        Engine {
            version: VERSION,
            game: None,
        }
    }

    fn info(&self) -> String {
        format!("{}", self.version)
    }

    fn new_game(&mut self) -> String {
        self.game = Some(Game::new());
        self.game.as_ref().expect("No game found.").game_string()
    }

    fn play(&mut self, move_str: String) -> String {
        let m = Move::from_str(move_str.as_str()).expect("Couldn't read move");
        self.game.as_mut().unwrap().play_move(m);
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
        format!("{game_string};{moves_string}")
    }

    fn pass(&self) -> String {
        "pass not implemented yet.".to_string()
    }

    fn valid_moves(&self) -> String {
        let mut moves_str = vec![];
        for m in self.game.as_ref().unwrap().compute_valid_moves() {
            moves_str.push(format!("{m}"))
        }
        moves_str.join(";")
    }

    fn best_move(&self) -> String {
        "bestmove not implemented yet.".to_string()
    }

    fn options(&self) -> String {
        "DummyOption;bool;False;False".to_string()
    }

    pub fn process_command(&mut self, command: String) -> Result<String, String> {
        let command_stripped = command.strip_suffix('\n').unwrap().to_string();
        let mut keyword: String;
        let mut args: String;
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
                "play" => Ok(self.play(args)),
                _ => Err("Unknown command.".to_string()),
            }
        } else {
            keyword = command_stripped;
            match keyword.as_str() {
                "info" => Ok(self.info()),
                "newgame" => Ok(self.new_game()),
                "pass" => Err(self.pass()),
                "validmoves" => Ok(self.valid_moves()),
                "bestmove" => Err(self.best_move()),
                "options" => Ok(self.options()),
                _ => Err("Unknown command.".to_string()),
            }
        }
    }
}
