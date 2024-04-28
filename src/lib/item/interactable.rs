#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interactable {
    pub desc_id: usize
}

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