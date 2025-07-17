mod arrays;
mod strings;

use std::{env};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <task>");
//        println!("Available tasks: christmas, temperature, fibonacci");
        return;
    }

    match args[1].as_str() {
        "727" => arrays::remove_duplicates_from_sorted_array::solution_remove_duplicates(),
        "564" => arrays::best_time_to_buy_and_sell_stock_2::best_profit(),
        "646" => arrays::rotate_array::solution_rotate(),
        "578" => arrays::contains_duplicate::solution_contains_duplicates(),
        "549" => arrays::single_number::solution_single_numbers(),
        "674" => arrays::intersection_of_two_arrays_2::solution_intersection(),
        "559" => arrays::plus_one::solution_plus_one(),
        "567" => arrays::move_zeroes::solution_move_zeroes(),
        "546" => arrays::two_sum::solution_two_sums(),
        "769" => arrays::valid_sudoku::valid_sudoku(),
        "770" => arrays::rotate_image::Solution::rotate_image_solution(),
        "879" => strings::reverse_string::Solution::reverse_string_solution(),
        _ => println!("Unknown task: {}", args[1]),

    }
}

