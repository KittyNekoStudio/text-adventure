use super::def::InteractableItem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interactable {
    pub desc_id: usize
}

pub const MIRROR: InteractableItem = InteractableItem {
    name: "Mirror",
    description: "It refects your beautiful face."
};

pub const BED: InteractableItem = InteractableItem {
    name: "Your Bed",
    description: "The place you slept in for the past 6 years. That changes today."
};

impl Interactable {
    pub fn new() -> Self {
        Self {
            desc_id: 1
        }
    }
    pub fn change_desc_id(&mut self, num: usize) -> Self {
        self.desc_id = num;
        *self
    }
}