use lib::{gamestate::def::GameState, history::movement::move_to_room};

use super::all_scenes::{fifth_scene, first_scene, fourth_scene, second_scene, start_up_scene, third_scene};

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
        } else if !gamestate.scenes_completed[4] {
            if gamestate.fourth_check() {
                fourth_scene();
                gamestate.complete_scene(4);
                gamestate.push_movement(&"auditorium".to_string());
                move_to_room(&mut gamestate);
                gamestate.add_entered();
                gamestate.print_room();
                gamestate.update_area();
            } else {
                gamestate.default_state();
            }
        } else if !gamestate.scenes_completed[5] {
            if gamestate.fifth_check() {
                fifth_scene();
                gamestate.complete_scene(5);
                gamestate.push_movement(&"auditorium".to_string());
                move_to_room(&mut gamestate);
                gamestate.add_entered();
                gamestate.print_room();
                gamestate.update_area();
            } else {
                gamestate.default_state();
            }
        } else {
            gamestate.default_state();
        }
    }
}