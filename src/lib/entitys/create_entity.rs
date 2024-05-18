use crate::{damage_types::damage_mod::MagicType, def::recive_input};

use super::{entity::Entity, npcs::NPC, player_character::PlayerCharacter};
/// A function that creates a character and allows modification of some values.
pub fn create_player() -> PlayerCharacter {
    let mut player = PlayerCharacter::new();
    println!("Please choose your combat magic type.");
    let magic = choose_magic_type();
    player.combat_magic = magic.expect("Error at player choosing magic type.");
    return player;
}
/// Allows the choosing of the players magic type.
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
/// Create the dorm manager.
pub fn create_dorm_manager() -> NPC {
    let manager = NPC {
        entity: Entity::new(),
        name: "manager".to_string(),
        dialogue: vec!["What are you doing here Adam? It't not like you to be late.
Get to the auditorium before you miss the ceremony.".to_string(), "Why are you still here, get a move on.".to_string()],
        talked_to: 0,
        id: 1
    };
    
    return manager;
}