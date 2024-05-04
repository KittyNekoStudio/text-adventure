use colored::Colorize;

/// Get the descriptions or name of an item.
pub fn get_item_lore(index1: usize, index2: usize) -> String {
    let item_lore = [
    ["Basic Staff".bold(), "A staff that can be found anywhere. Nothing special.".into()], 
    ["Basic Health Potion".bold(), "The cheapest of healing potions.".into()],
    ["Basic Mage Robe".bold(), "As much use as a bath robe.".into()],
    ["Your Bed".bold(), "The place you slept in for the past 6 years. That changes today.".into()],
    ["Mirror".bold(), "It refects your beautiful face.".into()],
    ];
    item_lore[index1][index2].to_string()
}
/// Get the descriptions or name of the room.
pub fn get_room_lore(index1: usize, index2: usize) -> String {
    let room_lore = [
    ["Bedroom".bold(), "The dorm looks clean but empty. Seems like the owner isn't home often.".into()],
    ["Bathroom".bold(), "A small dorm bathroom.".into()],
    ["Hallway".bold(), "The unkept hallway that connects school dorms together.".into()]
    ];
    room_lore[index1][index2].to_string()
}

pub fn get_search_lore(index: usize) -> String {
    let search_lore = [
        // TODO! find a better way to concat strings
        [format!("There is a {} {} {} {} {}{}", "staff".bold(), "by the bed. The only doors lead to the", "bathroom".bold(), "and the", "hallway".bold(), ".")],
        [format!("This is a bathroom connected to a {} {} {}{} {} {} {}", "bedroom.".bold(), "It houses a giant", "mirror".bold(), ".", "With your", "robe".bold(), "hanging off it.")],
        [format!("Doors connecting to many dorm rooms line this hallway.
Leaving the hallway leads to the rest of the {}{}", "school dorms".bold(), ".")]    
    ];
    search_lore[index][0].to_string()
}
