use crate::item::Item;

pub struct Player {
    name: String,
    current_health: u32,
    max_health: u32,
    strength: u32,
    gold: u32,
    inventory: Vec<Item>,
    level: u32,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            current_health: 100,
            max_health: 100,
            strength: 10,
            gold: 0,
            inventory: Vec::new(),
            level: 1,
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_current_health(&self) -> u32 {
        self.current_health
    }

    pub fn get_max_health(&self) -> u32 {
        self.max_health
    }

    pub fn get_strength(&self) -> u32 {
        self.strength
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn add_gold(&mut self, gold: u32) {
        self.gold += gold;
    }

    pub fn reduce_current_health(&mut self, damage: u32) {
        self.current_health = if damage > self.current_health {
            0
        } else {
            self.current_health - damage
        };
    }

    pub fn is_dead(&self) -> bool {
        self.current_health == 0
    }
}
