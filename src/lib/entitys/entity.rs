use crate::{item::def::{print_collectable, CollectableItem}, spells::spells::{print_circle, print_spell}};

#[derive(Debug, Clone, PartialEq)]
/// The basic character
pub struct Entity {
    pub spells: Vec<usize>,
    pub spell_circle: Vec<usize>,
    pub inventory: Vec<CollectableItem>
}
impl Entity {
    pub fn new() -> Self {
        Self {
            spells: vec![],
            spell_circle: vec![],
            inventory: vec![]
        }
    }
    /// Prints entity.
    pub fn print_entity(&self, num: usize) {
        match num {
            1 => for spell in &self.spells {
                print_spell(spell);
            },
            2 => for item in &self.inventory {
                print_collectable(*item);
            },
            3 => for circle in &self.spell_circle {
                print_circle(&circle)
            },
            _ => ()
        }
    }
    /// Adds spell.
    pub fn add_spell(&mut self, spell: usize) {
        self.spells.push(spell)
    }
}