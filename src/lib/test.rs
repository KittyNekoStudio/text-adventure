use crate::{gamestate::{def::GameState, default_sate::get_default_state}, item::{consumable::HealthPotion, held_item::{Staff, Sword}, wearable::Robe}};

/// A function to test various code.
pub fn test() {
    let mut gamestate = GameState::new();
    let staff = Staff::new();
    let sword = Sword::new();
    let potion = HealthPotion::new();
    let robe = Robe::new();
    gamestate.player.add_staff(staff);
    gamestate.player.add_sword(sword);
    gamestate.player.add_health_potion(potion);
    gamestate.player.add_robe(robe);
    get_default_state(&mut gamestate);
}