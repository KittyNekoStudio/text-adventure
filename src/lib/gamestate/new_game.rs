use super::{def::GameState, default_state::get_default_state};
/// Creates start of the game.
pub fn new_game() {
    let mut gamestate = GameState::new();
    get_default_state(&mut gamestate);
    
}