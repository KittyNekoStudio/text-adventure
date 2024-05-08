use crate::{entitys::npcs::BasicNPC, item::def::{CollectableItem, InteractableItem}};

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
    pub entitys: Vec<BasicNPC>,
    pub collectable_item: Vec<(String, CollectableItem)>,
    pub interactable_items: Vec<(String, InteractableItem)>,
    pub main_area_name: String,
    pub sub_area_names: Vec<String>,
    pub lore: usize,
    pub times_entered: usize
}

impl Room {
    pub fn new() -> Self {
        Self {
            entitys: vec![],
            collectable_item: vec![],
            interactable_items: vec![],
            main_area_name: String::from(""),
            sub_area_names: vec![],
            lore: 0,
            times_entered: 0
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
    pub fn get_item(&mut self, index: usize) -> (String, CollectableItem) {
        self.collectable_item.remove(index)
    }
    pub fn get_npc_index(&self, input: &String) -> Option<usize> {
        Some(self
        .entitys
        .iter()
        .position(|i| input.contains(&i.name))?
        )
    }
    /// Increments talked_to
    pub fn add_talked_to(&mut self, index: usize) {
        self.entitys[index].talked_to += 1;
    }
}