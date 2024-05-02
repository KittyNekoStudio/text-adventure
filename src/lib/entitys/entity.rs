use crate::item::def::{print_collectable, CollectableItem};

#[derive(Debug, Clone, PartialEq)]
/// The basic character
pub struct Entity {
    pub stats: (u16, u16, u16, u16, u16),
    pub inventory: Vec<CollectableItem>
}
impl Entity {
    pub fn new() -> Self {
        Self {
            stats: (1, 50, 5, 5, 5),
            inventory: vec![]
        }
    }
    pub fn get_level(&self) -> u16 {
        self.stats.0
    }
    pub fn get_health(&self) -> u16 {
        self.stats.1
    }
    pub fn get_vit(&self) -> u16 {
        self.stats.2
    }
    pub fn get_int(&self) -> u16 {
        self.stats.3
    }
    pub fn get_speed(&self) -> u16 {
        self.stats.4
    }
    pub fn add_level(&mut self, num: u16) -> &Self {
        self.stats.0 += num;
        self
    }
    pub fn sub_level(&mut self, num: u16) -> &Self {
        self.stats.0 -= num;
        self
    }
    pub fn add_health(&mut self, num: u16) -> &Self {
        self.stats.1 += num;
        self
    }
    pub fn sub_health(&mut self, num: u16) -> &Self {
        self.stats.1 -= num;
        self
    }
    pub fn add_vit(&mut self, num: u16) -> &Self {
        self.stats.2 += num;
        self
    }
    pub fn sub_vit(&mut self, num: u16) -> &Self {
        self.stats.2 += num;
        self
    }
    pub fn add_int(&mut self, num: u16) -> &Self {
        self.stats.3 += num;
        self
    }
    pub fn sub_int(&mut self, num: u16) -> &Self {
        self.stats.3 -= num;
        self
    }
    pub fn add_speed(&mut self, num: u16) -> &Self {
        self.stats.4 += num;
        self
    }
    pub fn sub_speed(&mut self, num: u16) -> &Self {
        self.stats.4 -= num;
        self
    }
    pub fn print_entity(&self, num: usize) {
        match num {
            1 => println!("Level: {}, Health: {}, Vitality: {}, Intelligence: {}, Speed: {}",
            self.stats.0, self.stats.1, self.stats.2, self.stats.3, self.stats.4),
            2 => for item in &self.inventory {
                print_collectable(*item);
                println!("");
            },
            _ => ()
        }
    }

}