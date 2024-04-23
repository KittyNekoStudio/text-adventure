use super::def::ItemEffect;
#[derive(Debug, Clone, Copy)]
/// The basic clothes
pub struct Clothes {
    pub item_type: u8,
    pub item_effect: ItemEffect
}
// TODO! create a wearable for accessories
impl Clothes {
    pub fn new() -> Self {
        Self {
            item_type: 3,
            item_effect: ItemEffect {
                damage: false,
                recover: false,
                buff: false,
                debuff: false,
                defence: true
            }
        }
    }
}
#[derive(Debug, Clone, Copy)]
/// A derivative of the clothes struct
pub struct LeatherArmor {
    pub armor: Clothes,
    pub defence: u8
}

impl LeatherArmor {
    pub fn new() -> Self {
        Self {
            armor: Clothes::new(),
            defence: 3
        }
    }
}