use crate::{damage_types::damage_mod::MagicType, item::def::CollectableItem};

use super::entity::Entity;
/// The struct for the player character.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerCharacter {
    pub entity: Entity,
    pub first_name: String,
    pub last_name: String,
    pub combat_magic: MagicType,
    pub hair_color: String,
    pub eye_color: String
    
}

impl PlayerCharacter {
    pub fn new() -> Self {
        PlayerCharacter {
            entity: Entity::new(),
            first_name: String::from("Bob"),
            last_name: String::from("Bob"),
            combat_magic: MagicType::Fire,
            hair_color: String::from("Brown"),
            eye_color: String::from("Brown")
        }
    }
    // !TODO add a way to not need to update the variable with these methods
    // example 
    // player = player.add_sword(sword)
    pub fn add_item(&mut self, item: CollectableItem) -> &Self {
        self.entity.inventory.push(item);
        self
    }
}