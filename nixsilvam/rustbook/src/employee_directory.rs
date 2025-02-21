use std::io;
use std::collections::HashMap;


pub fn employee_directory() {
let input = input_data();
println!("{:?}", input);
}

fn input_data() -> Vec<(String, String)>{
    let mut data_list = Vec::new();

    loop {
        println!("Please input employer and directory in format \"add <Name> to <directory>\"");

        let mut user_line = String::new();

        io::stdin()
            .read_line(&mut user_line)
            .expect("Failed to read line");
        
        if user_line.trim().is_empty() {
            break;
        }
        let parsed_data = parse_input(user_line.trim());
        match parsed_data {
            Some(data) => data_list.push(data),
            None => println!("Uncorrected format line. Please use format: add <Name> to <directory>")
            
        }
    }
    data_list
} 

fn parse_input(input: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() == 4 && parts[0].to_lowercase() == "add" && parts[2].to_lowercase() == "to" {
        Some((parts[1].to_string(), parts[3].to_string()))
    } else {
        None
    }
}


fn data_map(data: &mut Vec<(&str, &str)>) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for (name, directory) in data {
        map.insert(directory.to_string(), name.to_string());
    }
    map
}


fn department (input: &str, map: HashMap<&str, &str>) {

}


fn company (input: &str, map: HashMap<&str, &str>) {

}


    
