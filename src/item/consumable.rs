use super::def::ItemEffect;
#[derive(Debug, Clone, Copy)]
/// The basic potion
pub struct Potion {
    pub item_type: u8,
    pub item_effect: ItemEffect
}

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
#[derive(Debug, Clone, Copy)]
// TODO! learn how to reference other code in a doc comment
/// A derivative of the potion struct that heals
pub struct HealthPotion {
    pub h_potion: Potion,
    pub heal: u8
}

impl HealthPotion {
    pub fn new() -> Self {
        Self {
            h_potion: Potion::new(),
            heal: 10
        }
    }
}