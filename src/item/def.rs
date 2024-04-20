#[derive(Debug)]
/// The different effects that items can have.
/// Implenented as a bool value
pub struct ItemEffect {
    pub damage: bool,
    pub recover: bool,
    pub buff: bool,
    pub debuff: bool,
    pub defence: bool
}
#[derive(Debug)]
/// The struct that defines the base item.
pub struct Item {
    pub item_effect: ItemEffect
}

impl Item {
    pub fn new() -> Self {
        Self {
            item_effect: ItemEffect {
                damage: false,
                recover: false,
                buff: false,
                debuff: false,
                defence: false
                }
        }
    }
}
/// Returns the item_type based on the argument
pub fn check_item_type(item_type: u8) -> &'static str {
    match item_type {
        1 => "held",
        2 => "consumable",
        3 => "wearable",
        4 => "throwable",
        _ => "Item type is not an allowed number"
    }
}
/// Checks if the item has any of the item effects
// Returns a bool and u8 to as I want to cheack against wrongly types arguments
pub fn check_item_effect(item: Item, text: &str) -> (bool, u8) {
    match text {
        "damage" => match item {
            Item { item_effect, .. } => (item_effect.damage, 1)
        },
        "healing" => match item {
            Item { item_effect, .. } => (item_effect.recover, 1)
        },
        "buff" => match item {
            Item { item_effect, .. } => (item_effect.buff, 1)
        },
        "debuff" => match item {
            Item { item_effect, .. } => (item_effect.debuff, 1)
        },
        "defence" => match item {
            Item { item_effect, .. } => (item_effect.defence, 1)
        },
        _ => (false, 0)
    }
}