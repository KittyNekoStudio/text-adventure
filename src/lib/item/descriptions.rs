use colored::Colorize;

/// Get the descriptions or name of an item.
pub fn get_item_lore(index1: usize, index2: usize) -> String {
    let item_lore = [
        // 0
    ["Basic Staff".bold(), "A staff that can be found anywhere. Nothing special.".into()], 
        // 1
    ["Basic Health Potion".bold(), "The cheapest of healing potions.".into()],
        // 2
    ["Basic Mage Robe".bold(), "As much use as a bath robe.".into()],
        // 3
    ["Your Bed".bold(), "The place you slept in for the past 6 years. That changes today.".into()],
        // 4
    ["Mirror".bold(), "It refects your beautiful face.".into()],
        // 5
    ["Dorm Map".bold(), format!("{} {} {} {} {} {} {} {}", "West 1".bold(), "and", "West 2".bold(), "for the left hallways.",
"East 1".bold(), "and", "East 2".bold(), "for the right hallways.").into()],
        // 6
    ["School map".bold(), format!("To the north is the {}{} {} {}{} {} {}{}", "school entrance".bold(), ",", "to the south is the", "school dorms".bold(), ",", "to the west is the", "school auditorium".bold(), ".").into()]
    ];
    item_lore[index1][index2].to_string()
}
/// Get the descriptions or name of the room.
pub fn get_room_lore(index1: usize, index2: usize) -> String {
    let room_lore = [
        // 0
    ["Bedroom".bold(), "The dorm looks clean but empty. Seems like the owner isn't home often.".into()],
        // 1
    ["Bathroom".bold(), "A small dorm bathroom.".into()],
        // 2  
    ["E1 Hallway".bold(), "The eastside 1 hallway that connects dorms together.".into()],
        // 3
    ["E1 Dorm Room".bold(), "Why are you in a strangers room, you freak.".into()],
        // 4
    ["School Dorms".bold(), "The main hub to get to all the dorms.".into()],
        // 5
    ["E2 Hallway".bold(), "The eastside 2 hallway that connects dorms together.".into()],
        // 6
    ["E2 Dorm Room".bold(), "Why are you in a strangers room, you freak.".into()],
        // 7
    ["W1 Hallway".bold(), "The westside 1 hallway that connects dorms together.".into()],
        // 8
    ["W1 Dorm Room".bold(), "Why are you in a strangers room, you freak.".into()],
        // 9
    ["W2 Hallway".bold(), "The westside 2 hallway that connects dorms together.".into()],
        // 10
    ["W2 Dorm Room".bold(), "Why are you in a strangers room, you freak.".into()],
        // 11
    ["Dorm Office".bold(), "An office for the dorm manager.".into()],
        // 12
    ["Campus Square".bold(), "The center of the campus.".into()]
    ];
    room_lore[index1][index2].to_string()
}

pub fn get_search_lore(index: usize) -> String {
    let search_lore = [
        // TODO! find a better way to concat strings
            // 0
        [format!("There is a {} {} {}{} {} {} {}{}", "staff".bold(), "by the", "bed".bold(), ". The only doors lead to the", "bathroom".bold(), "and the", "hallway".bold(), ".")],
            // 1
        [format!("This is a bathroom connected to a {} {} {}{} {} {}", "bedroom.".bold(), "It houses a giant", "mirror".bold(), ". With your", "robe".bold(), "hanging off it.")],
            // 2
        [format!("Doors connecting to many {} {} {}{}", "dorm rooms".bold(), "line this hallway.
Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
            // 3
        [format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "east 1 hallway".bold(), ".")],
            // 4
        [format!("There is an {} {} {} {} {} {}{}", "office".bold(), "with a person inside. A", "map".bold(), "of the dorms covers the right wall.", " 
Leaving the dorms leads to the", "campus square".bold(), ".")],
            // 5
        [format!("Doors connecting to many {} {} {}{}", "dorm rooms".bold(), "line this hallway.
Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
            // 6
        [format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "east 2 hallway".bold(), ".")],
            // 7
        [format!("Doors connecting to many {} {} {}{}", "dorm rooms".bold(), "line this hallway.
Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
            // 8
        [format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "west 1 hallway".bold(), ".")],
            // 9
        [format!("Doors connecting to many {} {} {}{}", "dorm rooms".bold(), "line this hallway.
Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
            // 10
        [format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "west 2 hallway".bold(), ".")],
            // 11
        [format!("The office has a tidy desk with the {} {} {}{}", "manager".bold(), "sitting behind it.
Vibrant plants and tall file cabinets line the walls. Leaving leads back to the", "school dorms".bold(), ".")],
            // 12
        [format!("Students are rushing through the square. Some hopping, some flying.
There is a big board to the north of the square. It has a {} {}", "map".bold(), "on it's front.")]
    ];
    search_lore[index][0].to_string()
}
