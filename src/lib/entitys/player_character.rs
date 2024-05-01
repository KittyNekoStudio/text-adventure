use crate::{damage_types::damage_mod::MagicType, item::{consumable::HealthPotion, def::{AllItems, ConsumableItem, HeldItem, WearableItem}, held_item::{Staff, Sword}, wearable::Robe}};

use super::entity::Entity;
/// The struct for the player character.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerCharacter<'a> {
    pub entity: Entity<'a>,
    pub first_name: String,
    pub last_name: String,
    pub combat_magic: MagicType,
    pub hair_color: String,
    pub eye_color: String
    
}

impl<'a> PlayerCharacter<'a> {
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
    pub fn add_held(&mut self, item: HeldItem) -> &Self {
        self.entity.inventory.push(AllItems::HeldItem(item));
        self
    }
    pub fn add_consumable(&mut self, item: ConsumableItem) -> &Self {
        self.entity.inventory.push(AllItems::ConsumableItem(item));
        self
    }
    pub fn add_wearable(&mut self, item: WearableItem) -> &Self {
        self.entity.inventory.push(AllItems::WearableItem(item));
        self
    }
}