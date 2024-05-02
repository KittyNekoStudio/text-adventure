use crate::{damage_types::damage_mod::MagicType, def::recive_input};

use super::player_character::PlayerCharacter;
/// A function that creates a character and allows modification of some values.
pub fn create_player() -> PlayerCharacter {
    let mut player = PlayerCharacter::new();
    println!("Please choose your first name.");
    let first_name = recive_input();
    println!("Please choose your last name.");
    let last_name = recive_input();
    println!("Please choose your hair color.");
    let hair = choose_hair_color();
    println!("Please choose your eye color.");
    let eye = choose_eye_color();
    println!("Please choose your combat magic type.");
    let magic = choose_magic_type();
    player.first_name = first_name;
    player.last_name = last_name;
    player.hair_color = hair;
    player.eye_color = eye;
    player.combat_magic = magic.expect("Error at player choosing magic type.");
    return player;
}

fn choose_hair_color() -> String {
    let mut hair =
    match recive_input().to_lowercase().as_str() {
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
        println!("Hair color not allowed.");
        hair = choose_hair_color();
    }
    return hair;
}
fn choose_eye_color() -> String {
    let mut eye =
    match recive_input().to_lowercase().as_str() {
        "brown" => String::from("Brown"),
        "black" => String::from("Black"),
        "blue" => String::from("Blue"),
        "pink" => String::from("Pink"),
        "yellow" => String::from("Yellow"),
        _ => String::from("Invalid eye color")
    };
    if eye == String::from("Invalid eye color") {
        println!("Eye color not allowed.");
        eye = choose_eye_color();
    }
    return eye;
}

fn choose_magic_type() -> Option<MagicType> {
    let mut magic = 
    match recive_input().to_lowercase().as_str() {
        "fire" => Some(MagicType::Fire),
        "ice" => Some(MagicType::Ice),
        "acid" => Some(MagicType::Acid),
        "arcane" => Some(MagicType::Arcane),
        _ => None

    };
    if magic == None {
        println!("Magic type not allowed.");
        magic = choose_magic_type();
    }
    return magic;
}