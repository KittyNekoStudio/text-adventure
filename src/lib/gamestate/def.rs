use crate::{area::{def::Area, first_room::{Bathroom, FirstRoom}}, entitys::{entity::Entity, player_character::PlayerCharacter}, history::{history::History, movement::get_area}};
#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub history: History,
    pub current_area: Area,
    pub previous_area: Area,
    pub all_areas: [Area; 2],
    pub movement: bool,
    pub store: bool,
    pub player: PlayerCharacter,
    pub all_entitys: Vec<Entity>
}

impl GameState {
    pub fn new() -> Self {
        Self {
            history: vec!["hi".to_string()],
            current_area: FirstRoom::new().area,
            // TODO! change this to a gloal variable
            previous_area: Area::new(),
            all_areas: [FirstRoom::new().area, Bathroom::new().area],
            movement: true,
            store: true,
            player: PlayerCharacter::new(),
            all_entitys: vec![]
        }
    }
    /// Player can no longer move.
    pub fn lock_movement(&mut self) -> &Self {
        self.movement = false;
        self
    }
    /// Player can move.
    pub fn unlock_movement(&mut self) -> &Self {
        self.movement = true;
        self
    }
    /// Player can not pick up items.
    pub fn lock_inventory(&mut self) -> &Self {
        self.store = false;
        self
    }
    /// Player can pick up items.
    pub fn unlock_invnetory(&mut self) -> &Self {
        self.store = true;
        self
    }
    /// Checks if player can move before pushing to history.
    pub fn push_movement(&mut self, room: &String) -> &Self {
        if self.movement {
            self.history.push(room.to_string());
        }
        self
    }
    /// Previous area becomes current area before swtiching to a new area.
    pub fn push_prev_area(&mut self) -> &Self {
        self.previous_area = self.current_area.clone();
        self
    }
    /// Takes the previous area and updates the stored area
    /// to be equal to the area the palyer has been through. 
    pub fn update_area(&mut self) -> &Self {
        let mut rev = self.history.iter().rev();
        let (_, previous_area) = (rev.next(), rev.next().expect("Previous area Err"));
        self.all_areas[get_area(previous_area)] = 
            self
            .previous_area
            .clone();
        self
    }
    /// Picks up the provided item.
    pub fn pickup_item(&mut self, index: Option<usize>) -> &Self {
        if index == None {
            self.store = false;
        } else {
            self.store = true;
        }
        if self.store {
            self.player.entity.inventory.push(self.current_area.room.get_item(index.expect("Error pickup.")).1);
        }
        self
    }
    /// Gets the index to the interactable item.
    pub fn get_inter_index(&self, input :&String) -> usize {
        self.current_area.room.get_inter_index(input)
    }
    /// Gets the index to the collectable item.
    pub fn get_collect_index(&self, input: &String) -> Option<usize> {
        self.current_area.room.get_collect_index(input)
    }
}

