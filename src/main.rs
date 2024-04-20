use item::{consumable::HealthPotion, def::AllItems, held_item::Sword};

use crate::item::{def::{check_item_effect, check_item_type}, wearable::LeatherArmor};

mod def;
mod item;
mod stats;
mod characters;
fn main() {
   /* let mut vec: Vec<AllItems> = vec![];
    let sword = Sword::new();
    let potion = HealthPotion::new();
    let armor = LeatherArmor::new();
    vec.push(AllItems::Sword(sword));
    vec.push(AllItems::HealthPotion(potion.clone()));
    vec.push(AllItems::LeatherArmor(armor));
    println!("{:?}", check_item_effect(vec[2].clone(), "damage"));
    println!("{:?}", check_item_effect(vec[2].clone(), "healing"));
    println!("{:?}", check_item_effect(vec[2].clone(), "buff"));
    println!("{:?}", check_item_effect(vec[2].clone(), "debuff"));
    println!("{:?}", check_item_effect(vec[2].clone(), "defence"));
    println!("{:?}", check_item_effect(vec[2].clone(), "loser")); */
}
