use crate::def::recive_input;

use super::player_character::PlayerCharacter;
/// A function that creates a character and allows modification of some values.
pub fn create_player() -> PlayerCharacter<'static> {
    let mut player = PlayerCharacter::new();
    let first_name = recive_input();
    let last_name = recive_input();
    let hair = choose_hair_color();
    let eye = choose_eye_color();
    player.first_name = first_name;
    player.last_name = last_name;
    player.hair_color = hair;
    player.eye_color = eye;
    return player;
}

pub fn choose_hair_color() -> String {
    let mut hair =
    match &recive_input().to_lowercase() as &str {
        "brown" => String::from("Brown"),
        "black" => String::from("Black"),
        "blond" => String::from("Blond"),
        "red" => String::from("Red"),
        "blue" => String::from("Blue"),
        "pink" => String::from("Pink"),
        "yellow" => String::from("Yellow"),
        _ => String::from("Invalid hair color")
    };
    if hair == String::from("Invalid hair color") {
        hair = choose_hair_color();
    }
    return hair;
}
pub fn choose_eye_color() -> String {
    let mut eye =
    match &recive_input().to_lowercase() as &str {
        "brown" => String::from("Brown"),
        "black" => String::from("Black"),
        "blue" => String::from("Blue"),
        "pink" => String::from("Pink"),
        "yellow" => String::from("Yellow"),
        _ => String::from("Invalid eye color")
    };
    if eye == String::from("Invalid eye color") {
        eye = choose_eye_color();
    }
    return eye;
}