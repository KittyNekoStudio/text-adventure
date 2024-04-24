use crate::def::recive_input;

use super::player_character::PlayerCharacter;

pub fn create_player() -> PlayerCharacter {
    let mut player = PlayerCharacter::new();
    let hair = choose_hair_color();
    let eye = choose_eye_color();
    player.hair_color = hair;
    player.eye_color = eye;
    return player;
}

pub fn choose_hair_color() -> String {
    let mut hair = recive_input();
    match &hair.to_lowercase() as &str {
        "brown" => hair = String::from("Brown"),
        "black" => hair = String::from("Black"),
        "blond" => hair = String::from("Blond"),
        "red" => hair = String::from("Red"),
        "blue" => hair = String::from("Blue"),
        "pink" => hair = String::from("Pink"),
        "yellow" => hair = String::from("Yellow"),
        _ => hair = String::from("Invalid hair color")
    }
    if hair == String::from("Invalid hair color") {
        hair = choose_hair_color();
    }
    return hair;
}
pub fn choose_eye_color() -> String {
    let mut eye = recive_input();
    match &eye.to_lowercase() as &str {
        "brown" => eye = String::from("Brown"),
        "black" => eye = String::from("Black"),
        "blue" => eye = String::from("Blue"),
        "pink" => eye = String::from("Pink"),
        "yellow" => eye = String::from("Yellow"),
        _ => eye = String::from("Invalid eye color")
    }
    if eye == String::from("Invalid eye color") {
        eye = choose_eye_color();
    }
    return eye;
}