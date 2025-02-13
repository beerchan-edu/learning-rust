#![allow(dead_code)]
mod front_of_house;


// Bringing a module into scope with use
pub use crate::front_of_house::hosting; // Making a name available for any code to use from a new scope with "pub use"

mod customer {
    pub fn eat_at_restaurant() {

        //  A use statement only applies in the scope it’s in
        // To fix this problem, move the use within the customer module too, 
        // or reference the shortcut in the parent module with super::hosting within the child customer module.
        super::hosting::add_to_waitlist();
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Shortcut with "use":
    // Bringing the function’s parent module into scope with use means 
    // we have to specify the parent module when calling the function. 
    // Specifying the parent module when calling the function makes it clear
    // that the function isn’t locally defined while still minimizing repetition of the full path.
    hosting::add_to_waitlist();

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
    println!("{:?}, {:?}", order1, order2)
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Structs are often useful without their fields being public, 
    // so struct fields follow the general rule of everything being private by default unless annotated with pub.
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { 
                toast: String::from(toast), 
                seasonal_fruit: String::from("peaches"), 
            }
        }
    }

    // Enums aren’t very useful unless their variants are public; 
    // it would be annoying to have to annotate all enum variants with pub in every case, 
    // so the default for enum variants is to be public. 
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}


// On the other hand, when bringing in structs, enums, and other items
// with "use", it’s idiomatic to specify the full path.
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}


// The exception to this idiom is if we’re bringing two items 
// with the same name into scope with use statements, because Rust doesn’t allow that.

// use std::fmt;
// use std::io;


// If instead we specified use std::fmt::Result and use std::io::Result, 
// we’d have two Result types in the same scope, and Rust wouldn’t know which one we meant when we used Result.

// fn function1() -> fmt::Result {
    // --snip--
// }

// fn function2() -> io::Result<()> {
    // --snip--
// }


// Renaming a type when it’s brought into scope with the "as" keyword

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function3() -> Result {
    // --snip--
// }

// fn function4() -> IoResult<()> {
    // --snip--
// }


// Specifying a nested path to bring multiple items with the same prefix into scope
// use std::{cmp::Ordering, io};


// Combining two use statements, where one is a subpath of the other, into one use statement
// use std::io::{self, Write};


// If we want to bring all public items defined in a path into scope, 
// we can specify that path followed by the * glob operator:

//use std::collections::*;

// Be careful when using the glob operator! Glob can make it harder to tell 
// what names are in scope and where a name used in your program was defined.