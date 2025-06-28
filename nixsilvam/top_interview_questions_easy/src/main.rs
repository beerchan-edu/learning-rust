mod arrays;

use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <task>");
//        println!("Available tasks: christmas, temperature, fibonacci");
        return;
    }

    match args[1].as_str() {
        "remove_duplicates" => arrays::array_remove_duplicates::solution_remove_duplicates(),
        "best_profit" => arrays::best_profit::best_profit(),
        "rotate" => arrays::rotate_array::solution_rotate(),
        "contains_duplicates" => arrays::contains_duplicates::solution_contains_duplicates(),
        _ => println!("Unknown task: {}", args[1]),

    }
}
