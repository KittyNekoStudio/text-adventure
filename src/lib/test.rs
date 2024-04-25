use crate::gamestate::{def::GameState, default_sate::get_default_state};

/// A function to test various code.
pub fn test() {
    let mut gamestate = GameState::new();
    get_default_state(&mut gamestate);
}