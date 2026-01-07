mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Using a use statement
    use crate::front_of_house::hosting;
    hosting::add_to_waitlist();

    // Using a use statement with a glob
    use crate::front_of_house::hosting::*;
    add_to_waitlist();
}

pub fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // super keyword is used to access a parent module
        super::serve_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // marking an enum as public makes all
    // of its variants public as well
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_back_of_house() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!("I'd like {} and {} please", order1, order2);
}
