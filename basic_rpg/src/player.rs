use crate::item::Item;

pub struct InventoryItem {
    item: Item,
    quantity: u32,
}

impl InventoryItem {
    pub fn new(item: Item, quantity: u32) -> Self {
        Self { item, quantity }
    }

    pub fn get_item(&self) -> &Item {
        &self.item
    }

    pub fn get_quantity(&self) -> u32 {
        self.quantity
    }
}

pub struct Player {
    name: String,
    current_health: u32,
    max_health: u32,
    strength: u32,
    boosted_strength: u32,
    boosted_strength_turns_remaining: u32,
    gold: u32,
    inventory: Vec<InventoryItem>,
    level: u32,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self {
            name,
            current_health: 100,
            max_health: 100,
            strength: 10,
            boosted_strength: 0,
            boosted_strength_turns_remaining: 0,
            gold: 100,
            inventory: Vec::new(),
            level: 1,
        }
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

    pub fn get_boosted_strength(&self) -> u32 {
        self.boosted_strength
    }

    pub fn get_boosted_strength_turns_remaining(&self) -> u32 {
        self.boosted_strength_turns_remaining
    }

    pub fn set_boosted_strength(&mut self, boosted_strength: u32) {
        self.boosted_strength = boosted_strength;
        self.boosted_strength_turns_remaining = 3;
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

    pub fn can_afford(&self, cost: u32) -> bool {
        self.gold >= cost
    }

    pub fn reduce_gold(&mut self, cost: u32) {
        self.gold -= cost;
    }

    fn add_inventory_item(&mut self, item: InventoryItem) {
        self.inventory.push(item);
    }

    fn get_inventory_item(&mut self, item: &Item) -> Option<&mut InventoryItem> {
        self.inventory
            .iter_mut()
            .find(|inventory_item| inventory_item.item.get_name() == item.get_name())
    }

    pub fn add_item(&mut self, item: Item) {
        let inventory_item = self.get_inventory_item(&item);
        if let Some(inventory_item) = inventory_item {
            inventory_item.quantity += 1;
        } else {
            self.add_inventory_item(InventoryItem { item, quantity: 1 });
        }
    }

    pub fn get_inventory_items(&self) -> &Vec<InventoryItem> {
        &self.inventory
    }

    pub fn remove_item(&mut self, item: &Item) {
        let inventory_item = self.get_inventory_item(item);
        if let Some(inventory_item) = inventory_item {
            if inventory_item.quantity == 1 {
                self.inventory.remove(self.inventory.iter().position(|i| i.item.get_name() == item.get_name()).unwrap());
                return;
            }
            inventory_item.quantity -= 1;
        }
    }

    pub fn get_gold(&self) -> u32 {
        self.gold
    }

    pub fn heal(&mut self, amount: u32) {
        self.current_health = if amount > self.max_health - self.current_health {
            self.max_health
        } else {
            self.current_health + amount
        };
    }
} 
