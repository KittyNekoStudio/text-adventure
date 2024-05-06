use colored::Colorize;

use crate::{area::{all_rooms::{Bathroom, DormRoom, FirstRoom, Hallway, SchoolDorm}, def::Area}, def::recive_input, entitys::{def::{check_entity_field, match_entity_field}, entity::Entity, player_character::PlayerCharacter}, history::{history::History, movement::{check_room, get_area, move_to_room}}, item::{descriptions::{get_room_lore, get_search_lore}, item_interaction::item_interaction}};
#[derive(Debug, Clone, PartialEq)]
pub struct GameState {
    pub history: History,
    pub current_area: Area,
    pub previous_area: Area,
    pub all_areas: [Area; 11],
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
            all_areas: [
            FirstRoom::new().area,
            Bathroom::new().area,
            Hallway::new_e1().area,
            DormRoom::new_e1().area,
            SchoolDorm::new().area,
            Hallway::new_e2().area,
            DormRoom::new_e2().area,
            Hallway::new_w1().area,
            DormRoom::new_w1().area,
            Hallway::new_w2().area,
            DormRoom::new_w2().area
            ],
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
            let item = self.current_area.room.get_item(index.expect("Error pickup."));
            println!("{}", format!("You collected a {}.", item.0.as_str().bold()));
            self.player.entity.inventory.push(item.1);
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
    // TODO! change how to check the dorm room
    // so it doesn't have to be in default_state()
    if self.dorm_room_check(&input) {
        println!("");
        println!("The bathroom is locked.");
        return true;
    }
    if input == "search" {
        println!("");
        self.print_search();
        return true;
    } else if check_room(&input, &self.current_area) {
        println!("");
        self.push_movement(&input);
        move_to_room(self);
        self.add_entered();
        self.print_room();
        self.update_area();
        return true;
    } else if check_entity_field(&input) {
        println!("");
        self.player.entity.print_entity(match_entity_field(&input));
        return true;
    } else if interaction == 1 {
        if self.current_area.room.collectable_item.len() == 0 {
            println!("");
            println!("Nothing matches with what you typed.");
            return true;
        }
        println!("");
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
        println!("");
        println!("Nothing matches with what you typed.");
        return true;
    }
  }
}
// In a different block for readability.
impl GameState {
    /// Check if second scene can be played.
    pub fn second_check(&self) -> bool {
        let mut hallway = Hallway::new_e1();
        hallway.area.room.times_entered = 1;
        if self.current_area == hallway.area {return true}
        else {return false}
    }
    /// If player types bathroom in a dorm room return true.
    // TODO! change code logic so this check doesn't need to be made.
    pub fn dorm_room_check(&mut self, input: &String) -> bool {
        let mut dorm_e1 = self.all_areas[3].clone();
        dorm_e1.room.times_entered += 1;
        let mut dorm_e2 = self.all_areas[6].clone();
        dorm_e2.room.times_entered += 1;
        let mut dorm_w1 = self.all_areas[8].clone();
        dorm_w1.room.times_entered += 1;
        let mut dorm_w2 = self.all_areas[10].clone();
        dorm_w2.room.times_entered += 1;
        let dorms = [
        dorm_e1,
        dorm_e2,
        dorm_w1,
        dorm_w2
        ];
        for i in dorms {
        if self.current_area == i && input == &"bathroom".to_string() {
            return true;
        }
        }
        return false;
    }
}