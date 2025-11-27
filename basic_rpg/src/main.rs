mod enemy;
mod input;
mod menu;
mod player;
mod item;

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
    let menu = Menu::new(String::from("What do you want to do?"), vec![
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
        MenuItem::new(String::from("Stats"), |_player, _| {
            println!("\nYou are at the stats.\n");
        }),
        MenuItem::new(String::from("Exit"), |_player, _| {
            std::process::exit(0);
        }),
    ]);

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

    let menu = Menu::new(String::from("What do you want to do?"), vec![
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
    ]);

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
}

fn run_away(player: &mut Player) {
    println!("You run away!");
    enter_to_continue();
    return start_menu(player);
}

fn open_shop(player: &mut Player) {
    let menu = Menu::new(String::from("What do you want to do?"), vec![
        MenuItem::new(String::from("Buy items"), |player, _| {
            buy_items(player);
        }),
        MenuItem::new(String::from("Sell items"), |player, _| {
            sell_items(player);
        }),
        MenuItem::new(String::from("Exit"), |player, _| {
            return start_menu(player);
        }), 
    ]);
    menu.run(player, None);
}

fn buy_items(player: &mut Player) {
    let items_for_sale = vec![
        Item::new(String::from("Healing potion"), 10, 10, 0),
        Item::new(String::from("Strength potion"), 20, 0, 10),
        Item::new(String::from("Large healing potion"), 30, 20, 0),
    ];

    let mut menu = Menu::new(String::from("What item do you want to buy?"), vec![]);
    
    for item in items_for_sale.iter() {
        let item_name = item.get_name().clone();
        let menu_item = MenuItem::new(item_name, |player, _| {
            // buy_item(player, item);
        });
        menu.push_menu_item(menu_item);
    }

    menu.run(player, None);
}

fn sell_items(player: &mut Player) {
    println!("You sell items!");
}

fn view_inventory(player: &mut Player) {
    println!("You view your inventory!");
}
