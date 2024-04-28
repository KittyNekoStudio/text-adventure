use super::def::ItemEffect;
#[derive(Debug, Clone, Copy, PartialEq)]
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
#[derive(Debug, Clone, Copy, PartialEq)]
/// A derivative of the clothes struct
pub struct Robe {
    pub name: usize,
    pub description: usize,
    pub armor: Clothes,
    pub defence: u8,
    pub desc_id: usize
}

impl Robe {
    pub fn new() -> Self {
        Self {
            name: 4,
            description: 4,
            armor: Clothes::new(),
            defence: 3,
            desc_id: 1
        }
    }
    pub fn change_desc_id(&mut self, num: usize) -> Self {
        self.desc_id = num;
        *self
    }
}