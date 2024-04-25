use crate::{entitys::{entity::Entity, player_character::PlayerCharacter}, map::map::Map};

pub const NORTH: &str = "north";
pub const EAST: &str = "east";
pub const SOUTH: &str = "south";
pub const WEST: &str= "west";
pub const CARDINALS: [&str; 4] = [NORTH, EAST, SOUTH, WEST];
#[derive(Debug, Clone)]
pub struct GameState {
    pub map: Map,
    pub movement: bool,
    pub player: PlayerCharacter,
    pub all_entitys: Vec<Entity>
}

impl GameState {
    pub fn new() -> Self {
        Self {
            map: Map::new(),
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
    pub fn push_movement(&mut self, direction: usize) -> &Self {
        if self.movement == true {
            self.map.push(direction);
        }
        self
    }
}

