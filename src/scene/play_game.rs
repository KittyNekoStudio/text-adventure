use lib::gamestate::def::GameState;

use super::all_scenes::{first_scene, second_scene, start_up_scene};

pub fn play_game() {
    let mut gamestate = GameState::new();
    start_up_scene();
    first_scene(&mut gamestate);
    loop {
        // TODO! find a way for the scene to print before the room text
        if !gamestate.scenes_completed[2] {
            if gamestate.second_check() {
                second_scene(&mut gamestate);
                gamestate.complete_scene(2);
            } else {
                gamestate.default_state();
            }
        } else {
            gamestate.default_state();
        }
    }
}