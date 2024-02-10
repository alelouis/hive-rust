use crate::game::Game;

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

    fn play(&self) -> String {
        "play not implemented yet.".to_string()
    }

    fn pass(&self) -> String {
        "pass not implemented yet.".to_string()
    }

    fn valid_moves(&self) -> String {
        "validmoves not implemented yet.".to_string()
    }

    fn best_move(&self) -> String {
        "bestmove not implemented yet.".to_string()
    }

    fn options(&self) -> String {
        "DummyOption;bool;False;False".to_string()
    }

    pub fn process_command(&mut self, command: String) -> Result<String, String> {
        match command.strip_suffix('\n').unwrap() {
            "info" => Ok(self.info()),
            "newgame" => Ok(self.new_game()),
            "play" => Err(self.play()),
            "pass" => Err(self.pass()),
            "validmoves" => Err(self.valid_moves()),
            "bestmove" => Err(self.best_move()),
            "options" => Ok(self.options()),
            _ => Err("Unknown command.".to_string()),
        }
    }
}
