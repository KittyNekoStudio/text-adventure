use crate::{entitys::entity::Entity, item::{def::AllItems, interactable::Interactable}};

/// A struct that makes all rooms one type.
#[derive(Debug, Clone, PartialEq)]
pub struct Area<'a> {
    pub room: Room<'a>
}

impl<'a> Area<'a> {
    pub fn new() -> Self {
        Self {
            room: Room::new()
        }
    }
}
/// A struct that holds the value of the room.
#[derive(Debug, Clone, PartialEq)]
pub struct Room<'a> {
    pub entitys: Vec<Entity<'a>>,
    pub collectable_item: Vec<(String, AllItems<'a>)>,
    pub interactable_items: Vec<(String, AllItems<'a>)>,
    pub main_area_name: String,
    pub sub_area_names: Vec<String>,
    pub id: usize
}

impl<'a> Room<'a> {
    pub fn new() -> Self {
        Self {
            entitys: vec![],
            collectable_item: vec![],
            interactable_items: vec![],
            main_area_name: String::from(""),
            sub_area_names: vec![],
            id: 0
        }
    }
    /// Gets the index to the interactable item.
    pub fn get_inter_index(&self, input: &String) -> usize {
        self
        .interactable_items
        .iter()
        .position(|i| input.contains(&i.0))
        .expect("Erorr at get inter index.")
    }
    /// Gets the index to the collectable item.
    pub fn get_collect_index(&self, input: &String) -> Option<usize> {
        Some(self
        .collectable_item
        .iter()
        .position(|i| input.contains(&i.0))?
        )
    }
    /// Removes and returns the provided item.
    pub fn get_item(&mut self, index: usize) -> (String, AllItems) {
        self.collectable_item.remove(index)
    }
}