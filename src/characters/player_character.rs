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
    pub fn update_level(mut self, operation: &str, num: u16) -> Self {
        if operation == "+" {
            self.stats.0 = self.stats.0 + num
        } else if operation == "-" {
            self.stats.0 = self.stats.0 - num
        }
        self
    }
    pub fn update_health(mut self, operation: &str, num: u16) -> Self {
        if operation == "+" {
            self.stats.1 += num
        } else if operation == "-" {
            self.stats.1 -= num
        }
        self
    }
    pub fn update_vit(mut self, operation: &str, num: u16) -> Self {
        if operation == "+" {
            self.stats.2 = self.stats.2 + num
        } else if operation == "-" {
            self.stats.2 = self.stats.2 - num
        }
        self
    }
    pub fn update_int(mut self, operation: &str, num: u16) -> Self {
        if operation == "+" {
            self.stats.3 = self.stats.3 + num
        } else if operation == "-" {
            self.stats.3 = self.stats.3 - num
        }
        self
    }
    pub fn update_speed(mut self, operation: &str, num: u16) -> Self {
        if operation == "+" {
            self.stats.4 = self.stats.4 + num
        } else if operation == "-" {
            self.stats.4 = self.stats.4 - num
        }
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