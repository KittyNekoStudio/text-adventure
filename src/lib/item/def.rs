use crate::{damage_types::damage_mod::WeaponType, item::descriptions::get_item_lore};

use super::{consumable::HEALTHPOTION, held_item::STAFF, interactable::{BED, CAMPUSSQUAREMAP, DORMMAP, MIRROR, SCHOOLTRASHCAN}, wearable::ROBE};
/// Every collectable item in the game.
pub const EVERYCOLITEM: [CollectableItem; 3] = [
    STAFF, HEALTHPOTION, ROBE
];
/// Every interactable item in the game.
pub const EVERYINTITEM: [InteractableItem; 5] = [
    BED, MIRROR, DORMMAP, CAMPUSSQUAREMAP, SCHOOLTRASHCAN
];

/// The struct that defines Collectable items.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CollectableItem {
    pub item_type: ItemType,
    pub value: i32,
    pub lore: usize
}
/// The struct that defines Interactable items.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InteractableItem {
    pub item_lore: usize,
    pub spell_lore: usize
}
/// An enum that defines the item type of a Collectable item.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ItemType {
    Weapon(WeaponType),
    Consumable,
    Wearable
}

/// Prints Interactable item.
pub fn print_interactable(item: InteractableItem) {
    println!("{}", get_item_lore(item.item_lore, 0));
    println!("{}", get_item_lore(item.item_lore, 1));
}

/// Prints Collectable item.
pub fn print_collectable(item: CollectableItem) {
    println!("{}", get_item_lore(item.lore, 0));
    println!("{}", get_item_lore(item.lore, 1));
    if item.item_type == ItemType::Consumable {
        println!("recover {} health.", item.value);
    } else if item.item_type == ItemType::Wearable {
        println!("{} defence.", item.value);
    } else {
        println!("{} damage.", item.value);
    }
}