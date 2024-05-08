use super::entity::Entity;

#[derive(Debug, Clone, PartialEq)]
pub struct BasicNPC {
    pub entity: Entity,
    pub name: String,
    pub dialogue: Vec<String>,
    pub talked_to: usize
}

impl BasicNPC {
    pub fn new() -> Self {
        Self {
            entity: Entity {
                stats: (0, 0, 0, 0, 0),
                inventory: vec![]
            },
            name: "".to_string(),
            dialogue: vec![],
            talked_to: 0
        }
    }
}