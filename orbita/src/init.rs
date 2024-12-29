use std::io;

use crate::config::Config;

pub fn init(skip: bool) {
    let mut args = std::env::args();

    args.next();

    let mut config = Config::new();

    if skip {
        if let Err(e) = config.write() {
            eprintln!("Error writing to file: {}", e);
        }
        return;
    }

    println!("What is your project name?");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    config.name = input.trim().to_string();

    println!("What is your project description?");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    config.description = input.trim().to_string();

    println!("What is your project entry point?");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    config.main = input.trim().to_string();

    println!("Who is the author?");
    input.clear();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    config.author = input.trim().to_string();

    if let Err(e) = config.write() {
        eprintln!("Error writing to file: {}", e);
    }
}
