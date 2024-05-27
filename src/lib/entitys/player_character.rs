use crate::{damage_types::damage_mod::MagicType, item::def::CollectableItem};

use super::entity::Entity;
/// The struct for the player character.
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerCharacter {
    pub entity: Entity,
    pub first_name: String,
    pub last_name: String,
    pub combat_magic: MagicType,
}

impl PlayerCharacter {
    pub fn new() -> Self {
        PlayerCharacter {
            entity: Entity {
                spells: vec![0, 1, 2, 3, 4, 5, 6],
                spell_circle: vec![0, 1],
                inventory: vec![],
                mana: 50
            },
            first_name: String::from("Adam"),
            last_name: String::from("See"),
            combat_magic: MagicType::Fire,
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