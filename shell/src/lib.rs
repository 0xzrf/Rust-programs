mod commands;
pub use commands::*;
use std::io::{self, Write};

pub fn run() {
    loop {
        // Uncomment this block to pass the first stage s
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        match *&input[..].trim() {
            "exit 0" => SystemConfig::exit(0),
            input if input.starts_with("echo") => SystemExecutables::echo(input),
            _ =>  SystemConfig::invalid_command(&input[..].trim()),
        }



    }
}