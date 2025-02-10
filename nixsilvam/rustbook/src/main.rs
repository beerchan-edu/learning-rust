mod christmas_song;
mod convert_temperatures;
mod generate_fibonacci;

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
        _ => println!("Unknown task: {}", args[1]),
    }
}

