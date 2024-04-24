use crate::def::recive_input;
/// Takes a string from the user and returns the paired number to be stored in the map vec.
pub fn match_for_direction() -> usize {
    let mut direction = 
    match &recive_input().to_lowercase() as &str {
        "north" => 1,
        "east" => 2,
        "south" => 3,
        "west" => 4,
        _ => 5
    };
    if direction == 5 {
        direction = match_for_direction();
    }
    return direction;
}