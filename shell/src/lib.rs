mod commands;
pub use commands::*;
use std::io::{self, Write};

pub fn run() -> Result<(), &'static str> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        // This is a variable that will be shared between the commands that manipulate the path value
        let mut current_path = std::env::current_dir()
                                .unwrap() // since the current_dir is always defined
                                .display()
                                .to_string();

        match *&input[..].trim() {
            "exit 0" => SystemConfig::exit(0),
            input if input.starts_with("echo") => SystemExecutables::echo(input)?,
            input if input.starts_with("type") => SystemExecutables::handle_type(input)?,
            input if input.starts_with("pwd") => SystemExecutables::handle_pwd(&mut current_path)?,
            input if input.starts_with("cd") => SystemExecutables::handle_cd(input, &mut current_path)?,
            _ =>  {
                match SystemConfig::execute_cmd(&input[..].trim()) {
                    Ok(_) => continue,
                    Err(_) => println!("{}: command not found", &input[..].trim().split(" ").next().unwrap())
                }
            },
        }
    }
}
