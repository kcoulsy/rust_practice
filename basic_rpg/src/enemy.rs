pub struct Enemy {
  name: String,
  current_health: u32,
  max_health: u32,
  strength: u32,
  gold: u32,
  inventory: Vec<Item>,
}

impl Enemy {
  pub fn new(player: &Player) -> Self {
    // level is players +- 1  
    let enemy_level = player.get_level() + rand::thread_rng().gen_range(0..=1);
    let enemy_health = player.get_strength() * (2.5 + rand::thread_rng().gen_range(0..=10));
    let enemy_strength = player.get_strength() * (1.5 + rand::thread_rng().gen_range(0..=10));
    let enemy_gold = player.get_gold() * (1.5 + rand::thread_rng().gen_range(0..=10));
    let enemy_inventory = player.get_inventory().clone();
    Self {
      name: String::from("Enemy"),
      current_health: enemy_health,
      max_health: enemy_health,
      strength: enemy_strength,
      gold: enemy_gold,
      inventory: enemy_inventory,
    }
  }
}