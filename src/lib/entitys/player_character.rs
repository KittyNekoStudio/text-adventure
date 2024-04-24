use crate::{damage_types::damage_mod::MagicType, item::{consumable::HealthPotion, def::AllItems, held_item::Sword, wearable::LeatherArmor}};

use super::entity::Entity;
/// The struct for the player character.
#[derive(Debug, Clone)]
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
    pub fn add_sword(mut self, item: Sword) -> Self {
        self.entity.inventory.push(AllItems::Sword(item));
        self
    }
    
    pub fn add_health_potion(mut self, item: HealthPotion) -> Self {
        self.entity.inventory.push(AllItems::HealthPotion(item));
        self
    }

    pub fn add_leather_armor(mut self, item: LeatherArmor) -> Self {
        self.entity.inventory.push(AllItems::LeatherArmor(item));
        self
    }
}