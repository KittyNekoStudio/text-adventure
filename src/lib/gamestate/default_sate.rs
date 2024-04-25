use crate::{def::recive_input, entitys::def::{check_entity_field, match_entity_field}, map::movement::{check_direction, match_for_direction}};

use super::def::GameState;

fn default_state(mut gamestate: &mut GameState) -> bool {
    let input = recive_input();
    if check_direction(&input) {
        gamestate.push_movement(match_for_direction(&input));
        println!("{:#?}", gamestate);
        return true;
    } else if check_entity_field(&input) {
        gamestate.player.entity.print_entity(match_entity_field(&input));
        return true;
    }
    println!("Nothing matches with what you typed.");
    return false;
}

pub fn get_default_state(mut gamestate: &mut GameState) {
    let mut default = default_state(gamestate);
    while !default {
        default = default_state(gamestate);
    }
}