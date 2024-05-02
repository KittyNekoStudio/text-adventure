use crate::damage_types::damage_mod::{MagicType, WeaponType};

use super::def::{CollectableItem, ItemType};

/// The basic staff.
pub const STAFF: CollectableItem = CollectableItem {
    item_type: ItemType::Weapon(WeaponType::Magic(MagicType::Fire)),
    value: 3,
    lore: 0,
};