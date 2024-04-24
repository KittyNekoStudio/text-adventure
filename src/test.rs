use crate::{entitys::create_entity::create_player, item::{def::AllItems, held_item::Sword}};

pub fn test() {
    let mut player = create_player();
    let sword = Sword::new();
    player = player.add_sword(sword);
    println!("{:?}", player);
}