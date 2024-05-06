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
                .expect("Error move to room 2"))]
                .clone();
        }
    }
    return gamestate;
}
/// Returns a number corresponding to an areas name.
pub fn get_area(area_type: &String) -> usize {
    match area_type.as_str() {
        "bedroom" => 0,
        "bathroom" => 1,
        "hallway" => 2,
        "dorm 1" => 3,
        "dorm 2" => 3,
        "dorm 3" => 0,
        "dorm 4" => 3,
        "dorm 5" => 6,
        "dorm 6" => 6,
        "dorm 7" => 6,
        "dorm 8" => 6,
        "dorm 9" => 8,
        "dorm 10" => 8,
        "dorm 11" => 8,
        "dorm 12" => 8,
        "dorm 13" => 10,
        "dorm 14" => 10,
        "dorm 15" => 10,
        "dorm 16" => 10,
        "school dorms" => 4,
        "east 1 hallway" => 2,
        "east 2 hallway" => 5,
        "west 1 hallway" => 7,
        "west 2 hallway" => 9,
        _ => 0
    }
}