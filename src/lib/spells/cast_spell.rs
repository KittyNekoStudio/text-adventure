use crate::{def::recive_input, gamestate::def::GameState, item::descriptions::get_room_lore};
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
                println!("{}", object.0);
            }
            println!("cancel");
            return 1;
        },
        "inspect person" => {
            println!("Choose an object to use spell on:");
            for person in &gamestate.current_area.room.entitys {
                println!("{}", person.name);
            }
            println!("cancel");
            return 2;
        },
        "enhanced insight" => return 3,
        "see into the past" => {
            println!("Choose spell target:");
            println!("{}", get_room_lore(gamestate.current_area.room.lore, 0).to_lowercase());
            for object in &gamestate.current_area.room.interactable_items {
                println!("{}", object.0);
            }
            println!("cancel");
            return 4;
        },
        "danger sense" => return 5,
        "wispers of the touch" => return 6,
        "hear the voices" => return 7,
        _ => return 0
    }
}

fn get_spell (input: &String, gamestate: &GameState) -> (usize ,Option<usize>) {
    let spell = get_spell_information(input, gamestate);
    if spell == 1 {
        let spell_input = recive_input().to_lowercase();
        if spell_input == "cancel" {
            return (1, Some(0));
        }
        for object in &gamestate.current_area.room.interactable_items {
            if spell_input == object.0 {
                return (1, Some(object.1.lore))
            }
        }
    } else if spell == 2 {
        let spell_input = recive_input().to_lowercase();
        if spell_input == "cancel" {
            return (2, Some(0));
        }
        for person in &gamestate.current_area.room.entitys {
            if spell_input == person.name {
                return (2, Some(person.id))
            }
        }
    } else if spell == 3 {
        return (3, Some(1))
    } else if spell == 4 {
        let spell_input = recive_input().to_lowercase();
        if spell_input == "cancel" {
            return (4, Some(0));
        }
        for object in &gamestate.current_area.room.interactable_items {
            if spell_input == object.0 {
                return (4, Some(object.1.lore))
            }
            if spell_input == get_room_lore(gamestate.current_area.room.lore, 0).to_lowercase() {
                return (4, Some(gamestate.current_area.room.lore));
            }
        }
    }
    return (0, None)
}

pub fn print_spell_information(input: &String, gamestate: &GameState) -> String {
    let mut spell = get_spell(input, gamestate);
    if spell.1 == None {
    while spell.1 == None {
        println!("");
        println!("Invalid spell target.");
        spell = get_spell(input, gamestate);
        }
    }
    if spell.1.unwrap() == 0 {
        return "Spell not used.".to_string();
    }
    if spell.0 == 1 { 
        let index = match spell.1.expect("Error spell information") {
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
    } else if spell.0 == 2 {
        let inspect_person = [
            "NPC Id starts at 1".to_string(),
            "This is the dorm manager.".to_string()
        ];
        return inspect_person[spell.1.unwrap()].clone();
    } else if spell.0 == 3 {
        return "Spell not used on it's own.".to_string();
    } else if spell.0 == 4 {
        // TODO! find a consistent voice for each spell
        let see_into_the_past = [
                // 0
            "You wake up and then walk into the bathroom.".to_string(),
                // 1
            "You see yourself walking into the bathroom and then looking into the mirror.".to_string(),
                // 2
            "All sorts of people walk through the hallway.".to_string(),
            "You see yourself looking into the mirror.".to_string(),
            "You see yourself waking up.".to_string(),
            "You see people passing by the map.".to_string(),
            "You see people coming up and looking at the map.".to_string()
        ];
        return see_into_the_past[spell.1.unwrap()].clone();
    }
    return "".to_string();
}