use super::def::{Item, ItemEffect};
#[derive(Debug)]
/// The basic potion
pub struct Potion {
    pub item_type: u8,
    pub potion: Item
}

impl Potion {
    pub fn new() -> Self {
        Self {
            item_type: 2,
            potion: Item {
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
// TODO! learn how to reference other code in a doc comment
/// A derivative of the potion struct that heals
pub struct HealthPotion {
    pub potion: Potion,
    pub heal: u8
}

impl HealthPotion {
    pub fn new() -> Self {
        Self {
            potion: Potion::new(),
            heal: 10
        }
    }
}