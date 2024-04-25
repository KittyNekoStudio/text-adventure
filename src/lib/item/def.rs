use super::{consumable::HealthPotion, held_item::{Staff, Sword}, wearable::LeatherArmor};

#[derive(Debug, Clone, Copy)]
pub enum AllItems {
    Sword(Sword),
    HealthPotion(HealthPotion),
    LeatherArmor(LeatherArmor),
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
        AllItems::HealthPotion(HealthPotion { h_potion, heal }) => h_potion.item_type,
        AllItems::Sword(Sword { sword, damage_type, damage }) => sword.item_type,
        AllItems::LeatherArmor(LeatherArmor { armor, defence }) => armor.item_type,
        AllItems::Staff(Staff { staff, damage_type, damage }) => staff.item_type
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
        AllItems::HealthPotion(HealthPotion { h_potion, heal }) => &h_potion.item_effect,
        AllItems::Sword(Sword { sword, damage_type, damage }) => &sword.item_effect,
        AllItems::LeatherArmor(LeatherArmor { armor, defence }) => &armor.item_effect,
        AllItems::Staff(Staff { staff, damage_type, damage }) => &staff.item_effect
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