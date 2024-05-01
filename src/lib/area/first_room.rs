use crate::item::{def::AllItems, held_item::{Staff, STAFF}, interactable::{Interactable, BED, MIRROR}, wearable::{Robe, ROBE}};

use super::def::{Area, Room};
#[derive(Debug, Clone, PartialEq)]
pub struct FirstRoom<'a> {
    pub area: Area<'a>

}

impl<'a> FirstRoom<'a> {
    pub fn new() -> Self {
    // TODO! create a way to move this into a function
            Self {
                area: Area {
                room: Room {
                entitys: vec![],
                collectable_item: vec![(String::from("staff"), AllItems::HeldItem(STAFF))],
                interactable_items: vec![(String::from("bed"), AllItems::InteractableItem(BED))],
                main_area_name: String::from("hallway"),
                sub_area_names: vec![String::from("bathroom")],
                id: 1
                }
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Bathroom<'a> {
    pub area: Area<'a>
}

impl<'a> Bathroom<'a> {
    pub fn new() -> Self {
            Self {
                area: Area {
                room: Room {
                entitys: vec![],
                collectable_item: vec![(String::from("robe"), AllItems::WearableItem(ROBE))],
                interactable_items: vec![(String::from("mirror"), AllItems::InteractableItem(MIRROR))],
                main_area_name: String::from("room"),
                sub_area_names: vec![],
                id: 2
                }
            }
        }
    }
}