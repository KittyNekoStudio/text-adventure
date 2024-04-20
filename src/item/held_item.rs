use crate::stats::damage_mod::DamageTypes;

use super::def::{Item, ItemEffect};
#[derive(Debug)]
/// The basic weapon
pub struct Weapon {
    pub item_type: u8,
    pub weapon: Item,
}

impl Weapon {
    pub fn new() -> Self {
        Self {
            item_type: 1,
            weapon: Item {
                item_effect: ItemEffect {
                    damage: true,
                    recover: false,
                    buff: false,
                    debuff: false,
                    defence: false
                }
            }            
        }
    }
}
#[derive(Debug)]
/// A derivative of the weapon struct
pub struct Sword {
    pub sword: Weapon,
    pub damage_type: DamageTypes,
    pub damage: u8
}

impl Sword {
    pub fn new() -> Self {
        Self {
            sword: Weapon::new(),
            damage_type: DamageTypes::Sharp,
            damage: 10
        }
    }
}