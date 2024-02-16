mod bug;
mod bugs;
mod engine;
mod game;
mod hive;
mod r#move;
mod player;
mod tile;

use crate::engine::Engine;
use std::io;

fn main() {
    let mut engine = Engine::new();

    let response = engine.process_command("info\n".to_string());

    match response {
        Ok(r) => {
            println!("{r}");
        }
        Err(e) => {
            println!("err {e}")
        }
    }
    println!("ok");

    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read command.");
        let response = engine.process_command(command);
        match response {
            Ok(r) => {
                println!("{r}");
            }
            Err(e) => {
                println!("err {e}")
            }
        }
        println!("ok")
    }
}
