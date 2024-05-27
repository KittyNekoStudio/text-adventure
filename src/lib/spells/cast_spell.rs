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
            if gamestate.player.entity.mana < 2 {
                return 100;
            }
            println!("Choose an object to use spell on:");
            for object in &gamestate.current_area.room.interactable_items {
                println!("{}", object.0);
            }
            println!("cancel");
            return 1;
        },
        "inspect person" => {
            if gamestate.player.entity.mana < 2 {
                return 100;
            }
            println!("Choose an object to use spell on:");
            for person in &gamestate.current_area.room.entitys {
                println!("{}", person.name);
            }
            println!("cancel");
            return 2;
        },
        "enhanced insight" => return 3,
        "see into the past" => {
            if gamestate.player.entity.mana < 5 {
                return 100;
            }
            println!("Choose spell target:");
            println!("{}", get_room_lore(gamestate.current_area.room.lore, 0).to_lowercase());
            for object in &gamestate.current_area.room.interactable_items {
                println!("{}", object.0);
            }
            println!("cancel");
            return 4;
        },
        "danger sense" => return 5,
        "wispers of the touch" => {
            if gamestate.player.entity.mana < 3 {
                return 100;
            }
            println!("Choose an object to use spell on:");
            for object in &gamestate.current_area.room.interactable_items {
                println!("{}", object.0);
            }
            println!("cancel");
            return 6;
        },
        "hear the voices" => {
            if gamestate.player.entity.mana < 5 {
                return 100;
            }
            println!("Choose an object to use spell on:");
            for object in &gamestate.current_area.room.interactable_items {
                println!("{}", object.0);
            }
            println!("cancel");
            return 7;
        },
        _ => return 0
    }
}

fn get_spell (input: &String, gamestate: &GameState) -> (usize, Option<usize>) {
    let spell = get_spell_information(input, gamestate);
    if spell == 100 {
        return (100, Some(100));
    } else if spell == 1 || spell == 6 || spell == 7 {
        let spell_input = recive_input().to_lowercase();
        if spell_input == "cancel" {
            return (1, Some(100));
        }
        for object in &gamestate.current_area.room.interactable_items {
            if spell_input == object.0 {
                if spell == 1 {
                    return (1, Some(object.1.item_lore));
                } else if spell == 6 {
                    return (6, Some(object.1.item_lore));
                } else {
                    return (7, Some(object.1.item_lore));
                }
            }
        }
    } else if spell == 2 {
        let spell_input = recive_input().to_lowercase();
        if spell_input == "cancel" {
            return (2, Some(100));
        }
        for person in &gamestate.current_area.room.entitys {
            if spell_input == person.name {
                return (2, Some(person.id))
            }
        }
    } else if spell == 3 {
        return (3, Some(0))
    } else if spell == 4 {
        let spell_input = recive_input().to_lowercase();
        if spell_input == "cancel" {
            return (4, Some(100));
        }
        if spell_input == get_room_lore(gamestate.current_area.room.lore, 0).to_lowercase() {
            return (4, Some(gamestate.current_area.room.lore));
        }
        for object in &gamestate.current_area.room.interactable_items {
            if spell_input == object.0 {
                return (4, Some(object.1.spell_lore))
            }
        }
    } else if spell == 5 {
        return (5, Some(0));
    }
    return (0, None)
}

pub fn print_spell_information(input: &String, gamestate: &mut GameState) -> String {
    let mut index = 100;
    let mut spell = get_spell(input, gamestate);
    if spell.0 == 100 {
        return "Not enough mana".to_string();
    }
    if spell.1 == None {
    while spell.1 == None {
        println!("");
        println!("Invalid spell target.");
        spell = get_spell(input, gamestate);
        }
    }
    if spell.1.unwrap() == 100 {
        return "Spell not used.".to_string();
    }
    if spell.0 == 1 || spell.0 == 6 || spell.0 == 7 { 
        index = match spell.1.expect("Error spell information") {
        4 => 0,
        3 => 1,
        5 => 2,
        6 => 3,
        _ => 100
    };
    }
    if spell.0 == 1 {
    let inspect_object = [
        // 0
    "This mirror was made in a factory.".to_string(),
        // 1
    "The bed is yours.".to_string(),
        // 2
    "This is a map.".to_string(),
        // 3
    "This is another map.".to_string()
    ];
    gamestate.player.entity.sub_mana(2);
    return inspect_object[index].clone();
    } else if spell.0 == 2 {
        let inspect_person = [
            "NPC Id starts at 1".to_string(),
            "This is the dorm manager.".to_string()
        ];
        gamestate.player.entity.sub_mana(2);
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
                // 3
            "A woman wakes up an walks into their bathroom.".to_string(),
                // 4
            "People come and go, leaving and entering the dorms.".to_string(),
                // 5
            "The hallway is crowded. People are rushing back and forth in a hurry to leave.".to_string(),
                // 6
            "A young man is sitting on his bed staring at the wall opposite him.".to_string(),
                // 7
            "The hallway is calm, there are only a handful of people leaving their rooms.".to_string(),
                // 8
            "The room is empty. There is no movement at all.".to_string(),
                // 9
            "Students come and go. It looks busy but orderly.".to_string(),
                // 10
            "You see two people sleeping in bed.".to_string(),
                // 11
            "The dorm manager is filing paperwork.".to_string(),
                // 12
            "A massive swarm of students and parents are rushing through the square.".to_string(),
                // 13
            "The faculty are getting ready for the ceremony, while the everyone else is getting seated.".to_string(),
                // 14
            "The headmaster is calling you up to the stage.".to_string(),
                // 15
            "People are sitting down around this seat.".to_string(),
                // 16
            "People are entering an exiting the school.".to_string(),
                // 17
            "You see yourself waking up and getting out of bed.".to_string(),
                // 18
            "You see yourself looking into the mirror.".to_string(),
                // 19
            "You see people passing by the map.".to_string(),
                // 20
            "You see people coming up and looking at the map.".to_string(),
        ];
        gamestate.player.entity.sub_mana(5);
        return see_into_the_past[spell.1.unwrap()].clone();
    } else if spell.0 == 5 {
        return "Find a way to use this spell. If I can't remove it.".to_string();
    } else if spell.0 == 6 {
        let wispers_of_the_touch = [
            // 0
        "Adam See, Jacob, Hall.".to_string(),
            // 1
        "Adam See, John Brunt, Jess Smith, Casey, Cross.".to_string(),
            // 2
        "Put names here 1.".to_string(),
            // 3
        "Put names here 2.".to_string()
        ];
        gamestate.player.entity.sub_mana(3);
        return wispers_of_the_touch[index].clone();
    } else if spell.0 == 7 {
        let hear_the_voices = [
            // 0
        "\"Uhg\"".to_string(),
            // 1
        "\"Grunting noises.\"".to_string(),
            // 2
        "\"Loud chatter from many people.\"".to_string(),
            // 3
        "\"Talking and footsteps are loud here.\"".to_string()
        ];
        gamestate.player.entity.sub_mana(5);
        return hear_the_voices[index].clone();
    }
    return "".to_string();
}