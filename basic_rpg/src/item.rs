#[derive(Clone)]
pub struct Item {
    name: String,
    cost: u32,
    heal_amount: u32,
    strength_boost: u32,
}

impl Item {
    pub fn new(name: String, cost: u32, heal_amount: u32, strength_boost: u32) -> Self {
        Self {
            name,
            cost,
            heal_amount,
            strength_boost,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_cost(&self) -> u32 {
        self.cost
    }

    pub fn get_heal_amount(&self) -> u32 {
        self.heal_amount
    }

    pub fn get_strength_boost(&self) -> u32 {
        self.strength_boost
    }
}
