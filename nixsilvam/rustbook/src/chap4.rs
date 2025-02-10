// Task: Word Counter with Borrowing

// Objective:
// Write a function that takes an immutable reference to a String and returns the number of words in it.
// Requirements:

//    The function should not take ownership of the String (use &str or &String).
//    The function should count the number of words, assuming words are separated by spaces.
//    Print the result in main().


pub fn word_counter() {
    let text = String::from("Rust is awesome!");
    let words = count_words(&text);

    println!("The number of words in the text is: {words}")

}

fn count_words(txt: &str) -> usize {
    txt.split_whitespace().count()
}