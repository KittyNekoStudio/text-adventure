use lib::{all_scenes::all_scenes::{first_scene, start_up_scene}, gamestate::def::GameState, history::movement::move_to_room};

pub fn play_game() {
    let mut gamestate = GameState::new();
    start_up_scene();
    first_scene();
    gamestate.push_movement(&"bathroom".to_string());
    move_to_room(&mut gamestate);
    gamestate.add_entered();
    gamestate.print_room();
    loop {
        gamestate.default_state();
    }
}