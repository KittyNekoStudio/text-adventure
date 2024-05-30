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
    ["Mirror".bold(), "You look into the mirror and see.
A man with short well kept brown hair.
With bright blue eyes and a clean shaven face.
He's not tall nor short looking average.
His most stiking feature is the mysterious look in his eyes.".into()],
        // 5
    ["Dorm Map".bold(), format!("{} {} {} {} {} {} {} {}", "West 1".bold(), "and", "West 2".bold(), "for the left hallways.",
"East 1".bold(), "and", "East 2".bold(), "for the right hallways.").into()],
        // 6
    ["School Map".bold(), format!("To the north is the {}{} {} {}{} {} {}{}", "school entrance".bold(), ",", "to the south is the", "school dorms".bold(), ",", "to the west is the", "auditorium".bold(), ".").into()],
        // 7
    ["Trash Can".bold(), format!("This trash can is full of trash.").into()]
    ];
    item_lore[index1][index2].to_string()
}
/// Get the descriptions or name of the room.
pub fn get_room_lore(index1: usize, index2: usize) -> String {
    let room_lore = [
        // 0
    ["Bedroom", "The dorm looks clean but empty. Seems like the owner isn't home often."],
        // 1
    ["Bathroom", "A small dorm bathroom."],
        // 2  
    ["E1 Hallway", "The eastside 1 hallway that connects dorms together."],
        // 3
    ["E1 Dorm Room", "Why are you in a strangers room, you freak."],
        // 4
    ["School Dorms", "The main hub to get to all the dorms."],
        // 5
    ["E2 Hallway", "The eastside 2 hallway that connects dorms together."],
        // 6
    ["E2 Dorm Room", "Why are you in a strangers room, you freak."],
        // 7
    ["W1 Hallway", "The westside 1 hallway that connects dorms together."],
        // 8
    ["W1 Dorm Room", "Why are you in a strangers room, you freak."],
        // 9
    ["W2 Hallway", "The westside 2 hallway that connects dorms together."],
        // 10
    ["W2 Dorm Room", "Why are you in a strangers room, you freak."],
        // 11
    ["Dorm Office", "An office for the dorm manager."],
        // 12
    ["Campus Square", "The center of the campus."],
        // 13
    ["School Auditorium", "An auditorium used for speeches and announcements."],
        // 14
    ["Stage", "A stage in the school auditorium"],
        // 15
    ["Seat", "A seat in the school auditorium"],
        // 16
    ["School Entrance", "The entrance to the school."]
    ];
    room_lore[index1][index2].to_string()
}
/// Get more information of a room.
pub fn get_search_lore(index: usize) -> String {
    let search_lore = [
        // TODO! find a better way to concat strings
            // 0
        [format!("With the room as empty as it is, the only things that stand out are the {} {} {}{} 
{} {} {} {}{}", "staff".bold(), "by the", "bed".bold(), ",", "and the doors that lead to the", "bathroom".bold(), "and the", "hallway".bold(), ".")],
            // 1
        [format!("This is the bathroom connected to your {} 
{} {}{} {} {}", "bedroom.".bold(), "It's nothing special except the giant", "mirror".bold(), ", with your", "robe".bold(), "hanging off it.")],
            // 2
        [format!("This is a hallway conecting many {} {}
{}
{} {}{}", "dorm rooms".bold(), "together.", "It is more clean than one would think for a dorm hallway.", "Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
            // 3
        [format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "east 1 hallway".bold(), ".")],
            // 4
        [format!("There are students coming in an out of the entrance. 
{} {} {} 
{} {} {}
{} {}{}", "There is an", "office".bold(), "with a person inside to the left.", "A", "map".bold(), "of the dorms covers the right wall.", "Leaving the dorms leads to the", "campus square".bold(), ".")],
            // 5
        [format!("Doors connecting to many {} {} 
{} {}{}", "dorm rooms".bold(), "line this hallway.", "Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
            // 6
        [format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "east 2 hallway".bold(), ".")],
            // 7
        [format!("Doors connecting to many {} {} 
{} {}{}", "dorm rooms".bold(), "line this hallway.", "Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
            // 8
        [format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "west 1 hallway".bold(), ".")],
            // 9
        [format!("Doors connecting to many {} {} 
{} {}{}", "dorm rooms".bold(), "line this hallway.", "Leaving the hallway leads to the rest of the", "school dorms".bold(), ".")],
            // 10
        [format!("Similar to your room there is a {} {} {}{}", "bathroom".bold(), "and the room leads out into the", "west 2 hallway".bold(), ".")],
            // 11
        [format!("The office has a tidy desk with the {} {}
{} {}{}", "manager".bold(), "sitting behind it.", "Vibrant plants and tall file cabinets line the walls. Leaving leads back to the", "school dorms".bold(), ".")],
            // 12
        [format!("Students are rushing through the square. Some hopping, some flying.
{} {} {}", "There is a big board to the north of the square. It has a", "map".bold(), "on it's front.")],
            // 13
        [format!("All students and faculty are gathered here. Parents have shown up as well. 
{} 
{} {} {} {}{}", "There is a stage up front covered by a curtain,", "with rows upon rows of", "seats".bold(), "in front of the", "stage".bold(), ".")],
            // 14
        [format!("On the stage is the headmaster giving the scrolls, faculty are standing behind the headmaster.
{} {}{}", "Walk down the stairs to go back to the", "auditorium".bold(), ".")],
            // 15
        [format!("It's just a seat in an auditorium. The only special feature of the seat is it's cleanliness.")],
            // 16
        [format!("The entrance is filled with people. They are coming in an out of the {}{}
{} {} {}
{} {}{}", "gate".bold(), ".", "There is a", "receptionist".bold(), "on the left side.", "On the right is a singing", "trash can".bold(), ".")]
    ];
    search_lore[index][0].to_string()
}
