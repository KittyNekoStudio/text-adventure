use super::def::{Item, ItemEffect, ItemType};
#[derive(Debug)]
pub struct Potion {
    pub potion: Item
}

impl Potion {
    pub fn new() -> Self {
        Self {
            potion: Item {
                item_type: ItemType {
                    held: false,
                    wearable: false,
                    consumable: true,
                    throwable: false
                },
                item_effect: ItemEffect {
                    damage: false,
                    recover: true,
                    buff: false,
                    debuff: false,
                    defence: false
                }
            }    
        }
    }
}
#[derive(Debug)]
pub struct HealthPotion {
    potion: Potion,
    heal: u8
}

impl HealthPotion {
    pub fn new() -> Self {
        Self {
            potion: Potion::new(),
            heal: 10
        }
    }
}