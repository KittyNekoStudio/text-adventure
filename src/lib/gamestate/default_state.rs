use crate::{area::first_room::check_item, def::recive_input, entitys::def::{check_entity_field, match_entity_field}, history::movement::{check_room, move_to_room}, item::{def::print_interactable, intercatable_items::check_int_item_field}};

use super::def::GameState;
/// The default state that player will be in.
fn default_state(gamestate: &mut GameState) -> bool {
    let input = recive_input().to_lowercase();
    
    if check_room(&input, &gamestate.current_area) {
        gamestate.push_movement(&input);
        move_to_room(gamestate);
        println!("{:?}", gamestate.current_area);
        println!("{:?}", gamestate.player.entity.inventory);
        gamestate.update_area();
        return true;
    } else if check_entity_field(&input) {
        gamestate.player.entity.print_entity(match_entity_field(&input));
        return true;
    } else if check_int_item_field(&input, gamestate.current_area.room.clone()) {
        print_interactable(gamestate.current_area.room.interactable_items[0].desc_id);
        return true;
    } else if check_item(&input) {
        gamestate.pickup_item(&input);
        return true;
    }
    if input == String::from("quit") {
        return false;
    } else {
        println!("Nothing matches with what you typed.");
        return true;
    }
}
/// Calls default_state() if it returns false.
pub fn get_default_state(gamestate: &mut GameState) {
    let mut default = default_state(gamestate);
    while default {
        default = default_state(gamestate);
    }
}

