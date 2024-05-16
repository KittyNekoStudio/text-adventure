use crate::{def::recive_input, gamestate::def::GameState};

/// Checks which spell you are looking for.
pub fn check_spell(input: &String) -> bool {
    let fields = [
        "inspect person",
        "inspect object",
        "enhanced insight",
        "see into the past",
        "danger sense",
        "wispers of the touch",
        "hear the voices"
        ];
    for i in fields {
        if input == i {
            return true;
        }
    }
    return false;
}

fn get_spell_information(input: &String, gamestate: &GameState) -> usize {
    match input.as_str() {
        "inspect object" => {
            println!("Choose an object to use spell on:");
            for object in &gamestate.current_area.room.interactable_items {
                println!("{}", object.0)
            }
            return 1;
        },
        "inspect person" => return 2,
        "enhanced insight" => return 3,
        "see into the past" => return 4,
        "danger sense" => return 5,
        "wispers of the touch" => return 6,
        "hear the voices" => return 7,
        _ => return 0
    }
}

fn get_object (input: &String, gamestate: &GameState) -> Option<usize> {
    let spell = get_spell_information(input, gamestate);
    if spell == 1 {
        let spell_input = recive_input().to_lowercase();
        for object in &gamestate.current_area.room.interactable_items {
            if spell_input == object.0 {
                return Some(object.1.lore)
            }
        }
    }
    return None
}

pub fn print_spell_information(input: &String, gamestate: &GameState) -> String {
    let object = get_object(input, gamestate).expect("Error spell information");
    let index = match object {
        4 => 0,
        3 => 1,
        5 => 2,
        6 => 3,
        _ => 100
    };
    let inspect_object = [
        "This mirror was made in a factory.".to_string(),
        "The bed is yours.".to_string(),
        "This is a map.".to_string(),
        "This is another map.".to_string()
    ];
    return inspect_object[index].clone();
}