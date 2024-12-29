use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, io};

use crate::DEPENDENCY_FILE;

#[derive(Debug)]
struct Config {
    name: String,
    version: String,
    description: String,
    main: String,
    dependencies: Vec<String>,
    author: String,
    license: String,
}

impl Config {
    fn new() -> Self {
        let current_directory = env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
        Config {
            name: current_directory.to_string_lossy().to_string(),
            version: "1.0.0".to_string(),
            description: String::new(),
            main: "main.lua".to_string(),
            dependencies: Vec::new(),
            author: String::new(),
            license: "ISC".to_string(),
        }
    }

    fn to_string(&self) -> String {
        let mut result = String::from("{\n");
        result.push_str(&format!("\tname = \"{}\",\n", self.name));
        result.push_str(&format!("\tversion = \"{}\",\n", self.version));
        result.push_str(&format!("\tdescription = \"{}\",\n", self.description));
        result.push_str(&format!("\tmain = \"{}\",\n", self.main));
        result.push_str("\tdependencies = {},\n");
        result.push_str(&format!("\tauthor = \"{}\",\n", self.author));
        result.push_str(&format!("\tlicense = \"{}\",\n", self.license));
        result.push_str("}\n");
        result
    }

    fn write(&self) -> io::Result<()> {
        let mut file = File::create(DEPENDENCY_FILE)?;
        file.write_all(format!("return {}\n", self.to_string()).as_bytes())?;
        Ok(())
    }
}

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
