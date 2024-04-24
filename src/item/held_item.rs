use crate::damage_types::damage_mod::WeaponType;

use super::def::ItemEffect;
#[derive(Debug, Clone, Copy)]
/// The basic weapon
pub struct Weapon {
    pub item_type: u8,
    pub item_effect: ItemEffect,
}

impl Weapon {
    pub fn new() -> Self {
        Self {
            item_type: 1,
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
#[derive(Debug, Clone, Copy)]
/// A derivative of the weapon struct
pub struct Sword {
    pub sword: Weapon,
    pub damage_type: WeaponType,
    pub damage: u8
}

impl Sword {
    pub fn new() -> Self {
        Self {
            sword: Weapon::new(),
            damage_type: WeaponType::Sharp,
            damage: 10
        }
    }
}