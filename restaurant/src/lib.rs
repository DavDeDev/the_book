// Module
mod front_of_house {
    // Here we can define modules, functions, structs, enums, and more

    // Module
    pub mod hosting {
        // Function
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    // Module
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

//While front_of_house isn’t public, because the eat_at_restaurant function is defined in the same module as front_of_house, we can use front_of_house::hosting::add_to_waitlist to call the add_to_waitlist function.
// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// This code generates the following Crate structure:
// root crate (lib.rs or main.rs)
//  └── front_of_house
//      ├── hosting
//      │   ├── add_to_waitlist
//      │   └── seat_at_table
//      └── serving
//          ├── take_order
//          ├── serve_order
//          └── take_payment

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // Super corresponds to .. in filesystem
        super::deliver_order();
    }

    fn cook_order() {}

    // we make the struct public, but the struct’s fields will still be private
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // .. by contrast we make the enum public, so ALL its variants are PUBLIC
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}