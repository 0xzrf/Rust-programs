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

        let input = input.trim();
        let (cmd, args) = input.split_once(" ").unwrap_or((input, ""));
        let system_cmd = SystemExecutables::build(cmd.trim(), args.trim());

        match cmd {
            "exit" => SystemConfig::exit(args),
            "echo" => system_cmd.echo()?,
            "type" => system_cmd.handle_type()?,
            "pwd" => system_cmd.handle_pwd()?,
            "cd" => system_cmd.handle_cd()?,
            "cat" => system_cmd.handle_cat()?,
            _ =>  if let Err(_) = SystemConfig::execute_cmd(&input[..].trim()) {println!("{cmd}: command not found")},
        }
    }
}
