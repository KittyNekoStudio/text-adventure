use super::def::{Item, ItemEffect, ItemType};
#[derive(Debug)]
pub struct Wearable {
    pub wearable: Item,
}
// TODO! create a wearable for accessories
impl Wearable {
    pub fn new() -> Self {
        Self {
            wearable: Item {
                item_type: ItemType {
                    held: false,
                    wearable: true,
                    consumable: false,
                    throwable: false
                },
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
pub struct LeatherArmor {
    armor: Wearable,
    defence: u8
}

impl LeatherArmor {
    pub fn new() -> Self {
        Self {
            armor: Wearable::new(),
            defence: 3
        }
    }
}