mod commands;
pub use commands::*;
mod helper_functions;
pub use helper_functions::*;

use std::io::{self, Write};

pub fn run() -> Result<(), &'static str> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let (cmd, args) = input.split_once(" ").unwrap();

        let system_cmd = SystemExecutables {
            cmd: cmd.to_string(),
            args: args.to_string()
        };

        match cmd {
            "exit 0" => SystemConfig::exit(0),
            "echo" => system_cmd.echo()?,
            // input if input.starts_with("type") => SystemExecutables::handle_type(input)?,
            // input if input.starts_with("pwd") => SystemExecutables::handle_pwd()?,
            // input if input.starts_with("cd") => SystemExecutables::handle_cd(input)?,
            // input if input.starts_with("cat") => SystemExecutables::handle_cat(input)?,
            _ =>  {
                match SystemConfig::execute_cmd(&input[..].trim()) {
                    Ok(_) => continue,
                    Err(_) => println!("{}: command not found", &input[..].trim().split(" ").next().unwrap())
                }
            },
        }
    }
}
