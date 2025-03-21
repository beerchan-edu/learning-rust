mod array_remove_duplicates;
mod best_profit;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <task>");
//        println!("Available tasks: christmas, temperature, fibonacci");
        return;
    }

    match args[1].as_str() {
        "remove_duplicates" => array_remove_duplicates::solution_remove_duplicates(),
        "best_profit" => best_profit::best_profit(),
        _ => println!("Unknown task: {}", args[1]),

    }
}
