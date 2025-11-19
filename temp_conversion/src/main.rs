use std::io;

const FAHRENHEIT_TO_CELSIUS: u32 = 1;
const CELSIUS_TO_FAHRENHEIT: u32 = 2;

fn main() {
    println!("Temperature Converter");

    let choice = get_direction_input();
    let temperature = get_temperature_input(choice);

    if choice == FAHRENHEIT_TO_CELSIUS {
        let celsius = f_to_c(temperature);
        println!("{temperature}°F = {celsius}°C");
    } else {
        let fahrenheit = c_to_f(temperature);
        println!("{temperature}°C = {fahrenheit}°F");
    }
}

fn f_to_c(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn c_to_f(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn get_direction_input() -> u32 {
    println!("Select the conversion direction:");
    println!("{} Fahrenheit to Celsius", FAHRENHEIT_TO_CELSIUS);
    println!("{} Celsius to Fahrenheit", CELSIUS_TO_FAHRENHEIT);

    let mut choice = String::new();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    let choice: u32 = choice.trim().parse().expect("Please type a number!");

    if choice != FAHRENHEIT_TO_CELSIUS && choice != CELSIUS_TO_FAHRENHEIT {
        println!("Invalid choice. Please enter 1 or 2.");
        return get_direction_input();
    }

    choice
}

fn get_temperature_input(choice: u32) -> f64 {
    if choice == FAHRENHEIT_TO_CELSIUS {
        println!("Enter the temperature in Fahrenheit:");
    } else {
        println!("Enter the temperature in Celsius:");
    }

    let mut temperature = String::new();

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = temperature.trim().parse().expect("Please type a number!");

    temperature
}
