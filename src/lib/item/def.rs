use crate::item::descriptions::get_item_lore;

use super::{consumable::HealthPotion, held_item::{Staff, Sword}, wearable::Robe};

#[derive(Debug, Clone, Copy)]
pub enum AllItems {
    Sword(Sword),
    HealthPotion(HealthPotion),
    Robe(Robe),
    Staff(Staff)
}

// TODO! add names to all individual items
#[derive(Debug, Clone, Copy)]
/// The different effects that items can have.
/// Implenented as a bool value
pub struct ItemEffect {
    pub damage: bool,
    pub recover: bool,
    pub buff: bool,
    pub debuff: bool,
    pub defence: bool
}

/// Returns the item_type based on the argument
pub fn check_item_type(item: AllItems) -> String {
    let matched_item = match &item {
        AllItems::HealthPotion(HealthPotion { h_potion, .. }) => h_potion.item_type,
        AllItems::Sword(Sword { sword, .. }) => sword.item_type,
        AllItems::Robe(Robe { armor, .. }) => armor.item_type,
        AllItems::Staff(Staff { staff, .. }) => staff.item_type
    };
    match matched_item {
        1 => "held".to_string(),
        2 => "consumable".to_string(),
        3 => "wearable".to_string(),
        4 => "throwable".to_string(),
        _ => "Invalid number".to_string()
    }
}
/// Checks if the item has any of the item effects
// Returns a bool and u8 to as I want to check against wrongly typed arguments
pub fn check_item_effect(item: AllItems, text: &str) -> (bool, u8) {
    let matched_item = match &item {
        AllItems::HealthPotion(HealthPotion { h_potion, .. }) => &h_potion.item_effect,
        AllItems::Sword(Sword { sword, .. }) => &sword.item_effect,
        AllItems::Robe(Robe { armor, .. }) => &armor.item_effect,
        AllItems::Staff(Staff { staff, .. }) => &staff.item_effect
    };
    match text {
        "damage" => match matched_item {
            ItemEffect { damage, .. } => (*damage, 1)
        },
        "healing" => match matched_item {
            ItemEffect { recover, .. } => (*recover, 1)
        },
        "buff" => match matched_item {
            ItemEffect { buff, .. } => (*buff, 1)
        },
        "debuff" => match matched_item {
            ItemEffect { debuff, .. } => (*debuff, 1)
        },
        "defence" => match matched_item {
            ItemEffect { defence, .. } => (*defence, 1)
        },
        _ => (false, 0)
    }
}
pub fn print_item(item: AllItems) {
    match item {
        AllItems::Staff(_) => print_staff(item),
        AllItems::Sword(_) => print_sword(item),
        AllItems::HealthPotion(_) => print_health_potion(item),
        AllItems::Robe(_) => print_robe(item)
    }
}
fn print_staff(item: AllItems) {
    let staff = match item {
        AllItems::Staff(item) => item,
        _ => Staff::new()
    };
    println!("{}", get_item_lore((1, 1, 1)));
    println!("{}", get_item_lore((2, 1, 1)));
    println!("{} damage", staff.damage);
}
fn print_health_potion(item: AllItems) {
    let potion = match item {
        AllItems::HealthPotion(item) => item,
        _ => HealthPotion::new()
    };
    // TODO! find a way to get a value to get item lore
    // hard coding the third value is not useful
    println!("{}", get_item_lore((1, 2, 1)));
    println!("{}", get_item_lore((2, 2, 1)));
    println!("{} health", potion.heal);
}
fn print_sword(item: AllItems) {
    let sword = match item {
        AllItems::Sword(item) => item,
        _ => Sword::new()
    };
    println!("{}", get_item_lore((1, 3, 1)));
    println!("{}", get_item_lore((2, 3, 1)));
    println!("{} damage", sword.damage);
}
fn print_robe(item: AllItems) {
    let robe = match item {
        AllItems::Robe(item) => item,
        _ => Robe::new()
    };
    println!("{}", get_item_lore((1, 4, 1)));
    println!("{}", get_item_lore((2, 4, 1)));
    println!("{} defence", robe.defence);
}