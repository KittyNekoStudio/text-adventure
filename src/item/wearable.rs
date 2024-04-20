use super::def::{Item, ItemEffect};
#[derive(Debug)]
/// The basic clothes
pub struct Clothes {
    pub item_type: u8,
    pub wearable: Item
}
// TODO! create a wearable for accessories
impl Clothes {
    pub fn new() -> Self {
        Self {
            item_type: 3,
            wearable: Item {
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
}
#[derive(Debug)]
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