use crate::{def::recive_input, entitys::def::{check_entity_field, match_entity_field}, history::movement::{check_room, move_to_room}, item::item_interaction::item_interaction};

use super::def::GameState;
/// The default state that player will be in.
fn default_state(gamestate: &mut GameState) -> bool {
    let input = recive_input().to_lowercase();
    let interaction = item_interaction(&input, gamestate);
    if check_room(&input, &gamestate.current_area) {
        gamestate.push_movement(&input);
        move_to_room(gamestate);
        gamestate.add_entered();
        gamestate.print_room();
        gamestate.update_area();
        println!("{:#?}", gamestate.current_area);
        return true;
    } else if check_entity_field(&input) {
        gamestate.player.entity.print_entity(match_entity_field(&input));
        return true;
    } else if interaction == 1 {
        if gamestate.current_area.room.collectable_item.len() == 0 {
            println!("Nothing matches with what you typed.");
            return true;
        }
        gamestate.pickup_item(gamestate.get_collect_index(&input));
        if !gamestate.store {
            println!("Nothing matches with what you typed.");
            return true;
        }
        return true;
    }
    if input == String::from("quit") {
        return false;
    } else {
        if interaction == 0 {return true;}
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

