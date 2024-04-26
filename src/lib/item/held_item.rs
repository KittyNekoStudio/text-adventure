use crate::damage_types::damage_mod::{MagicType, WeaponType};

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
pub struct Staff {
    pub name: usize,
    pub description: usize,
    pub staff: Weapon,
    pub damage_type: WeaponType,
    pub damage: u8
}
impl Staff {
    pub fn new() -> Self {
        Self {
            name: 1,
            description: 1,
            staff: Weapon::new(),
            damage_type: WeaponType::Magic(MagicType::Fire),
            damage: 3
        }
    }
}
#[derive(Debug, Clone, Copy)]
/// A derivative of the weapon struct
pub struct Sword {
    pub name: usize,
    pub description: usize,
    pub sword: Weapon,
    pub damage_type: WeaponType,
    pub damage: u8
}

impl Sword {
    pub fn new() -> Self {
        Self {
            name: 3,
            description: 3,
            sword: Weapon::new(),
            damage_type: WeaponType::Sharp,
            damage: 10
        }
    }
}