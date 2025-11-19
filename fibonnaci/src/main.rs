use std::io;

fn main() {
    println!("Fibonacci Sequence");

    let n = get_input_number();
    let fibonacci = get_nth_fibonacci_number(n);
    println!("The {n}th Fibonacci number is {fibonacci}");
}


fn get_input_number() -> u32 {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() == "" {
        return get_input_number();
    }

    if input.trim().starts_with('-') {
        println!("Please enter a positive number.");
        return get_input_number();
    }

    let input_number: u32 = input.trim().parse().expect("Please type a number!");

    if input_number > 50 {
        println!("Please enter a number less than 50. Lets not get carried away here.");
        return get_input_number();
    }

    input_number
}

fn get_nth_fibonacci_number(n: u32) -> u32 {
    if n == 0 {
       return 0;
    } 
    
    if n == 1 {
        return 1;
    }

    get_nth_fibonacci_number(n - 1) + get_nth_fibonacci_number(n - 2)
}