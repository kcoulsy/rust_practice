mod player;

use std::io;

use enemy::Enemy;
use player::Player;

fn main() {
    println!("Welcome to the RPG!");
    let name = get_user_input("Enter your name:");
    let mut player = Player::new(name);

    loop {
        println!("What do you want to do?");
        println!("1. Wander");
        println!("2. Shop");
        println!("3. Stats");
        println!("4. Exit");
        let choice = get_user_input("Enter your choice:");
        let choice: u32 = choice.trim().parse().expect("\nPlease type a number!\n");

        match choice {
            1 => println!("\nYou are wandering around.\n"),
            2 => println!("\nYou are at the shop.\n"),
            3 => println!("\nYou are at the stats.\n"), 
            4 => std::process::exit(0),
            _ => println!("\nYou do nothing.\n"),
        }
    }
}


fn get_user_input(prompt: &str) -> String {
    println!("\n{prompt}");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() == "" {
        return get_user_input(prompt);
    }

    input.trim().to_string()
}


fn run_battle(player: &mut Player) {
    let player_level = player.get_level();
    let player_strength = player.get_strength();
    let player_gold = player.get_gold();

    let player_inventory = player.get_inventory();

    // TODO: Implement enemy generation
    let enemy = Enemy::new(player);

    while player.get_current_health() > 0 && enemy.get_current_health()  > 0 {   
        println!("You are fighting an enemy!");
        println!("You have {} health and {} strength.", player.get_current_health(), player.get_strength());
        println!("The enemy has {} health and {} strength.", enemy.get_current_health(), enemy.get_strength());
        println!("What do you want to do?");
        println!("1. Attack");
        println!("2. Run");
        let choice = get_user_input("Enter your choice:");
        let choice: u32 = choice.trim().parse().expect("\nPlease type a number!\n");
    }
}