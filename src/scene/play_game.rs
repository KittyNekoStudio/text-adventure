use lib::gamestate::def::GameState;

use super::all_scenes::{first_scene, second_scene, start_up_scene, third_scene};

pub fn play_game() {
    let mut gamestate = GameState::new();
    start_up_scene();
    // TODO! fix bug where you can pickup the robe in the first_scene
    // but also when you re-enter the bathroom
    first_scene();
    loop {
        // TODO! find a way for the scene to print before the room text
        if !gamestate.scenes_completed[2] {
            if gamestate.second_check() {
                second_scene();
                gamestate.complete_scene(2);
            } else {
                gamestate.default_state();
            }
        } else if !gamestate.scenes_completed[3] {
            if gamestate.third_check() {
                third_scene();
                gamestate.complete_scene(3);
            } else {
                gamestate.default_state();
            }
        } else {
            gamestate.default_state();
        }
    }
}