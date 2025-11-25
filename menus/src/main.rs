use std::io;

const CLEAR_SCREEN: &str = "\x1B[2J\x1B[H";

fn main() {
    let mut shop = Shop::new();
    shop.print_menu();
}

fn get_string_input(prompt: &str) -> String {
    println!("\n{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() == "" {
        return get_string_input(prompt);
    }

    input.trim().to_string()
}

fn get_u32_input(prompt: &str) -> u32 {
    let input = get_string_input(prompt);
    let input: u32 = input.trim().parse().expect("Please type a number!");
    if input == 0 {
        return get_u32_input(prompt);
    }

    input
}

enum MenuState {
    MainMenu,
    SellItems,
    BuyItems,
}

#[derive(Clone)]
struct ShopItem {
    item_name: String,
    stock_count: u32,
}

struct Shop {
    items_for_sale: Vec<ShopItem>,
    current_menu_state: MenuState,
}

impl Shop {
    fn new() -> Self {
        Self {
            items_for_sale: vec![ShopItem {
                item_name: "Apple".to_string(),
                stock_count: 10,
            }],
            current_menu_state: MenuState::MainMenu,
        }
    }

    fn go_to_sell_items(&mut self) {
        self.current_menu_state = MenuState::SellItems;
        self.print_menu();
    }

    fn go_to_buy_items(&mut self) {
        self.current_menu_state = MenuState::BuyItems;
        self.print_menu();
    }

    fn go_to_main_menu(&mut self) {
        self.current_menu_state = MenuState::MainMenu;
        self.print_menu();
    }

    fn go_to_exit(&mut self) {
        std::process::exit(0);
    }

    fn sell_item_to_shop(&mut self) {
        println!("What item do you want to sell?");
        let item_name = get_string_input("Item name:");
        let stock_count = get_u32_input("Stock count:");
        if self
            .items_for_sale
            .iter()
            .any(|item| item.item_name == item_name)
        {
            let item_for_sale = self
                .items_for_sale
                .iter_mut()
                .find(|item_inner| item_inner.item_name == item_name)
                .unwrap();
            item_for_sale.stock_count += stock_count;
        } else {
            self.items_for_sale.push(ShopItem {
                item_name: item_name.clone(),
                stock_count,
            });
        }
        println!("You sold {stock_count} {item_name}");
    }

    fn buy_item_from_shop(&mut self) {
        println!("What item do you want to buy?");
        let available_items = self
            .items_for_sale
            .iter()
            .map(|item| item.item_name.clone())
            .collect::<Vec<String>>();

        if available_items.is_empty() {
            println!("No items available for sale!");
            self.go_to_main_menu();
            return;
        }

        for (index, item_name) in available_items.iter().enumerate() {
            let item_for_sale = self.items_for_sale.iter().find(|item_inner| item_inner.item_name == *item_name).unwrap();
            println!(
                "{} - {} ({} in stock)",
                index + 1,
                item_for_sale.item_name,
                item_for_sale.stock_count
            );
        }
        let item_index = (get_u32_input("Item index:") - 1) as usize;
        if item_index >= self.items_for_sale.len() {
            println!("Invalid item index!");
            self.buy_item_from_shop();
            return;
        }
        let item = &mut self.items_for_sale[item_index];
        let item_clone = item.clone();
        let stock_count = get_u32_input("Stock count:");


        item.stock_count -= stock_count;
        if item.stock_count == 0 {

            self.items_for_sale.remove(item_index as usize);
        }

        println!("You bought {stock_count} {}", item_clone.item_name);
    }

    fn print_items_for_sale(&self) {
        for item in &self.items_for_sale {
            println!("{} - {}", item.item_name, item.stock_count);
        }
    }

    fn print_menu(&mut self) {
        match self.current_menu_state {
            MenuState::MainMenu => {
                println!("Main Menu");
                println!("1. Sell items");
                println!("2. Buy items");
                println!("3. Exit");
                let choice = get_u32_input("Choice:");
                match choice {
                    1 => self.go_to_sell_items(),
                    2 => self.go_to_buy_items(),
                    3 => self.go_to_exit(),
                    _ => println!("Invalid choice!"),
                }
            }
            MenuState::SellItems => {
                println!("Sell Items");
                self.sell_item_to_shop();
                self.go_to_main_menu();
            }
            MenuState::BuyItems => {
                println!("Buy Items");
                self.buy_item_from_shop();
                self.go_to_main_menu();
            }
        }
    }
}
