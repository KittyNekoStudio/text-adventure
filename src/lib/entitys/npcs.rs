use super::entity::Entity;

#[derive(Debug, Clone, PartialEq)]
/// The struct that defines non player chracters.
pub struct NPC {
    pub entity: Entity,
    pub name: String,
    pub dialogue: Vec<String>,
    pub talked_to: usize
}

impl NPC {
    pub fn new() -> Self {
        Self {
            entity: Entity {
                spells: vec![],
                spell_circle: vec![],
                inventory: vec![],
                mana: 0
            },
            name: "".to_string(),
            dialogue: vec![],
            talked_to: 0
        }
    }
}