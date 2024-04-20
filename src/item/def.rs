use super::{consumable::HealthPotion, held_item::Sword, wearable::LeatherArmor};

// TODO! implement copy for AllItems and it's nestled items
#[derive(Debug, Clone)]
pub enum AllItems {
    Sword(Sword),
    HealthPotion(HealthPotion),
    LeatherArmor(LeatherArmor)
}

#[derive(Debug, Clone)]
/// The different effects that items can have.
/// Implenented as a bool value
pub struct ItemEffect {
    pub damage: bool,
    pub recover: bool,
    pub buff: bool,
    pub debuff: bool,
    pub defence: bool
}
#[derive(Debug, Clone)]
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
pub fn check_item_type(item: AllItems) -> String {
    let matched_item = match &item {
        AllItems::HealthPotion(HealthPotion { h_potion, heal }) => h_potion.item_type,
        AllItems::Sword(Sword { sword, damage_type, damage }) => sword.item_type,
        AllItems::LeatherArmor(LeatherArmor { armor, defence }) => armor.item_type
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
// Returns a bool and u8 to as I want to cheack against wrongly types arguments
pub fn check_item_effect(item: AllItems, text: &str) -> (bool, u8) {
    let matched_item = match &item {
        AllItems::HealthPotion(HealthPotion { h_potion, heal }) => &h_potion.potion,
        AllItems::Sword(Sword { sword, damage_type, damage }) => &sword.weapon,
        AllItems::LeatherArmor(LeatherArmor { armor, defence }) => &armor.wearable
    };
    match text {
        "damage" => match matched_item {
            Item { item_effect, .. } => (item_effect.damage, 1)
        },
        "healing" => match matched_item {
            Item { item_effect, .. } => (item_effect.recover, 1)
        },
        "buff" => match matched_item {
            Item { item_effect, .. } => (item_effect.buff, 1)
        },
        "debuff" => match matched_item {
            Item { item_effect, .. } => (item_effect.debuff, 1)
        },
        "defence" => match matched_item {
            Item { item_effect, .. } => (item_effect.defence, 1)
        },
        _ => (false, 0)
    }
}