// Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.

use std::io;
use std::collections::HashMap;

/// Entry point for the employee directory program.
/// It collects employee data, builds a company directory, and then allows
/// the user to query the directory.
pub fn employee_directory() {
    
    let company = data_map(input_data());

    show_department(company);

}

/// Prompts the user to input employee names and departments.
/// The expected format is: "add <Name> to <Department>".
/// The function continues to prompt until the user enters an empty line (after
/// at least one record is provided) or "exit" (which terminates the program immediately).
/// 
/// Returns a vector of tuples where each tuple contains (Name, Department).
fn input_data() -> Vec<(String, String)> {
    let mut data_list = Vec::new();
    println!("Please input employer name and department in format \"add <Name> to <Department>\" \nPush \"Enter\" in new line to stop adding.");
    println!("Input \"exit\" to terminate the programm immediately");

    loop {
        let mut user_line = String::new();

        io::stdin()
            .read_line(&mut user_line)
            .expect("Failed to read line");

        if user_line.trim() == "exit" {
            std::process::exit(0);
        }
        
        if user_line.trim().is_empty() {
            if data_list.is_empty() {
                println!("No data provided yet. Please enter at least one record.");
                continue;
            } else {
                println!("Employees and Departments data are saved");
                break;
            }
        }
        let parsed_data = parse_input(user_line.trim());
        match parsed_data {
            Some(data) => data_list.push(data),
            None => println!("Uncorrected format line. Please use format: add <Name> to <Department>")
            
        }
    }
    data_list.sort_by(|a, b| (a.1.as_str(), a.0.as_str()).cmp(&(b.1.as_str(), b.0.as_str())));
    data_list
} 

/// Parses a single input line and returns a tuple (Name, Department) if the input
/// matches the expected format "add <Name> to <Department>".
/// 
/// Returns Some((Name, Department)) on success, or None if the format is incorrect.
fn parse_input(input: &str) -> Option<(String, String)> {

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() == 4 && parts[0].to_lowercase() == "add" && parts[2].to_lowercase() == "to" {
        Some((parts[1].to_string(), parts[3].to_string()))
    } else {
        None
    }
}

/// Constructs a company directory from the provided vector of (Name, Department) tuples.
/// Returns a HashMap where each key is a department and the corresponding value is a vector
/// of employee names in that department.
fn data_map(data: Vec<(String, String)>) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for (name, department) in data {
        map.entry(department)
            .or_insert_with(Vec::new)
            .push(name);
    }
    map
}

/// Provides a text-based interface for the user to retrieve employee data.
/// 
/// The user can:
/// - Input "all" to display all employees grouped by department (sorted alphabetically).
/// - Input a specific department name (e.g., "Engineering") to display all employees in that department.
/// - Input "exit" to quit the program.
fn show_department (map: HashMap<String, Vec<String>>) {
    println!("To show all employees in the company by department input \"all\"");
    println!("To show a list of all employees in a department input the Department name in format: <Department>");
    println!("Input \"exit\" to terminate the program immediately");

    loop {
        let mut user_command = String::new();
        io::stdin()
                .read_line(&mut user_command)
                .expect("Failed to read line");

        let user_command: &str = user_command.trim();

        if user_command == "exit" {
            break;
        } else if user_command == "all" {
            let mut departments: Vec<&String> = map.keys().collect();
            departments.sort();

            for department in departments {
                let mut employees = map.get(department).unwrap().clone();
                employees.sort();
                println!("{}: {}", department, employees.join(","));
            }
        } else if let Some(employees) = map.get(user_command) {

            for employee in employees {
                println!("{}", employee)
            }
        } else {
            println!("Not found department with name \"{}\" or any related command (\"all\", \"exit\"). Try again.", user_command)
        }
    }
}
