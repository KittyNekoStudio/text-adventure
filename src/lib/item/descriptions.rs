use colored::Colorize;

/// Get the descriptions or name of an item.
pub fn get_item_lore(index1: usize, index2: usize) -> String {
    let item_lore = [
    ["Basic Staff".bold(), "A staff that can be found anywhere. Nothing special.".into()], 
    ["Basic Health Potion".bold(), "The cheapest of healing potions.".into()],
    ["Basic Mage Robe".bold(), "As much use as a bath robe.".into()],
    ["Your Bed".bold(), "The place you slept in for the past 6 years. That changes today.".into()],
    ["Mirror".bold(), "It refects your beautiful face.".into()],
    ["Dorm Map".bold(), format!("{} {} {} {} {} {} {} {}", "West 1".bold(), "and", "West 2".bold(), "for the left hallways.",
"East 1".bold(), "and", "East 2".bold(), "for the right hallways.").into()]
    ];
    item_lore[index1][index2].to_string()
}
/// Get the descriptions or name of the room.
pub fn get_room_lore(index1: usize, index2: usize) -> String {
    let room_lore = [
    ["Bedroom".bold(), "The dorm looks clean but empty. Seems like the owner isn't home often.".into()],
    ["Bathroom".bold(), "A small dorm bathroom.".into()],
    ["E1 Hallway".bold(), "The eastside 1 hallway that connects dorms together.".into()],
    ["E1 Dorm Room".bold(), "Why are you in a strangers room, you freak.".into()],
    ["School Dorms".bold(), "The main hub to get to all the dorms.".into()],
    ["E2 Hallway".bold(), "The eastside 2 hallway that connects dorms together.".into()],
    ["E2 Dorm Room".bold(), "Why are you in a strangers room, you freak.".into()],
    ["W1 Hallway".bold(), "The westside 1 hallway that connects dorms together.".into()],
    ["W1 Dorm Room".bold(), "Why are you in a strangers room, you freak.".into()],
    ["W2 Hallway".bold(), "The westside 2 hallway that connects dorms together.".into()],
    ["W2 Dorm Room".bold(), "Why are you in a strangers room, you freak.".into()],
    ["Dorm Office".bold(), "An office for the dorm manager.".into()]
    ];
    room_lore[index1][index2].to_string()
}

pub fn get_search_lore(index: usize) -> String {
    let search_lore = [
        // TODO! find a better way to concat strings
[format!("There is a {} {} {}{} {} {} {}{}", "staff".bold(), "by the", "bed".bold(), ". The only doors lead to the", "bathroom".bold(), "and the", "hallway".bold(), ".")],
[format!("This is a bathroom connected to a {} {} {}{} {} {}", "bedroom.".bold(), "It houses a giant", "mirror".bold(), ". With your", "robe".bold(), "hanging off it.")],
[format!("Doors connecting to many {} {} {}{}", "dorm rooms".bold(), "line this hallway.
Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
[format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "east 1 hallway".bold(), ".")],
[format!("There is an {} {} {} {}", "office".bold(), "with a person inside. A", "map".bold(), "of the dorms covers the right wall.")],
[format!("Doors connecting to many {} {} {}{}", "dorm rooms".bold(), "line this hallway.
Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
[format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "east 2 hallway".bold(), ".")],
[format!("Doors connecting to many {} {} {}{}", "dorm rooms".bold(), "line this hallway.
Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
[format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "west 1 hallway".bold(), ".")],
[format!("Doors connecting to many {} {} {}{}", "dorm rooms".bold(), "line this hallway.
Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
[format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "west 2 hallway".bold(), ".")],
[format!("The office has a tidy dest with the {} {}", "manager".bold(), "sitting behind it. With vibrant plants and tall file cabinets lining the walls.")]
    ];
    search_lore[index][0].to_string()
}
