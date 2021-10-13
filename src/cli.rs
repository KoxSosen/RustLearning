// Just some fancy cli stuff
// Using the default lib of Rust

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "yes";
    let status = "100%";

    println!("Args: {:?}", command);

    if command == "hello" {
        println!("{}", name);
    } else if command == "status" {
        println!("Status: {}", status);
    } else {
        println!("Invalid");
    }

}