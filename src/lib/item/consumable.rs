use super::def::{ConsumableItem, ItemEffect};
#[derive(Debug, Clone, Copy, PartialEq)]
/// The basic potion
pub struct Potion {
    pub item_type: u8,
    pub item_effect: ItemEffect
}


pub const HEALTHPOTION: ConsumableItem = ConsumableItem {
    recover: 10,
    name: "Basic Health Potion",
    description: "The cheapest of healing potions."
};

impl Potion {
    pub fn new() -> Self {
        Self {
            item_type: 2,
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
#[derive(Debug, Clone, Copy, PartialEq)]
// TODO! learn how to reference other code in a doc comment
/// A derivative of the potion struct that heals
pub struct HealthPotion {
    pub name: usize,
    pub description: usize,
    pub h_potion: Potion,
    pub heal: u8,
    pub desc_id: usize
}

impl HealthPotion {
    pub fn new() -> Self {
        Self {
            name: 2,
            description: 2,
            h_potion: Potion::new(),
            heal: 10,
            desc_id: 1
        }
    }
    pub fn change_desc_id(&mut self, num: usize) -> Self {
        self.desc_id = num;
        *self
    }
}