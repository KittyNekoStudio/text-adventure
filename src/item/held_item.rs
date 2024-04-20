use crate::stats::damage_mod::DamageTypes;

use super::def::{Item, ItemEffect, ItemType};
#[derive(Debug)]
pub struct Weapon {
    pub weapon: Item,
}

impl Weapon {
    pub fn new() -> Self {
        Self {
            weapon: Item {
                item_type: ItemType {
                    held: true,
                    wearable: false,
                    consumable: false,
                    throwable: false
                },
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
pub struct Sword {
    sword: Weapon,
    damage_type: DamageTypes,
    damage: u8
}

impl Sword {
    fn new() -> Self {
        Self {
            sword: Weapon::new(),
            damage_type: DamageTypes::sharp,
            damage: 10
        }
    }
}