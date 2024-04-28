use crate::item::{def::AllItems, held_item::Staff, interactable::Interactable};

use super::def::{Area, Room};
#[derive(Debug, Clone, PartialEq)]
pub struct FirstRoom {
    pub area: Area

}

impl FirstRoom {
    pub fn new() -> Self {
    // TODO! create a way to move this into a function
            Self {
                area: Area {
                room: Room {
                entitys: vec![],
                collectable_item: vec![AllItems::Staff(Staff::new())],
                interactable_items: vec![Interactable::new().change_desc_id(2)],
                interactable_item_name: vec![String::from("bed")],
                main_area_name: String::from("hallway"),
                sub_area_names: vec![String::from("bathroom")],
                id: 1
                }
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Bathroom {
    pub area: Area
}

impl Bathroom {
    pub fn new() -> Self {
            Self {
                area: Area {
                room: Room {
                entitys: vec![],
                collectable_item: vec![],
                interactable_items: vec![Interactable::new()],
                interactable_item_name: vec![String::from("mirror")],
                main_area_name: String::from("room"),
                sub_area_names: vec![],
                id: 2
                }
            }
        }
    }
}

pub fn check_item(item: &String) -> bool {
    let fields = ["staff"];
    for i in fields {
        if item == i {
            return true;
        }
    }
    return false;
}

pub fn choose_item(item: &String) -> usize {
    match item as &str {
        "staff" => 0,
        _ => 10
    }
}