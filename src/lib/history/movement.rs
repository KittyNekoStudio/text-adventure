use crate::{area::def::Area, gamestate::def::GameState};
/// Returns true if input is equal to a movement option.
pub fn check_room(input: &String, area: &Area) -> bool {
    if input == &area.room.main_area_name {
        return true;
    }
    for i in &area.room.sub_area_names {
        if &input == &i {
        return true;
    }
    }
    return false;
}
/// Moves to the provided room.
pub fn move_to_room(gamestate: &mut GameState) -> &GameState {
    gamestate.push_prev_area();
    if gamestate.history.last() == Some(&gamestate.current_area.room.main_area_name) {
        gamestate.current_area = gamestate.all_areas[get_area
        (gamestate
            .history
            .last()
            .expect("Error move to room 1"))]
            .clone();
        return gamestate;
    }
    for i in gamestate.current_area.room.sub_area_names.clone() {
        if gamestate.history.last() == Some(&i) {
            gamestate.current_area = gamestate.all_areas[get_area
            (gamestate
                .history
                .last()
                .expect("Error move to room 3"))]
                .clone();
        }
    }
    return gamestate;
}

pub fn get_area(area_type: &String) -> usize {
    match area_type.as_str() {
        "room" => 0,
        "bathroom" => 1,
        _ => 0
    }
}