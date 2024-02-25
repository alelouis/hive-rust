use crate::logic::game::Game;

pub(crate) struct Eval;
impl minimax::Evaluator for Eval {
    type G = Game;
    fn evaluate(&self, state: &Game) -> minimax::Evaluation {
        state
            .get_hive()
            .as_ref()
            .expect("Couldn't get hive.")
            .count_bugs_of_color(state.turn_color) as minimax::Evaluation
    }
}
