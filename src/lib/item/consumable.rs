use super::def::{CollectableItem, ItemType};

/// The basic health potion.
pub const HEALTHPOTION: CollectableItem = CollectableItem {
    item_type: ItemType::Consumable,
    value: 10,
    lore: 1
};