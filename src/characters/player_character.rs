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
    pub fn add_level(mut self, num: u16) -> Self {
        self.stats.0 += num;
        self
    }
    pub fn sub_level(mut self, num: u16) -> Self {
        self.stats.0 -= num;
        self
    }
    pub fn add_health(mut self, num: u16) -> Self {
        self.stats.1 += num;
        self
    }
    pub fn sub_health(mut self, num: u16) -> Self {
        self.stats.1 -= num;
        self
    }
    pub fn add_vit(mut self, num: u16) -> Self {
        self.stats.2 += num;
        self
    }
    pub fn sub_vit(mut self, num: u16) -> Self {
        self.stats.2 += num;
        self
    }
    pub fn add_int(mut self, num: u16) -> Self {
        self.stats.3 += num;
        self
    }
    pub fn sub_int(mut self, num: u16) -> Self {
        self.stats.3 -= num;
        self
    }
    pub fn add_speed(mut self, num: u16) -> Self {
        self.stats.4 += num;
        self
    }
    pub fn sub_speed(mut self, num: u16) -> Self {
        self.stats.4 -= num;
        self
    }
    pub fn get_level(self) -> u16 {
        self.stats.0
    }
    pub fn get_health(self) -> u16 {
        self.stats.1
    }
    pub fn get_vit(self) -> u16 {
        self.stats.2
    }
    pub fn get_int(self) -> u16 {
        self.stats.3
    }
    pub fn get_speed(self) -> u16 {
        self.stats.4
    }
}