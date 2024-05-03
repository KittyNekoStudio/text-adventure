use lib::gamestate::def::GameState;

use super::all_scenes::{first_scene, second_scene};

pub fn play_game() {
    let mut gamestate = GameState::new();
    first_scene(&mut gamestate);
    gamestate.default_state();
    loop {
        if !gamestate.all_scenes[1] {
            if gamestate.second_check() {
                second_scene(&mut gamestate);
                gamestate.scene_complete(1);
            } else {
                gamestate.default_state();
            }
        } else {
            gamestate.default_state();
        }
    }
}