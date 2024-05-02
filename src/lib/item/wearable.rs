use super::def::{CollectableItem, ItemType};

/// The basic mage robe.
pub const ROBE: CollectableItem = CollectableItem {
    item_type: ItemType::Wearable,
    value: 3,
    lore: 2
};