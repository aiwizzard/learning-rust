use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let name: String = "Ajmal".to_string();
    let status = "100%".to_string();

    println!("Args: {:?}", args);

    let command = args[1].clone();
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command");
    }
}
