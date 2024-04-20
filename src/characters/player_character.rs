use crate::stats::character_stats::{Health, Intelligence, Level, Speed, Vitality};

pub struct PlayerCharacter {
    level: Level,
    health: Health,
    int: Intelligence,
    vitality: Vitality,
    speed: Speed
}

impl PlayerCharacter {
    pub fn new() -> Self {
        Self {
            level: 1,
            health: 50,
            int: 5,
            vitality: 5,
            speed: 5
        }
    }
}