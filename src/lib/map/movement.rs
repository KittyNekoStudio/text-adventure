use crate::gamestate::def::{CARDINALS, EAST, NORTH, SOUTH, WEST};
/// Takes a string from the user and returns the paired number to be stored in the map vec.
pub fn match_for_direction(input: &String) -> usize {
    let direction = 
    match &input.to_lowercase() as &str {
        NORTH => 1,
        EAST => 2,
        SOUTH => 3,
        WEST => 4,
        _ => 5
    };
    return direction;
}
/// Returns true if input is equal to a movement option.
pub fn check_direction(input: &String) -> bool {
    for i in CARDINALS {
        if input == i {
            return true;
        }
    }
    return false;
}