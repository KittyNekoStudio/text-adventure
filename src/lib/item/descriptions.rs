/// Get the descriptions or name of an item.
pub fn get_item_lore(index: (usize, usize, usize)) -> &'static str {
    match index.0 {
        1 => item_name((index.1, index.2)),
        2 => item_descriptions((index.1, index.2)),
        _ => "Error get item lore"
    }
}
/// Calls the item types description.
fn item_descriptions(index: (usize, usize)) -> &'static str {
    match index.0 {
        1 => staff_descriptions(index.1),
        2 => potion_descriptions(index.1),
        3 => sword_descriptions(index.1),
        4 => robe_descriptions(index.1),
        _ => "Error item descriptions"
    }
}
/// Calls the item types name.
fn item_name(index: (usize, usize)) -> &'static str {
    match index.0 {
        1 => staff_name(index.1),
        2 => potion_name(index.1),
        3 => sword_name(index.1),
        4 => robe_name(index.1),
        _ => "Error item name"
    }
}
fn staff_name(index: usize) -> &'static str {
    match index {
        1 => "Basic Staff",
        2 => "Strongest staff ever made",
        _ => "Non allowed index"
    }
}
fn sword_name(index: usize) -> &'static str {
    match index {
        1 => "Basic Sword",
        _ => "Non allowed index"
    }
}
fn potion_name(index: usize) -> &'static str {
    match index {
        1 => "Basic Health Potion",
        _ => "Non allowed index"
    }
}
fn robe_name(index: usize) -> &'static str {
    match index {
        1 => "Basic Mage Robe",
        _ => "Non allowed index"
    }
}
pub fn interactable_name(index: usize) -> &'static str {
    match index {
        1 => "Mirror",
        2 => "Your Bed",
        _ => "Non allowed index"
    }
}
fn staff_descriptions(index: usize) -> &'static str {
    match index {
        1 => "A staff that can be found anywhere. Nothing special.",
        2 => "How is it so strong",
        _ => "Non allowed index"
    }
}
fn sword_descriptions(index: usize) -> &'static str {
    match index {
        1 => "A piece of sharpened iron.",
        _ => "Non allowed index"
    }
}
fn potion_descriptions(index: usize) -> &'static str {
    match index {
        1 => "The cheapest of healing potions.",
        _ => "Non allowed index"
    }
}
fn robe_descriptions(index: usize) -> &'static str {
    match index {
        1 => "As much use as a bath robe.",
        _ => "Non allowed index"
    }
}
pub fn interactable_desctiptions(index: usize) -> &'static str {
    match index {
        1 => "It refects your beautiful face.",
        2 => "The place you slept in for the past 6 years. That changes today.",
        _ => "Non allowed index"
    }
}