// Convert strings to pig latin. The first consonant of each word is moved to the end of the word and ay is added, so first becomes irst-fay. Words that start with a vowel have hay added to the end instead (apple becomes apple-hay). Keep in mind the details about UTF-8 encoding!

pub fn pig_latin() {
    let txt = String::from("The first consonant of each word is moved to the end of the word and ay is added");

    println!("{}", pig_latin_converter(&txt));

}

fn pig_latin_converter(txt: &str) -> String {
    let txt = txt.to_lowercase();
    let mut pig_txt = String::new();

    for word in txt.split_whitespace() {
        let mut pig_word = String::new();

        if let Some(letter) = word.chars().next() {
            if  "aeiuo".contains(letter) {
                pig_word.push_str(&word);
                pig_word.push_str("-hay");
            } else {
                pig_word.push_str(&word[letter.len_utf8()..]);
                pig_word.push_str("-");
                pig_word.push(letter);
                pig_word.push_str("ay");
            }
        } else {
            println!("The word is empty!");
        }
        
        pig_txt.push_str(&pig_word);
        pig_txt.push_str(" ");
    }
    pig_txt.trim().to_string();
    return pig_txt;
}