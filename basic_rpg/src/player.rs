
struct Item {
  name: String,
  description: String,
  value: u32,
}


pub struct Player {
  name: String,
  current_health: u32,
  max_health: u32,
  strength: u32,
  gold: u32,
  inventory: Vec<Item>,
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

  pub fn get_gold(&self) -> u32 {
      self.gold
  }

  pub fn get_inventory(&self) -> &Vec<Item> {
    &self.inventory
  }
}