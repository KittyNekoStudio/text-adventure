#[derive(Debug, Clone)]
/// The basic character
pub struct PlayerCharacter {
    // TODO! create a system able to index into a collection
    // based on the provided stat type 
    pub stats: (u16, u16, u16, u16, u16)
}

impl PlayerCharacter {
    pub fn new() -> Self {
        Self {
            stats: (1, 50, 5, 5, 5)
        }
    }
}