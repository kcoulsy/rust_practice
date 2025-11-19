fn main() {
    println!("Twelve Days of Christmas");

    for day in 1..13 {
        println!("On the {day} day of Christmas my true love sent to me \n");

        
        for idx in (0..day).rev() {
            println!("{}", get_gifts_for_day(idx + 1));
        }

        println!("\n\n");

    }
}

fn get_gifts_for_day(day: u32) -> String {
    let gifts = match day {
        1 => "A Partridge in a Pear Tree",
        2 => "Two turtle doves",
        3 => "Three french hens",
        4 => "Four calling birds",
        5 => "Five gold rings",
        6 => "Six geese a laying",
        7 => "Seven swans a swimming",
        8 => "Eight maids a milking",
        9 => "Nine ladies dancing",
        10 => "Ten lords a leaping",
        11 => "Eleven pipers piping",
        12 => "Twelve drummers drumming",
        _ => "A Partridge in a Pear Tree", //  catch all for non-exhaustive patterns: `0_u32` and `13_u32..=u32::MAX` not covered
    };

    gifts.to_string()
}