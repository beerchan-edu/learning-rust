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
        "single_numbers" => arrays::single_numbers::solution_single_numbers(),
        "intersection" => arrays::intersection_arrays::solution_intersection(),
        "plus_one" => arrays::plus_one::solution_plus_one(),
        "move_zeroes" => arrays::move_zeroes::solution_move_zeroes(),
        "two_sums" => arrays::two_sums::solution_two_sums(),
        _ => println!("Unknown task: {}", args[1]),

    }
}

