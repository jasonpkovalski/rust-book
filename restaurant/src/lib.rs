mod front_of_house;

pub use crate::front_of_house::hosting;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // All of a public enum's variants are public
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Bring Breakfast struct from back_of_house module into scope
use crate::back_of_house::Breakfast;

// use std::fmt::Result;
// use std::io::Result as IoResult;

// pub use crate::front_of_house::hosting;

// use std::{cmp::Ordering, io};
// use std::io::{self, Write};
// use std::collections::*;

pub fn eat_at_restaurant() {

    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    println!(
        "Order1 includes appetizer: {:?}.  Order2 includes appetizer: {:?}.",
        order1, order2
    )
}
