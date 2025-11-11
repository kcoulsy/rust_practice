use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    
    // let x = 5;
    // let y = 10;
    // println!("x is {x} and y + 2 is {}", y + 2);

    println!("Guess the number!");

    // gen_range accepts a range expression, which implements the RangeBounds trait
    // The 1..=100 syntax is an inclusive range in Rust.
    // 1..=100 — inclusive range: includes both endpoints (1 through 100)
    // 1..100 — half-open range: includes 1, excludes 100 (1 through 99)
    // 1.. — open-ended range: starts at 1, no upper bound
    // ..100 — range up to (but not including) 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // let - create a variable - immutable by default
        // mut - mutable variable
        // strings have to be initialized
        let mut guess = String::new();

        io::stdin()
            // the &mut here is because the read_line function modifies the variable
            // & - reference to the variable
            .read_line(&mut guess)
            .expect("Failed to read line");

        // guess is already defined above, however shadowing lets us reuse the variable name.
        // it lets use reuse the same name for a transformed value, changing types etc.
        // parse the key here that uses the u32 type from the annotation.
        // match can be used on a Result<T, E> type too!
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // the match statement consiste of `arms` - which are the patterns to match against
        // cmp returns an Ordering enum, used to compare two values
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
