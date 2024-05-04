use crate::{area::{all_rooms::{Bathroom, FirstRoom, Hallway}, def::Area}, def::recive_input, entitys::{def::{check_entity_field, match_entity_field}, entity::Entity, player_character::PlayerCharacter}, history::{history::History, movement::{check_room, get_area, move_to_room}}, item::{descriptions::{get_room_lore, get_search_lore}, item_interaction::item_interaction}};
#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub history: History,
    pub current_area: Area,
    pub previous_area: Area,
    pub all_areas: [Area; 3],
    pub scenes_completed: [bool; 3],
    pub movement: bool,
    pub store: bool,
    pub player: PlayerCharacter,
    pub all_entitys: Vec<Entity>
}

impl GameState {
    pub fn new() -> Self {
        Self {
            history: vec!["hi".to_string()],
            current_area: Bathroom::new().area,
            // TODO! change this to a gloal variable
            previous_area: Area::new(),
            all_areas: [FirstRoom::new().area, Bathroom::new().area, Hallway::new().area],
            scenes_completed: [true, true, false],
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
    /// Returns time entered.
    pub fn times_entered(&self, index: usize) -> usize {
        self.all_areas[index].room.times_entered
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
        let (_, previous_area) = (rev.next(), rev.next().expect("Update area Err"));
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
    /// Prints the room lore.
    pub fn print_room(&self) {
        println!("{}", get_room_lore(self.current_area.room.lore, 0));
        println!("{}", get_room_lore(self.current_area.room.lore, 1));
    }
    /// Prints the clues a room has.
    pub fn print_search(&self) {
        println!("{}", get_search_lore(self.current_area.room.lore))
    }
    /// Adds to time entered room
    pub fn add_entered(&mut self) -> &Self {
        self.current_area.room.times_entered += 1;
        self
    }
    /// Sets a scene to complete.
    pub fn complete_scene(&mut self, index: usize) -> &Self {
        self.scenes_completed[index] = true;
        self
    }
}
// In a different block for readability.
impl GameState {
    pub fn default_state(&mut self) -> bool {
    let input = recive_input().to_lowercase();
    let interaction = item_interaction(&input, self);
    if input == "search" {
        self.print_search();
        return true;
    } else if check_room(&input, &self.current_area) {
        self.push_movement(&input);
        move_to_room(self);
        self.add_entered();
        self.print_room();
        self.update_area();
        return true;
    } else if check_entity_field(&input) {
        self.player.entity.print_entity(match_entity_field(&input));
        return true;
    } else if interaction == 1 {
        if self.current_area.room.collectable_item.len() == 0 {
            println!("Nothing matches with what you typed.");
            return true;
        }
        self.pickup_item(self.get_collect_index(&input));
        if !self.store {
            println!("Nothing matches with what you typed.");
            return true;
        }
        return true;
    }
    if input == String::from("quit") {
        std::process::exit(0);
    } else {
        if interaction == 0 {return true;}
        println!("Nothing matches with what you typed.");
        return true;
    }
  }
}
// In a different block for readability.
impl GameState {
    /// Check if second scene can be played.
    pub fn second_check(&self) -> bool {
        let mut hallway = Hallway::new();
        hallway.area.room.times_entered = 1;
        if self.current_area == hallway.area {return true}
        else {return false}
    }
}