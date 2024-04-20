use crate::stats::character_stats::{Health, Intelligence, Level, Speed, Vitality};
#[derive(Debug)]
/// The basic character
pub struct PlayerCharacter {
    // TODO! create a system able to index into a collection
    // based on the provided stat type 
    pub stats: (Level, Health, Intelligence, Vitality, Speed)
}

impl PlayerCharacter {
    pub fn new() -> Self {
        Self {
            stats: (1, 50, 5, 5, 5)
        }
    }
}