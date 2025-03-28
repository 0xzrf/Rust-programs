mod commands;
pub use commands::*;
use std::io::{self, Write};

pub fn run() -> Result<(), &'static str> {
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
            input if input.starts_with("echo") => SystemExecutables::echo(input)?,
            input if input.starts_with("type") => SystemExecutables::handle_type(input)?,
            input if input.starts_with("pwd") => SystemExecutables::handle_pwd()?,
            _ =>  {
                match SystemConfig::invalid_command(&input[..].trim()) {
                    Ok(_) => continue,
                    Err(_) => println!("{}: command not found", &input[..].trim().split(" ").next().unwrap())
                }
            },
        }
    }
}
