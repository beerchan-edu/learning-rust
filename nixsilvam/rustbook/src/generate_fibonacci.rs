use std::io;

pub fn generate_fibonacci() {
    println!("Enter a number to calculate its Fibonacci value:");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Please type a number!");

    let fibonacci_result = fibonacci_number(n);

    println!("The {n}th Fibonacci number is {fibonacci_result}");
}

fn fibonacci_number(n: u32) -> u32 {
    if n == 0 {
        return 0
    }
    if n == 1 {
        return 1
    }

    let mut current = 1;
    let mut previous = 0;

    for _element in 2..n + 1 {
        let result = current + previous;
        previous = current;
        current = result;
    }

    return current
}
