use crate::{entitys::entity::Entity, item::{def::AllItems, interactable::Interactable}};

use super::first_room::choose_item;
/// A struct that makes all rooms one type.
#[derive(Debug, Clone, PartialEq)]
pub struct Area {
    pub room: Room
}

impl Area {
    pub fn new() -> Self {
        Self {
            room: Room::new()
        }
    }
}
/// A struct that holds the value of the room.
#[derive(Debug, Clone, PartialEq)]
pub struct Room {
    pub entitys: Vec<Entity>,
    pub collectable_item: Vec<AllItems>,
    pub interactable_items: Vec<Interactable>,
    pub interactable_item_name: Vec<String>, 
    pub main_area_name: String,
    pub sub_area_names: Vec<String>,
    pub id: usize
}

impl Room {
    pub fn new() -> Self {
        Self {
            entitys: vec![],
            collectable_item: vec![],
            interactable_items: vec![],
            interactable_item_name: vec![],
            main_area_name: String::from(""),
            sub_area_names: vec![],
            id: 0
        }
    }
    pub fn get_item(&mut self, item: &String) -> AllItems {
        self.collectable_item.remove(choose_item(item))
    }
}