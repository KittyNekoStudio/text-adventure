use crate::item::def::{EVERYCOLITEM, EVERYINTITEM};

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
                collectable_item: vec![(String::from("staff"), EVERYCOLITEM[0])],
                interactable_items: vec![(String::from("bed"), EVERYINTITEM[0])],
                main_area_name: String::from("hallway"),
                sub_area_names: vec![String::from("bathroom")],
                id: 1,
                lore: 0
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
                collectable_item: vec![(String::from("robe"), EVERYCOLITEM[2])],
                interactable_items: vec![(String::from("mirror"), EVERYINTITEM[1])],
                main_area_name: String::from("bedroom"),
                sub_area_names: vec![],
                id: 2,
                lore: 1
                }
            }
        }
    }
}