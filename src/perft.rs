mod engine;
mod logic;

use crate::engine::Engine;
use crate::logic::game::Game;
use std::time::Instant;

fn count_moves_depth(g: Game, depth: u32) -> usize {
    if depth == 0 {
        g.compute_valid_moves().len()
    } else {
        let mut total_moves = 0;
        for m in g.compute_valid_moves() {
            let mut new_e = g.clone();
            new_e.play_move(m);
            total_moves += count_moves_depth(new_e, depth - 1);
        }
        total_moves
    }
}

fn main() {
    let mut e = Engine::new();
    e.new_game();
    let g = e.game.unwrap();
    let now = Instant::now();
    let moves_depth = count_moves_depth(g, 4);
    let elapsed = now.elapsed();
    let kn_per_s = (moves_depth as f32) / (1000.0 * elapsed.as_secs_f32());
    println!("{moves_depth} computed in {elapsed:?}.");
    println!("{kn_per_s} KN/s");
}
