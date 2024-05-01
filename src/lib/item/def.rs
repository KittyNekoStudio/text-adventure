use crate::{damage_types::damage_mod::WeaponType, item::descriptions::{get_item_lore, interactable_desctiptions}};

use super::{consumable::{HealthPotion, HEALTHPOTION}, descriptions::interactable_name, held_item::{Staff, Sword, STAFF}, interactable::{self, BED, MIRROR}, wearable::{Robe, ROBE}};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AllItems<'a> {
    HeldItem(HeldItem<'a>),
    ConsumableItem(ConsumableItem<'a>),
    WearableItem(WearableItem<'a>),
    InteractableItem(InteractableItem<'a>)
}

/* pub const EVERYITEM: [Vec<AllItems>; 4] = [
    vec![AllItems::HeldItem(STAFF)],
    vec![AllItems::ConsumableItem(HEALTHPOTION)],
    vec![AllItems::WearableItem(ROBE)],
    vec![AllItems::InteractableItem(MIRROR), AllItems::InteractableItem(BED)]
    ];
*/
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HeldItem<'a> {
    pub weapon_type: WeaponType,
    pub damage: u8,
    pub name: &'a str,
    pub description: &'a str
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct ConsumableItem<'a> {
    pub recover: u8,
    pub name: &'a str,
    pub description: &'a str
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WearableItem<'a> {
    pub defence: u8,
    pub name: &'a str,
    pub description: &'a str
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct InteractableItem<'a> {
    pub name: &'a str,
    pub description: &'a str
}

// TODO! add names to all individual items
#[derive(Debug, Clone, Copy, PartialEq)]
/// The different effects that items can have.
/// Implenented as a bool value
pub struct ItemEffect {
    pub damage: bool,
    pub recover: bool,
    pub buff: bool,
    pub debuff: bool,
    pub defence: bool
}
/// Prints the item.
pub fn print_item(item: AllItems) {
    match item {
        AllItems::HeldItem(_) => print_held(item),
        AllItems::ConsumableItem(_) => print_consumable(item),
        AllItems::WearableItem(_) => print_wearable(item),
        AllItems::InteractableItem(_) => print_interactable(item)
    }
}

/// Prints interactable item.
pub fn print_interactable(item: AllItems) {
    let interactable = match item {
        AllItems::InteractableItem(item) => item,
        _=> todo!()
    };
    println!("{}", interactable.name);
    println!("{}", interactable.description);
}
fn print_held(item: AllItems) {
    let held = match item {
        AllItems::HeldItem(item) => item,
        _ => todo!()
    };
    println!("{}", held.name);
    println!("{}", held.description);
    println!("{} damage", held.damage);
}
fn print_consumable(item: AllItems) {
    let consumable = match item {
        AllItems::ConsumableItem(item) => item,
        _ => todo!()
    };
    // TODO! find a way to get a value to get item lore
    // hard coding the third value is not useful
    println!("{}", consumable.name);
    println!("{}", consumable.description);
    println!("{} health", consumable.recover);
}
fn print_wearable(item: AllItems) {
    let wearable = match item {
        AllItems::WearableItem(item) => item,
        _ => todo!()
    };
    println!("{}", wearable.name);
    println!("{}", wearable.description);
    println!("{} defence", wearable.defence);
}