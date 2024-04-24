use crate::{damage_types::damage_mod::MagicType, item::{consumable::HealthPotion, def::AllItems, held_item::Sword, wearable::LeatherArmor}};

use super::entity::Entity;

#[derive(Debug, Clone)]
pub struct PlayerCharacter {
    pub entity: Entity,
    pub combat_magic: MagicType,
    pub eye_color: String,
    pub hair_color: String,
}

impl PlayerCharacter {
    pub fn new() -> Self {
        PlayerCharacter {
            entity: Entity::new(),
            combat_magic: MagicType::Fire,
            eye_color: String::from("Brown"),
            hair_color: String::from("Brown")
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