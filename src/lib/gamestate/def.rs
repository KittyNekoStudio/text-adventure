use crate::{area::{def::Area, first_room::{Bathroom, FirstRoom}}, entitys::{entity::Entity, player_character::PlayerCharacter}, history::{history::History, movement::get_area}};
#[derive(Debug, Clone)]
pub struct GameState {
    pub history: History,
    pub current_area: Area,
    pub previous_area: Area,
    pub all_areas: [Area; 2],
    pub movement: bool,
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
            player: PlayerCharacter::new(),
            all_entitys: vec![]
        }
    }
    pub fn lock_movement(&mut self) -> &Self {
        self.movement = false;
        self
    }
    pub fn unlock_movement(&mut self) -> &Self {
        self.movement = true;
        self
    }
    pub fn push_movement(&mut self, room: &String) -> &Self {
        if self.movement == true {
            self.history.push(room.to_string());
        }
        self
    }
    pub fn push_prev_area(&mut self) -> &Self {
        self.previous_area = self.current_area.clone();
        self
    }
    pub fn update_area(&mut self) -> &Self {
        let mut rev = self.history.iter().rev();
        let (_, previous_area) = (rev.next(), rev.next().expect("Previous area Err"));
        self.all_areas[get_area(previous_area)] = 
            self
            .previous_area
            .clone();
        self
    }
    // TODO! fix the panic of when the vec runs out of items
    // it panics when you type the item in again
    pub fn pickup_item(&mut self, item: &String) -> &Self {
        self.player.entity.inventory.push(self.current_area.room.get_item(item));
        self
    }
}

