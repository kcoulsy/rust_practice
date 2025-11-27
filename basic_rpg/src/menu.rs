use crate::enemy::Enemy;
use crate::input::get_user_input;
use crate::player::Player;

pub struct MenuItem {
    label: String,
    action: fn(&mut Player, Option<&mut Enemy>),
}

impl MenuItem {
    pub fn new(label: String, action: fn(&mut Player, Option<&mut Enemy>)) -> Self {
        Self { label, action }
    }
}

pub struct Menu {
    title: String,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(title: String, items: Vec<MenuItem>) -> Self {
        Self { title, items }
    }

    pub fn run(&self, player: &mut Player, enemy: Option<&mut Enemy>) {
        let items = &self.items;
        println!("\n{}\n", self.title);
        for (index, item) in items.iter().enumerate() {
            println!("{}. {}", index + 1, item.label);
        }
        let choice = get_user_input("Enter your choice:");
        let choice: u32 = choice.trim().parse().expect("\nPlease type a number!\n");
        if !self.validate_choice(choice) {
            println!("\nPlease enter a valid choice!\n");
            return self.run(player, enemy);
        }
        self.on_choice(choice, player, enemy);
    }

    fn validate_choice(&self, choice: u32) -> bool {
        choice >= 1 && choice <= self.items.len() as u32
    }

    fn on_choice(&self, choice: u32, player: &mut Player, enemy: Option<&mut Enemy>) {
        let choice = &self.items[choice as usize - 1];
        (choice.action)(player, enemy)
    }

    pub fn push_menu_item(&mut self, menuItem: MenuItem) {
        self.items.push(menuItem);
    }
}
