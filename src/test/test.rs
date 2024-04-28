use lib::gamestate::{default_state::get_default_state, new_game::new_game};

/// A function to test various code.
pub fn test() {
    let mut game = new_game();
    get_default_state(&mut game);
}