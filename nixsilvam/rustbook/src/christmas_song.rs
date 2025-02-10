pub fn christmas() {
    let song_strings = ["And a partridge in a pear tree.", "Two turtle doves,", "Three French hens,", "Four calling birds,", "Five golden rings,", 
                        "Six geese a-laying,", "Seven swans a-swimming,", "Eight maids a-milking,", "Nine ladies dancing,", "Ten lords a-leaping,",
                        "Eleven pipers piping,", "Twelve drummers drumming,"];

    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];

    let mut index = 0;

    let mut next_string = String::new();

    for day in days {
        let first_strings = format!("On the {day} day of Christmas,\nmy true love gave to me");

        next_string.insert_str(0, song_strings[index]);
        next_string.insert_str(0, "\n");
        index += 1;

        println!("{first_strings}{next_string}\n");

    }
}

