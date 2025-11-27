mod enemy;
mod input;
mod item;
mod menu;
mod player;

use enemy::Enemy;
use input::enter_to_continue;
use input::get_user_input;
use menu::Menu;
use menu::MenuItem;
use player::Player;

use crate::item::Item;

fn main() {
    start_game();
}

fn start_menu(player: &mut Player) {
    let mut menu = Menu::new(
        String::from("What do you want to do?"),
        vec![
            MenuItem::new(String::from("Wander"), |player, _| {
                println!("\nYou are wandering around.\n");
                let mut enemy = Enemy::new(player);
                run_battle(player, &mut enemy);
            }),
            MenuItem::new(String::from("Shop"), |player, _| {
                open_shop(player);
            }),
            MenuItem::new(String::from("View inventory"), |player, _| {
                view_inventory(player);
            }),
            MenuItem::new(String::from("Stats"), |player, _| {
                view_stats(player);
            }),
            MenuItem::new(String::from("Exit"), |_player, _| {
                std::process::exit(0);
            }),
        ],
    );

    menu.run(player, None);
}

fn run_battle(player: &mut Player, enemy: &mut Enemy) {
    println!(
        "You encounter a level {} {}!",
        enemy.get_level(),
        enemy.get_name()
    );
    println!(
        "You have {} health and {} strength.",
        player.get_current_health(),
        player.get_strength()
    );

    let mut menu = Menu::new(
        String::from("What do you want to do?"),
        vec![
            MenuItem::new(String::from("Attack"), |player, enemy| {
                if enemy.is_some() {
                    attack_enemy(player, enemy.unwrap());
                } else {
                    println!("No enemy to attack!");
                    return start_menu(player);
                }
            }),
            MenuItem::new(String::from("Use item"), |player, enemy| {
                if enemy.is_some() {
                    use_item(player, enemy.unwrap());
                } else {
                    println!("No enemy to use item on!");
                    return start_menu(player);
                }
            }),
            MenuItem::new(String::from("Run"), |player, _| {
                run_away(player);
            }),
        ],
    );

    menu.run(player, Some(enemy));
}

fn attack_enemy(player: &mut Player, enemy: &mut Enemy) {
    println!("You attack the enemy!");
    let player_strength = player.get_strength();
    let damage_done_to_enemy = player_strength; // TODO make this variable

    enemy.reduce_current_health(damage_done_to_enemy);
    println!("\nYou deal {} damage to the enemy!", damage_done_to_enemy);

    if enemy.is_dead() {
        println!("\nYou have defeated the enemy!");
        let award_gold = enemy.get_gold();
        player.add_gold(award_gold);
        let drop_items = enemy.get_drop_items();
        for item in drop_items {
            println!("You have found a {}!", item.get_name());
            player.add_item(item.clone());
        }
        println!("You have gained {} gold. \n", award_gold);

        return start_menu(player);
    }

    println!("The enemy has {} health left.", enemy.get_current_health());

    let enemy_strength = enemy.get_strength();
    let damage_done_to_player = enemy_strength; // TODO make this variable

    player.reduce_current_health(damage_done_to_player);
    println!("\nThe enemy deals {} damage to you!", damage_done_to_player);

    if player.is_dead() {
        return game_over();
    }

    run_battle(player, enemy);
}

fn start_game() {
    println!("Welcome to the RPG!");
    let name = get_user_input("Enter your name:");
    let mut player = Player::new(name);
    start_menu(&mut player);
}

fn game_over() {
    println!("Game over! You have died.");
    enter_to_continue();

    start_game();
}

fn use_item(player: &mut Player, enemy: &mut Enemy) {
    println!("You use an item!");
    let mut menu = Menu::new(String::from("What item do you want to use?"), vec![]);

    for inventory_item in player.get_inventory_items() {
        let item_clone = inventory_item.get_item().clone();
        let label = format!("Use 1x {}", item_clone.get_name());
        let use_item_menu_item = MenuItem::new(label, move |player, _| {
            use_item_action(player, &item_clone);
        });

        menu.push_menu_item(use_item_menu_item);
    }

    menu.push_menu_item(MenuItem::new(String::from("Go back"), |player, enemy| {
        return run_battle(player, enemy.unwrap());
    }));

    menu.run(player, Some(enemy));

    return run_battle(player, enemy);
}

fn use_item_action(player: &mut Player, item: &Item) {
    println!("You use the {}!", item.get_name());

    if item.get_heal_amount() > 0 {
        player.heal(item.get_heal_amount());
        println!("You heal for {} health!", item.get_heal_amount());
    }

    if item.get_strength_boost() > 0 {
        player.set_boosted_strength(item.get_strength_boost());
        println!("You gain {} strength for 3 turns!", item.get_strength_boost());
    }
    
    player.remove_item(item);
}

fn run_away(player: &mut Player) {
    println!("You run away!");
    enter_to_continue();
    return start_menu(player);
}

fn open_shop(player: &mut Player) {
    let mut menu = Menu::new(
        String::from("What do you want to do?"),
        vec![
            MenuItem::new(String::from("Buy items"), |player, _| {
                buy_items(player);
            }),
            MenuItem::new(String::from("Sell items"), |player, _| {
                sell_items(player);
            }),
            MenuItem::new(String::from("Exit"), |player, _| {
                return start_menu(player);
            }),
        ],
    );
    menu.run(player, None);
}

fn buy_items(player: &mut Player) {
    let items_for_sale = vec![
        Item::new(String::from("Healing potion"), 10, 10, 0),
        Item::new(String::from("Strength potion"), 20, 0, 10),
        Item::new(String::from("Large healing potion"), 30, 20, 0),
    ];

    let mut menu = Menu::new(String::from("What item do you want to buy?"), vec![]);

    for item in items_for_sale {
        let label = format!("Buy {} ({} gold)", item.get_name(), item.get_cost());
        let menu_item = MenuItem::new(label, move |player, _| {
            buy_item(player, &item);
        });
        menu.push_menu_item(menu_item);
    }

    menu.run(player, None);
}

fn buy_item(player: &mut Player, item: &Item) {
    if !player.can_afford(item.get_cost()) {
        println!("\nYou cannot afford this item!\n");
        return open_shop(player);
    }

    player.reduce_gold(item.get_cost());
    println!("\nYou buy the {}!\n", item.get_name());

    let item_clone = item.clone();
    player.add_item(item_clone);

    return open_shop(player);
}

fn sell_items(player: &mut Player) {
    println!("You sell items!");
    let mut menu = Menu::new(String::from("What item do you want to sell?"), vec![]);

    for inventory_item in player.get_inventory_items() {
        let item_clone = inventory_item.get_item().clone();
        let label = format!("Sell 1x {} for {} gold", item_clone.get_name(), item_clone.get_cost());
        let sell_single_item_menu_item = MenuItem::new(label, move |player, _| {
            sell_item(player, &item_clone);
        });
        
        menu.push_menu_item(sell_single_item_menu_item);

        if inventory_item.get_quantity() > 1 {
            let item_clone = inventory_item.get_item().clone();
            let label = format!("Sell {}x {} for {} gold", inventory_item.get_quantity(), item_clone.get_name(), item_clone.get_cost() * inventory_item.get_quantity());
            let sell_multiple_items_menu_item = MenuItem::new(label, move |player, _| {
                sell_item(player, &item_clone);
            });
            menu.push_menu_item(sell_multiple_items_menu_item);
        }

    }

    menu.push_menu_item(MenuItem::new(String::from("Exit"), |player, _| {
        return open_shop(player);
    }));
    
    menu.run(player, None);
}

fn sell_item(player: &mut Player, item: &Item) {
    player.remove_item(item);
    player.add_gold(item.get_cost());
    println!("\nYou sell the {}!\n", item.get_name());
    enter_to_continue();
    return open_shop(player);
}

fn view_inventory(player: &mut Player) {
    println!("You view your inventory!");

    if player.get_inventory_items().is_empty() {
        println!("\nYour inventory is empty!\n");
        enter_to_continue();
        return start_menu(player);
    }

    println!("Inventory:\n");
    for inventory_item in player.get_inventory_items() {
        println!("{} (x{})", inventory_item.get_item().get_name(), inventory_item.get_quantity());
    }
    println!("\n");

    enter_to_continue();

    start_menu(player);
}


fn view_stats(player: &mut Player) {
    println!("You view your stats!");
    println!("Name: {}", player.get_name());
    println!("Level: {}", player.get_level());
    println!("Health: {}", player.get_current_health());
    println!("Strength: {}", player.get_strength());
    println!("Gold: {}", player.get_gold());
    enter_to_continue();
    start_menu(player);
}