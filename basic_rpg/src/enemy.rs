use crate::item::Item;
use crate::player::Player;

#[derive(Debug)]
pub struct Enemy {
    name: String,
    current_health: u32,
    max_health: u32,
    strength: u32,
    gold: u32,
    // inventory: Vec<Item>,
    level: u32,
}

impl Enemy {
    pub fn new(player: &Player) -> Self {
        // enemy level (player level +/- 1)
        let player_level: u32 = player.get_level();
        let min_enemy_level: u32 = if player_level > 1 {
            player_level - 1
        } else {
            1
        };
        let enemy_level: u32 = rand::random_range(min_enemy_level..=player_level + 2 as u32);

        // enemy health
        let player_strength: f32 = player.get_strength() as f32;
        let bonus: f32 = 2.5 + rand::random_range(0..=10) as f32;
        let enemy_health: u32 = (player_strength + bonus).floor() as u32;

        // enemey strength
        let player_strength: f32 = player.get_strength() as f32;
        let bonus: f32 = 2.5 + rand::random_range(0..=10) as f32;
        let enemy_strength: u32 = (player_strength + bonus).floor() as u32;

        // rewards
        let enemy_gold = enemy_level * 10 + rand::random_range(0..=100);
        // let enemy_inventory = Vec::new();

        Self {
            name: Self::generate_enemy_name(enemy_level),
            current_health: enemy_health as u32,
            max_health: enemy_health as u32,
            strength: enemy_strength,
            gold: enemy_gold,
            // inventory: enemy_inventory,
            level: enemy_level,
        }
    }

    fn generate_enemy_name(level: u32) -> String {
        let mut possible_names = vec!["Spider", "Rat"];

        let possible_adjectives = vec![
            "Red", "Black", "Green", "Blue", "Yellow", "Purple", "Large", "Small", "Fast", "Slow",
            "Strong", "Weak", "Smart", "Dumb",
        ];

        if level > 3 {
            possible_names.push("Wolf");
            possible_names.push("Bear");
        }

        if level > 6 {
            possible_names.push("Goblin");
            possible_names.push("Orc");
            possible_names.push("Troll");
        }

        if level > 9 {
            possible_names.push("Dragon");
            possible_names.push("Vampire");
        }

        let name = possible_names[rand::random_range(0..=possible_names.len() - 1)];
        let adjective = possible_adjectives[rand::random_range(0..=possible_adjectives.len() - 1)];
        let name = format!("{} {}", adjective, name);
        name.to_string()
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_current_health(&self) -> u32 {
        self.current_health
    }

    pub fn get_gold(&self) -> u32 {
        self.gold
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn get_strength(&self) -> u32 {
        self.strength
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
