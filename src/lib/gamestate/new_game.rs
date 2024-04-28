use crate::{area::first_room::FirstRoom, item::{held_item::Staff, wearable::Robe}};

use super::def::GameState;
/// Creates start of the game.
pub fn new_game() -> GameState {
    let mut gamestate = GameState::new();
    gamestate.player.add_robe(Robe::new());
    return gamestate;
}