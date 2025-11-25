use std::{thread, time::Duration};
use std::io::Write;

fn main() {
    println!("Loading...");

    for i in 0..100 {
        print_loading_bar(i);
        thread::sleep(Duration::from_millis(30));
    }
}


fn print_loading_bar(percentage: u32) {
    assert!(percentage <= 100);
    let bar_length = 50;
    let filled_length = (percentage as f64 / 100.0 * bar_length as f64) as usize;
    let empty_length = bar_length - filled_length;

    let mut bar = String::new();    
    bar.push_str("[");
    for _ in 0..filled_length {
        bar.push_str("=");
    }
    for _ in 0..empty_length {
        bar.push_str(" ");
    }
    bar.push_str("]");

    // note here: not using println! because it will print a new line and overwrite the loading bar.
    // instead we use print! and flush the stdout to ensure the loading bar is printed correctly.
    print!("\r{bar} {percentage}%");
    std::io::stdout().flush().unwrap();
}