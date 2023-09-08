// Here we're using front_of_house from a module defined here, and not the one in the file 
// front_of_house.rs; for info about how to use that file see notes below 

mod front_of_house {    // -> Note: front_of_house can be called by eat_at_restaurant below, despite being private, as they are siblings
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_a_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Using super::
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order(); 
        super::deliver_order(); // -> Parent items are always public to inner modules 
    }

    fn cook_order() {}

    // With structs -> We need to declare all the fields we want public 
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

    // With enums -> once the enum is pub, all of its variants are public 
    pub enum Appetizer {
        Soup, 
        Salad, 
    }
}

fn deliver_order() {}

// Paths for referring to an Item in the Module Tree 
pub fn eat_at_restaurant() {
    // Absoulute path 
    crate::front_of_house::hosting::add_to_waitlist(); 

    // Relative path 
    front_of_house::hosting::add_to_waitlist(); 
}

// Example for structs 
pub fn eat_at_restaurant_2() {
    // Summer Rye toast order
    let mut meal = back_of_house::Breakfast::summer("Rye"); 
    // Change type of bread: 
    meal.toast = String::from("Wheat"); 
    println!("I'd like {} toast please", meal.toast);   

    // Next line won't compiled if uncommented due to privacy rules
    // meal.seasonal_fruit = String::from("blueberries"); 
}

// Example for enums
pub fn eat_at_restaurant_3() {
    let order1 = back_of_house::Appetizer::Soup; 
    let order2 = back_of_house::Appetizer::Salad; 
}

// Using "use"
use crate::front_of_house::hosting; 
// We could also export using use .. as : use crate::front_of_house::hosting as front_hosting , for example 
// We could also use: pub use crate::front_of_house::hosting with re-exporting 
pub fn eat_at_restaurant_4() {
    hosting::add_to_waitlist; 
}

// Moving eat_at_restaruant to a child mod 
mod customer {
    pub fn eat_at_restaruant() {
        // hosting::add_to_waitlist() won't compile unless we add inside the module: 
        // use crate::front_of_house::hosting; 
        // or in the main scope we declare: pub use crate::front_of_house::hosting; 
        // hosting::add_to_waitlist(); // -> This won't compile 
    }
}

// Nested paths
// use std::{cmp::Ordering, io}; 

// use std::io; 
// use std::io::Write; 
// -> become use std::io::{self, Write}; 

// Glob operator -> brings all public items defined in a path into scope 
// use std::collections::*; 

// Working from different files 
// Assume that front_of_house module is in its own file; this is how lib.rs will look like (same for binary crates): 
// mod front_of_house; 
// 
// pub use crate::front_of_house::hosting; 
// 
// pub fn eat_at_restaurant() {
//     hosting::add_to_waitlist(); 
// }