// Module
mod front_of_house {
    // Here we can define modules, functions, structs, enums, and more

    // Module
    mod hosting {
        // Function
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    // Module
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

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
