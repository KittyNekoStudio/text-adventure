use crate::item::{def::AllItems, held_item::Staff, interactable::Interactable, wearable::Robe};

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
                collectable_item: vec![(String::from("staff"), AllItems::Staff(Staff::new()))],
                interactable_items: vec![(String::from("bed"), Interactable::new().change_desc_id(2))],
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
                collectable_item: vec![(String::from("robe"), AllItems::Robe(Robe::new()))],
                interactable_items: vec![(String::from("mirror"), Interactable::new())],
                main_area_name: String::from("room"),
                sub_area_names: vec![],
                id: 2
                }
            }
        }
    }
}