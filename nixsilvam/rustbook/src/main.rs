mod christmas_song;
mod convert_temperatures;
mod generate_fibonacci;
mod chap4;
mod rectangles_structs;
mod rectangles_methods;
mod coordinate_system;
mod rune_system;
mod median_and_mode;
mod pig_latin;


use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run <task>");
//        println!("Available tasks: christmas, temperature, fibonacci");
        return;
    }

    match args[1].as_str() {
        "christ" => christmas_song::christmas(),
        "temp" => convert_temperatures::convert_temperatures(),
        "fib" => generate_fibonacci::generate_fibonacci(),
        "chap4" => chap4::word_counter(),
        "rectstr" => rectangles_structs::rectangles(),
        "rectmthd" => rectangles_methods::rectangles(),
        "coord" => coordinate_system::coordinate_system(),
        "rune" => rune_system::mystic_rune_system(),
        "median" => median_and_mode::median_and_mode(),
        "pig" => pig_latin::pig_latin(),
        _ => println!("Unknown task: {}", args[1]),
    }
}

