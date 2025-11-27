use std::io;

pub fn get_user_input(prompt: &str) -> String {
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

pub fn enter_to_continue() {
    println!("Type anything to continue...");
    let mut input = String::new();
    let _ = io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Don't actualy care to do anything with the input, just want to wait for the user to press a key
}
